use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::Arc;
use std::time::{Duration, Instant};

use anyhow::{Context as _, Result, anyhow, bail};
use extension::{
    ExtensionCommandDescriptor, ExtensionCommandProxy, ExtensionDiagnostic, ExtensionDiagnosticSeverity,
    ExtensionEditorProxy, ExtensionHostProxy, ExtensionNotification, ExtensionNotificationLevel,
    ExtensionStatusBarAlignment, ExtensionStatusBarItem, ExtensionUiProxy,
};
use parking_lot::Mutex;
use serde_json::{Value, json};

/// Embedded single-process Node.js extension host runtime.
const HOST_RUNTIME_JS: &str = include_str!("host_runtime.js");

/// Default V8 heap cap (`--max-old-space-size=256`).
pub const DEFAULT_NODE_MAX_OLD_SPACE_MB: u32 = 256;

/// Default idle shutdown duration (15 minutes) so unused hosts return RAM to the OS.
pub const DEFAULT_IDLE_SHUTDOWN: Duration = Duration::from_secs(15 * 60);

/// Default Win32 Job Object process memory limit (384 MiB: 256 MiB V8 heap + Node runtime).
pub const DEFAULT_JOB_MEMORY_LIMIT_BYTES: usize = 384 * 1024 * 1024;

/// Minimal allowlist of host environment variables passed into the isolated Node.js host.
/// Prevents leaking secrets/plugin variables and isolates the integrated terminal PTY.
const ALLOWED_ENV_VARS: &[&str] = &[
    "PATH",
    "Path",
    "SYSTEMROOT",
    "SystemRoot",
    "COMSPEC",
    "ComSpec",
    "TEMP",
    "TMP",
    "HOME",
    "USERPROFILE",
    "LANG",
    "LC_ALL",
];

#[derive(Clone, Debug)]
pub struct NodeHostConfig {
    pub node_binary: PathBuf,
    pub max_old_space_mb: u32,
    pub idle_timeout: Duration,
    pub job_memory_limit_bytes: usize,
}

impl Default for NodeHostConfig {
    fn default() -> Self {
        Self {
            node_binary: PathBuf::from("node"),
            max_old_space_mb: DEFAULT_NODE_MAX_OLD_SPACE_MB,
            idle_timeout: DEFAULT_IDLE_SHUTDOWN,
            job_memory_limit_bytes: DEFAULT_JOB_MEMORY_LIMIT_BYTES,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DynamicVsixEntry {
    pub extension_id: String,
    pub entry_path: PathBuf,
    pub activation_events: Vec<String>,
}

impl DynamicVsixEntry {
    pub fn matches_event(&self, event: &str) -> bool {
        self.activation_events.iter().any(|pattern| {
            pattern == "*" || pattern == "onStartupFinished" || pattern == event
        })
    }
}

struct RunningHost {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    next_id: u64,
    activated_extensions: HashSet<String>,
    last_activity: Instant,
    _runtime_script: tempfile::TempPath,
    #[cfg(windows)]
    _job_object: Option<win_job::JobObjectGuard>,
}

impl RunningHost {
    fn spawn(config: &NodeHostConfig) -> Result<Self> {
        let mut script_file = tempfile::Builder::new()
            .prefix("gram-vscode-host-")
            .suffix(".js")
            .tempfile()
            .context("failed to create temporary extension host script")?;
        script_file
            .write_all(HOST_RUNTIME_JS.as_bytes())
            .context("failed to write extension host runtime script")?;
        script_file.flush()?;
        let runtime_script = script_file.into_temp_path();

        let mut cmd = Command::new(&config.node_binary);
        cmd.arg(format!("--max-old-space-size={}", config.max_old_space_mb))
            .arg(&runtime_script)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());

        // Strict environment allowlist isolation
        cmd.env_clear();
        for key in ALLOWED_ENV_VARS {
            if let Ok(val) = std::env::var(key) {
                cmd.env(key, val);
            }
        }
        cmd.env("NODE_ENV", "production");
        cmd.env("GRAM_EXTENSION_HOST", "1");

        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt as _;
            const BELOW_NORMAL_PRIORITY_CLASS: u32 = 0x0000_4000;
            const CREATE_NO_WINDOW: u32 = 0x0800_0000;
            cmd.creation_flags(BELOW_NORMAL_PRIORITY_CLASS | CREATE_NO_WINDOW);
        }

        let mut child = cmd
            .spawn()
            .with_context(|| format!("failed to spawn Node.js extension host ({:?})", config.node_binary))?;

        #[cfg(windows)]
        let job_object = win_job::attach_process_to_limited_job(&child, config.job_memory_limit_bytes).ok();

        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| anyhow!("failed to capture node host stdin"))?;
        let stdout = BufReader::new(
            child
                .stdout
                .take()
                .ok_or_else(|| anyhow!("failed to capture node host stdout"))?,
        );

        Ok(Self {
            child,
            stdin,
            stdout,
            next_id: 1,
            activated_extensions: HashSet::new(),
            last_activity: Instant::now(),
            _runtime_script: runtime_script,
            #[cfg(windows)]
            _job_object: job_object,
        })
    }

    fn call(
        &mut self,
        method: &str,
        params: Value,
        proxy: Option<&ExtensionHostProxy>,
    ) -> Result<Value> {
        let id = self.next_id;
        self.next_id += 1;

        let line = serde_json::to_string(&json!({
            "id": id,
            "method": method,
            "params": params,
        }))?;
        writeln!(self.stdin, "{line}")?;
        self.stdin.flush()?;
        self.last_activity = Instant::now();

        let mut response_line = String::new();
        loop {
            response_line.clear();
            let n = self.stdout.read_line(&mut response_line)?;
            if n == 0 {
                bail!("node extension host exited unexpectedly while awaiting response to '{method}'");
            }
            let msg: Value = serde_json::from_str(response_line.trim())
                .with_context(|| format!("invalid JSON from node host: {}", response_line.trim()))?;

            match msg.get("kind").and_then(Value::as_str) {
                Some("event") => {
                    if let Some(proxy) = proxy {
                        dispatch_host_event(proxy, &msg);
                    }
                }
                Some("response") if msg.get("id").and_then(Value::as_u64) == Some(id) => {
                    self.last_activity = Instant::now();
                    if let Some(err) = msg.get("error").and_then(Value::as_str) {
                        bail!("{err}");
                    }
                    return Ok(msg.get("result").cloned().unwrap_or(Value::Null));
                }
                _ => {}
            }
        }
    }

    fn terminate(&mut self) {
        let _ = self.call("shutdown", json!({}), None);
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

impl Drop for RunningHost {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn dispatch_host_event(proxy: &ExtensionHostProxy, msg: &Value) {
    let Some(event) = msg.get("event").and_then(Value::as_str) else {
        return;
    };
    let payload = msg.get("payload").unwrap_or(&Value::Null);
    let ext_id: Arc<str> = Arc::from(
        payload
            .get("extensionId")
            .and_then(Value::as_str)
            .unwrap_or("unknown"),
    );

    match event {
        "command.register" => {
            if let Some(id) = payload.get("id").and_then(Value::as_str) {
                let title = payload
                    .get("title")
                    .and_then(Value::as_str)
                    .unwrap_or(id);
                let _ = proxy.register_command(
                    ext_id,
                    ExtensionCommandDescriptor {
                        id: Arc::from(id),
                        title: Arc::from(title),
                        description: None,
                    },
                );
            }
        }
        "ui.statusBar.set" => {
            if let Some(id) = payload.get("id").and_then(Value::as_str) {
                let alignment = match payload.get("alignment").and_then(Value::as_str) {
                    Some("right") => ExtensionStatusBarAlignment::Right,
                    _ => ExtensionStatusBarAlignment::Left,
                };
                proxy.set_status_bar_item(
                    ext_id,
                    ExtensionStatusBarItem {
                        id: Arc::from(id),
                        text: Arc::from(payload.get("text").and_then(Value::as_str).unwrap_or("")),
                        tooltip: payload.get("tooltip").and_then(Value::as_str).map(Arc::from),
                        command: payload.get("command").and_then(Value::as_str).map(Arc::from),
                        alignment,
                        priority: payload.get("priority").and_then(Value::as_i64).unwrap_or(0) as i32,
                    },
                );
            }
        }
        "ui.statusBar.remove" => {
            if let Some(id) = payload.get("id").and_then(Value::as_str) {
                proxy.remove_status_bar_item(&ext_id, id);
            }
        }
        "ui.notify" => {
            let level = match payload.get("level").and_then(Value::as_str) {
                Some("warning") => ExtensionNotificationLevel::Warning,
                Some("error") => ExtensionNotificationLevel::Error,
                _ => ExtensionNotificationLevel::Info,
            };
            let message = payload.get("message").and_then(Value::as_str).unwrap_or("");
            proxy.show_notification(
                ext_id,
                ExtensionNotification {
                    level,
                    message: Arc::from(message),
                },
            );
        }
        "editor.diagnostics.publish" => {
            let uri = payload.get("uri").and_then(Value::as_str).unwrap_or("");
            let diagnostics = payload
                .get("diagnostics")
                .and_then(Value::as_array)
                .map(|arr| {
                    arr.iter()
                        .map(|d| ExtensionDiagnostic {
                            line: d.get("line").and_then(Value::as_u64).unwrap_or(0) as u32,
                            character: d.get("character").and_then(Value::as_u64).unwrap_or(0) as u32,
                            end_line: d.get("end_line").and_then(Value::as_u64).unwrap_or(0) as u32,
                            end_character: d.get("end_character").and_then(Value::as_u64).unwrap_or(0) as u32,
                            severity: match d.get("severity").and_then(Value::as_str) {
                                Some("error") => ExtensionDiagnosticSeverity::Error,
                                Some("warning") => ExtensionDiagnosticSeverity::Warning,
                                Some("hint") => ExtensionDiagnosticSeverity::Hint,
                                _ => ExtensionDiagnosticSeverity::Information,
                            },
                            message: Arc::from(d.get("message").and_then(Value::as_str).unwrap_or("")),
                            source: d.get("source").and_then(Value::as_str).map(Arc::from),
                        })
                        .collect()
                })
                .unwrap_or_default();
            proxy.publish_diagnostics(ext_id, Arc::from(uri), diagnostics);
        }
        "editor.diagnostics.clear" => {
            proxy.clear_diagnostics(&ext_id);
        }
        _ => {}
    }
}

struct NodeHostState {
    registered: HashMap<String, DynamicVsixEntry>,
    running: Option<RunningHost>,
}

/// On-demand, single-shared-process VS Code compatible Node.js extension host.
///
/// Consumes 0 MB until a registered dynamic `.vsix` matches an activation event,
/// enforces `--max-old-space-size=256`, OS low priority + Win32 Job Object limits,
/// and automatically shuts down after `idle_timeout` (default 15 minutes).
pub struct VsCodeNodeHost {
    config: NodeHostConfig,
    proxy: Option<Arc<ExtensionHostProxy>>,
    state: Mutex<NodeHostState>,
}

impl VsCodeNodeHost {
    pub fn new(config: NodeHostConfig, proxy: Option<Arc<ExtensionHostProxy>>) -> Self {
        Self {
            config,
            proxy,
            state: Mutex::new(NodeHostState {
                registered: HashMap::new(),
                running: None,
            }),
        }
    }

    /// Registers a dynamic `.vsix` extension without spawning Node.js.
    pub fn register_extension(&self, entry: DynamicVsixEntry) {
        self.state
            .lock()
            .registered
            .insert(entry.extension_id.clone(), entry);
    }

    pub fn is_running(&self) -> bool {
        self.state.lock().running.is_some()
    }

    /// Triggers an activation event (e.g. `onLanguage:rust`, `onCommand:ext.run`).
    ///
    /// Spawns the shared Node.js process ONLY if at least one registered extension matches.
    pub fn trigger_activation_event(&self, event: &str) -> Result<Vec<String>> {
        let mut state = self.state.lock();
        let matching: Vec<DynamicVsixEntry> = state
            .registered
            .values()
            .filter(|e| e.matches_event(event))
            .cloned()
            .collect();

        if matching.is_empty() {
            return Ok(Vec::new());
        }

        if state.running.is_none() {
            state.running = Some(RunningHost::spawn(&self.config)?);
        }

        let host = state.running.as_mut().unwrap();
        let mut newly_activated = Vec::new();
        for entry in matching {
            if host.activated_extensions.contains(&entry.extension_id) {
                continue;
            }
            host.call(
                "activate",
                json!({
                    "extensionId": entry.extension_id,
                    "entryPath": entry.entry_path.to_string_lossy(),
                }),
                self.proxy.as_deref(),
            )?;
            host.activated_extensions.insert(entry.extension_id.clone());
            newly_activated.push(entry.extension_id);
        }
        Ok(newly_activated)
    }

    /// Executes a command contributed by a dynamic `.vsix` extension.
    pub fn execute_command(&self, command_id: &str, args: Vec<Value>) -> Result<Value> {
        // Trigger `onCommand:<id>` lazily if not yet activated.
        let _ = self.trigger_activation_event(&format!("onCommand:{command_id}"))?;

        let mut state = self.state.lock();
        let host = state
            .running
            .as_mut()
            .ok_or_else(|| anyhow!("no active node extension host for command '{command_id}'"))?;
        host.call(
            "executeCommand",
            json!({ "commandId": command_id, "args": args }),
            self.proxy.as_deref(),
        )
    }

    /// Sends a lightweight health ping to the active host (does not spawn if idle).
    pub fn ping(&self) -> Result<Option<Value>> {
        let mut state = self.state.lock();
        let Some(host) = state.running.as_mut() else {
            return Ok(None);
        };
        Ok(Some(host.call("ping", json!({}), self.proxy.as_deref())?))
    }

    /// Terminates the shared Node.js process if it has been idle for `config.idle_timeout`.
    pub fn shutdown_if_idle(&self) -> bool {
        let mut state = self.state.lock();
        let should_stop = state
            .running
            .as_ref()
            .is_some_and(|h| h.last_activity.elapsed() >= self.config.idle_timeout);
        if should_stop {
            if let Some(mut host) = state.running.take() {
                host.terminate();
            }
            return true;
        }
        false
    }

    /// Immediately shuts down the shared Node.js process if running.
    pub fn shutdown(&self) {
        if let Some(mut host) = self.state.lock().running.take() {
            host.terminate();
        }
    }

    /// Helper to check whether a path should be watched or excluded.
    pub fn node_binary(&self) -> &Path {
        &self.config.node_binary
    }
}

#[cfg(windows)]
mod win_job {
    use std::ffi::c_void;
    use std::os::windows::io::AsRawHandle;
    use std::process::Child;

    use anyhow::{Result, bail};

    type Handle = *mut c_void;

    const JOB_OBJECT_EXTENDED_LIMIT_INFORMATION_CLASS: u32 = 9;
    const JOB_OBJECT_LIMIT_PROCESS_MEMORY: u32 = 0x0000_0100;
    const JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE: u32 = 0x0000_2000;

    #[repr(C)]
    #[derive(Default)]
    struct JobObjectBasicLimitInformation {
        per_process_user_time_limit: i64,
        per_job_user_time_limit: i64,
        limit_flags: u32,
        minimum_working_set_size: usize,
        maximum_working_set_size: usize,
        active_process_limit: u32,
        affinity: usize,
        priority_class: u32,
        scheduling_class: u32,
    }

    #[repr(C)]
    #[derive(Default)]
    struct IoCounters {
        read_operation_count: u64,
        write_operation_count: u64,
        other_operation_count: u64,
        read_transfer_count: u64,
        write_transfer_count: u64,
        other_transfer_count: u64,
    }

    #[repr(C)]
    #[derive(Default)]
    struct JobObjectExtendedLimitInformation {
        basic_limit_information: JobObjectBasicLimitInformation,
        io_info: IoCounters,
        process_memory_limit: usize,
        job_memory_limit: usize,
        peak_process_memory_used: usize,
        peak_job_memory_used: usize,
    }

    unsafe extern "system" {
        fn CreateJobObjectW(lp_job_attributes: *mut c_void, lp_name: *const u16) -> Handle;
        fn SetInformationJobObject(
            h_job: Handle,
            job_object_information_class: u32,
            lp_job_object_information: *const c_void,
            cb_job_object_information_length: u32,
        ) -> i32;
        fn AssignProcessToJobObject(h_job: Handle, h_process: Handle) -> i32;
        fn CloseHandle(h_object: Handle) -> i32;
    }

    pub struct JobObjectGuard(Handle);

    // SAFETY: Win32 Job Object HANDLEs can be closed from any thread.
    unsafe impl Send for JobObjectGuard {}
    unsafe impl Sync for JobObjectGuard {}

    impl Drop for JobObjectGuard {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe {
                    CloseHandle(self.0);
                }
            }
        }
    }

    pub fn attach_process_to_limited_job(child: &Child, memory_limit_bytes: usize) -> Result<JobObjectGuard> {
        let job = unsafe { CreateJobObjectW(std::ptr::null_mut(), std::ptr::null()) };
        if job.is_null() {
            bail!("CreateJobObjectW failed");
        }
        let guard = JobObjectGuard(job);

        let mut info = JobObjectExtendedLimitInformation::default();
        info.basic_limit_information.limit_flags =
            JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE | JOB_OBJECT_LIMIT_PROCESS_MEMORY;
        info.process_memory_limit = memory_limit_bytes;

        let ok = unsafe {
            SetInformationJobObject(
                job,
                JOB_OBJECT_EXTENDED_LIMIT_INFORMATION_CLASS,
                (&info as *const JobObjectExtendedLimitInformation).cast::<c_void>(),
                std::mem::size_of::<JobObjectExtendedLimitInformation>() as u32,
            )
        };
        if ok == 0 {
            bail!("SetInformationJobObject failed");
        }

        let assigned = unsafe { AssignProcessToJobObject(job, child.as_raw_handle()) };
        if assigned == 0 {
            bail!("AssignProcessToJobObject failed");
        }

        Ok(guard)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn extended_limit_info_matches_win64_abi_size() {
            #[cfg(target_pointer_width = "64")]
            assert_eq!(std::mem::size_of::<JobObjectExtendedLimitInformation>(), 144);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::Task;

    struct RecordingCommandProxy(Arc<Mutex<Vec<ExtensionCommandDescriptor>>>);

    impl ExtensionCommandProxy for RecordingCommandProxy {
        fn register_command(&self, _extension_id: Arc<str>, command: ExtensionCommandDescriptor) -> Result<()> {
            self.0.lock().push(command);
            Ok(())
        }
        fn unregister_commands(&self, _extension_id: &str) {
            self.0.lock().clear();
        }
        fn execute_command(&self, _command_id: &str, _args_json: Option<String>) -> Task<Result<Option<String>>> {
            Task::ready(Ok(None))
        }
    }

    struct RecordingUiProxy {
        status_items: Arc<Mutex<Vec<ExtensionStatusBarItem>>>,
        notifications: Arc<Mutex<Vec<ExtensionNotification>>>,
    }

    impl ExtensionUiProxy for RecordingUiProxy {
        fn set_status_bar_item(&self, _extension_id: Arc<str>, item: ExtensionStatusBarItem) {
            self.status_items.lock().push(item);
        }
        fn remove_status_bar_item(&self, _extension_id: &str, item_id: &str) {
            self.status_items.lock().retain(|i| i.id.as_ref() != item_id);
        }
        fn show_notification(&self, _extension_id: Arc<str>, notification: ExtensionNotification) {
            self.notifications.lock().push(notification);
        }
        fn show_quick_pick(
            &self,
            _extension_id: Arc<str>,
            _request: extension::ExtensionQuickPickRequest,
        ) -> Task<Result<Option<String>>> {
            Task::ready(Ok(None))
        }
        fn show_input_box(
            &self,
            _extension_id: Arc<str>,
            _request: extension::ExtensionInputBoxRequest,
        ) -> Task<Result<Option<String>>> {
            Task::ready(Ok(None))
        }
    }

    struct RecordingEditorProxy(Arc<Mutex<Vec<(Arc<str>, Vec<ExtensionDiagnostic>)>>>);

    impl ExtensionEditorProxy for RecordingEditorProxy {
        fn active_document(&self) -> Task<Result<Option<extension::ExtensionDocumentSnapshot>>> {
            Task::ready(Ok(None))
        }
        fn publish_diagnostics(&self, _extension_id: Arc<str>, uri: Arc<str>, diagnostics: Vec<ExtensionDiagnostic>) {
            self.0.lock().push((uri, diagnostics));
        }
        fn clear_diagnostics(&self, _extension_id: &str) {
            self.0.lock().clear();
        }
    }

    #[test]
    fn node_host_real_process_lifecycle_proxies_guards_and_idle_shutdown() {
        let tmp = tempfile::tempdir().unwrap();
        let entry_js = tmp.path().join("extension.js");
        std::fs::write(
            &entry_js,
            r#"
const vscode = require('vscode');

exports.activate = function(context) {
  const sb = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 10);
  sb.text = 'Gram Plugin Active';
  sb.show();

  const diags = vscode.languages.createDiagnosticCollection('demo');
  diags.set('file:///demo.rs', [{ line: 2, character: 4, end_line: 2, end_character: 10, severity: 1, message: 'demo warning' }]);

  context.subscriptions.push(
    vscode.commands.registerCommand('demo.greet', (name) => {
      vscode.window.showInformationMessage('Hello ' + name);
      return 'greeted:' + name;
    }),
    vscode.commands.registerCommand('demo.tryWebview', () => {
      vscode.window.createWebviewPanel('test', 'Test', 1, {});
    }),
    vscode.commands.registerCommand('demo.tryTerminal', () => {
      vscode.window.createTerminal('evil');
    })
  );
};
"#,
        )
        .unwrap();

        let proxy = Arc::new(ExtensionHostProxy::new());
        let cmds = Arc::new(Mutex::new(Vec::new()));
        let status_items = Arc::new(Mutex::new(Vec::new()));
        let notifications = Arc::new(Mutex::new(Vec::new()));
        let diagnostics = Arc::new(Mutex::new(Vec::new()));

        proxy.register_command_proxy(RecordingCommandProxy(cmds.clone()));
        proxy.register_ui_proxy(RecordingUiProxy {
            status_items: status_items.clone(),
            notifications: notifications.clone(),
        });
        proxy.register_editor_proxy(RecordingEditorProxy(diagnostics.clone()));

        let host = VsCodeNodeHost::new(
            NodeHostConfig {
                idle_timeout: Duration::from_millis(60),
                ..NodeHostConfig::default()
            },
            Some(proxy),
        );

        host.register_extension(DynamicVsixEntry {
            extension_id: "gram.demo".to_string(),
            entry_path: entry_js,
            activation_events: vec!["onCommand:demo.greet".to_string()],
        });

        // 0 MB before any matching activation event fires
        assert!(!host.is_running());
        assert!(host.trigger_activation_event("onLanguage:python").unwrap().is_empty());
        assert!(!host.is_running());

        // Triggering matching command spawns the shared Node.js process on demand
        let res = host.execute_command("demo.greet", vec![json!("Gram")]).unwrap();
        assert!(host.is_running());
        assert_eq!(res["value"], json!("greeted:Gram"));

        // Verify ExtensionHostProxy received command, status bar, notification, and diagnostic
        assert!(cmds.lock().iter().any(|c| c.id.as_ref() == "demo.greet"));
        assert!(status_items.lock().iter().any(|s| s.text.as_ref() == "Gram Plugin Active"));
        assert!(notifications.lock().iter().any(|n| n.message.as_ref() == "Hello Gram"));
        assert_eq!(diagnostics.lock().len(), 1);
        assert_eq!(diagnostics.lock()[0].0.as_ref(), "file:///demo.rs");

        // Verify Webview is permanently rejected
        let webview_err = host.execute_command("demo.tryWebview", vec![]).unwrap_err();
        assert!(
            webview_err.to_string().contains("GRAM_UNSUPPORTED_WEBVIEW"),
            "unexpected webview error: {webview_err}"
        );

        // Verify integrated terminal PTY access is denied by default
        let pty_err = host.execute_command("demo.tryTerminal", vec![]).unwrap_err();
        assert!(
            pty_err.to_string().contains("GRAM_DENIED_TERMINAL_PTY"),
            "unexpected terminal error: {pty_err}"
        );

        // Verify env allowlist isolation (ping returns envKeys visible inside Node.js)
        let ping = host.ping().unwrap().unwrap();
        let env_keys = ping["envKeys"].as_array().unwrap();
        assert!(env_keys.iter().any(|k| k.as_str() == Some("GRAM_EXTENSION_HOST")));
        assert!(!env_keys.iter().any(|k| k.as_str() == Some("OPENAI_API_KEY")));

        // Verify idle auto-shutdown terminates the shared Node.js process
        std::thread::sleep(Duration::from_millis(80));
        assert!(host.shutdown_if_idle());
        assert!(!host.is_running());
    }
}
