use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Result;
use fs::Fs;
use gpui::{App, Global, ReadGlobal, SharedString, Task};
use language::{BinaryStatus, LanguageMatcher, LanguageName, LoadedLanguage};
use lsp::LanguageServerName;
use parking_lot::RwLock;

use serde::{Deserialize, Serialize};

use crate::Extension;

#[derive(Default)]
struct GlobalExtensionHostProxy(Arc<ExtensionHostProxy>);

impl Global for GlobalExtensionHostProxy {}

/// A proxy for interacting with the extension host.
///
/// This object implements each of the individual proxy types so that their
/// methods can be called directly on it.
#[derive(Default)]
pub struct ExtensionHostProxy {
    theme_proxy: RwLock<Option<Arc<dyn ExtensionThemeProxy>>>,
    grammar_proxy: RwLock<Option<Arc<dyn ExtensionGrammarProxy>>>,
    language_proxy: RwLock<Option<Arc<dyn ExtensionLanguageProxy>>>,
    language_server_proxy: RwLock<Option<Arc<dyn ExtensionLanguageServerProxy>>>,
    snippet_proxy: RwLock<Option<Arc<dyn ExtensionSnippetProxy>>>,
    debug_adapter_provider_proxy: RwLock<Option<Arc<dyn ExtensionDebugAdapterProviderProxy>>>,
    locale_proxy: RwLock<Option<Arc<dyn ExtensionLocaleProxy>>>,
    command_proxy: RwLock<Option<Arc<dyn ExtensionCommandProxy>>>,
    ui_proxy: RwLock<Option<Arc<dyn ExtensionUiProxy>>>,
    editor_proxy: RwLock<Option<Arc<dyn ExtensionEditorProxy>>>,
}

impl ExtensionHostProxy {
    /// Returns the global [`ExtensionHostProxy`].
    pub fn global(cx: &App) -> Arc<Self> {
        GlobalExtensionHostProxy::global(cx).0.clone()
    }

    /// Returns the global [`ExtensionHostProxy`].
    ///
    /// Inserts a default [`ExtensionHostProxy`] if one does not yet exist.
    pub fn default_global(cx: &mut App) -> Arc<Self> {
        cx.default_global::<GlobalExtensionHostProxy>().0.clone()
    }

    pub fn new() -> Self {
        Self {
            theme_proxy: RwLock::default(),
            grammar_proxy: RwLock::default(),
            language_proxy: RwLock::default(),
            language_server_proxy: RwLock::default(),
            snippet_proxy: RwLock::default(),
            debug_adapter_provider_proxy: RwLock::default(),
            locale_proxy: RwLock::default(),
            command_proxy: RwLock::default(),
            ui_proxy: RwLock::default(),
            editor_proxy: RwLock::default(),
        }
    }

    pub fn register_theme_proxy(&self, proxy: impl ExtensionThemeProxy) {
        self.theme_proxy.write().replace(Arc::new(proxy));
    }

    pub fn register_grammar_proxy(&self, proxy: impl ExtensionGrammarProxy) {
        self.grammar_proxy.write().replace(Arc::new(proxy));
    }

    pub fn register_language_proxy(&self, proxy: impl ExtensionLanguageProxy) {
        self.language_proxy.write().replace(Arc::new(proxy));
    }

    pub fn register_language_server_proxy(&self, proxy: impl ExtensionLanguageServerProxy) {
        self.language_server_proxy.write().replace(Arc::new(proxy));
    }

    pub fn register_snippet_proxy(&self, proxy: impl ExtensionSnippetProxy) {
        self.snippet_proxy.write().replace(Arc::new(proxy));
    }

    pub fn register_debug_adapter_proxy(&self, proxy: impl ExtensionDebugAdapterProviderProxy) {
        self.debug_adapter_provider_proxy.write().replace(Arc::new(proxy));
    }

    pub fn register_locale_proxy(&self, proxy: impl ExtensionLocaleProxy) {
        self.locale_proxy.write().replace(Arc::new(proxy));
    }

    pub fn register_command_proxy(&self, proxy: impl ExtensionCommandProxy) {
        self.command_proxy.write().replace(Arc::new(proxy));
    }

    pub fn register_ui_proxy(&self, proxy: impl ExtensionUiProxy) {
        self.ui_proxy.write().replace(Arc::new(proxy));
    }

    pub fn register_editor_proxy(&self, proxy: impl ExtensionEditorProxy) {
        self.editor_proxy.write().replace(Arc::new(proxy));
    }
}

pub trait ExtensionThemeProxy: Send + Sync + 'static {
    fn set_extensions_loaded(&self);

    fn list_theme_names(&self, theme_path: PathBuf, fs: Arc<dyn Fs>) -> Task<Result<Vec<String>>>;

    fn remove_user_themes(&self, themes: Vec<SharedString>);

    fn load_user_theme(&self, theme_path: PathBuf, fs: Arc<dyn Fs>) -> Task<Result<()>>;

    fn reload_current_theme(&self, cx: &mut App);

    fn list_icon_theme_names(&self, icon_theme_path: PathBuf, fs: Arc<dyn Fs>) -> Task<Result<Vec<String>>>;

    fn remove_icon_themes(&self, icon_themes: Vec<SharedString>);

    fn load_icon_theme(&self, icon_theme_path: PathBuf, icons_root_dir: PathBuf, fs: Arc<dyn Fs>) -> Task<Result<()>>;

    fn reload_current_icon_theme(&self, cx: &mut App);
}

impl ExtensionThemeProxy for ExtensionHostProxy {
    fn set_extensions_loaded(&self) {
        let Some(proxy) = self.theme_proxy.read().clone() else {
            return;
        };

        proxy.set_extensions_loaded()
    }

    fn list_theme_names(&self, theme_path: PathBuf, fs: Arc<dyn Fs>) -> Task<Result<Vec<String>>> {
        let Some(proxy) = self.theme_proxy.read().clone() else {
            return Task::ready(Ok(Vec::new()));
        };

        proxy.list_theme_names(theme_path, fs)
    }

    fn remove_user_themes(&self, themes: Vec<SharedString>) {
        let Some(proxy) = self.theme_proxy.read().clone() else {
            return;
        };

        proxy.remove_user_themes(themes)
    }

    fn load_user_theme(&self, theme_path: PathBuf, fs: Arc<dyn Fs>) -> Task<Result<()>> {
        let Some(proxy) = self.theme_proxy.read().clone() else {
            return Task::ready(Ok(()));
        };

        proxy.load_user_theme(theme_path, fs)
    }

    fn reload_current_theme(&self, cx: &mut App) {
        let Some(proxy) = self.theme_proxy.read().clone() else {
            return;
        };

        proxy.reload_current_theme(cx)
    }

    fn list_icon_theme_names(&self, icon_theme_path: PathBuf, fs: Arc<dyn Fs>) -> Task<Result<Vec<String>>> {
        let Some(proxy) = self.theme_proxy.read().clone() else {
            return Task::ready(Ok(Vec::new()));
        };

        proxy.list_icon_theme_names(icon_theme_path, fs)
    }

    fn remove_icon_themes(&self, icon_themes: Vec<SharedString>) {
        let Some(proxy) = self.theme_proxy.read().clone() else {
            return;
        };

        proxy.remove_icon_themes(icon_themes)
    }

    fn load_icon_theme(&self, icon_theme_path: PathBuf, icons_root_dir: PathBuf, fs: Arc<dyn Fs>) -> Task<Result<()>> {
        let Some(proxy) = self.theme_proxy.read().clone() else {
            return Task::ready(Ok(()));
        };

        proxy.load_icon_theme(icon_theme_path, icons_root_dir, fs)
    }

    fn reload_current_icon_theme(&self, cx: &mut App) {
        let Some(proxy) = self.theme_proxy.read().clone() else {
            return;
        };

        proxy.reload_current_icon_theme(cx)
    }
}

pub trait ExtensionGrammarProxy: Send + Sync + 'static {
    fn register_grammars(&self, grammars: Vec<(Arc<str>, PathBuf)>);
}

impl ExtensionGrammarProxy for ExtensionHostProxy {
    fn register_grammars(&self, grammars: Vec<(Arc<str>, PathBuf)>) {
        let Some(proxy) = self.grammar_proxy.read().clone() else {
            return;
        };

        proxy.register_grammars(grammars)
    }
}

pub trait ExtensionLanguageProxy: Send + Sync + 'static {
    fn register_language(
        &self,
        language: LanguageName,
        grammar: Option<Arc<str>>,
        matcher: LanguageMatcher,
        hidden: bool,
        load: Arc<dyn Fn() -> Result<LoadedLanguage> + Send + Sync + 'static>,
    );

    fn remove_languages(&self, languages_to_remove: &[LanguageName], grammars_to_remove: &[Arc<str>]);
}

impl ExtensionLanguageProxy for ExtensionHostProxy {
    fn register_language(
        &self,
        language: LanguageName,
        grammar: Option<Arc<str>>,
        matcher: LanguageMatcher,
        hidden: bool,
        load: Arc<dyn Fn() -> Result<LoadedLanguage> + Send + Sync + 'static>,
    ) {
        let Some(proxy) = self.language_proxy.read().clone() else {
            return;
        };

        proxy.register_language(language, grammar, matcher, hidden, load)
    }

    fn remove_languages(&self, languages_to_remove: &[LanguageName], grammars_to_remove: &[Arc<str>]) {
        let Some(proxy) = self.language_proxy.read().clone() else {
            return;
        };

        proxy.remove_languages(languages_to_remove, grammars_to_remove)
    }
}

pub trait ExtensionLanguageServerProxy: Send + Sync + 'static {
    fn register_language_server(
        &self,
        extension: Arc<dyn Extension>,
        language_server_id: LanguageServerName,
        language: LanguageName,
    );

    fn remove_language_server(
        &self,
        language: &LanguageName,
        language_server_id: &LanguageServerName,
        cx: &mut App,
    ) -> Task<Result<()>>;

    fn update_language_server_status(&self, language_server_id: LanguageServerName, status: BinaryStatus);
}

impl ExtensionLanguageServerProxy for ExtensionHostProxy {
    fn register_language_server(
        &self,
        extension: Arc<dyn Extension>,
        language_server_id: LanguageServerName,
        language: LanguageName,
    ) {
        let Some(proxy) = self.language_server_proxy.read().clone() else {
            return;
        };

        proxy.register_language_server(extension, language_server_id, language)
    }

    fn remove_language_server(
        &self,
        language: &LanguageName,
        language_server_id: &LanguageServerName,
        cx: &mut App,
    ) -> Task<Result<()>> {
        let Some(proxy) = self.language_server_proxy.read().clone() else {
            return Task::ready(Ok(()));
        };

        proxy.remove_language_server(language, language_server_id, cx)
    }

    fn update_language_server_status(&self, language_server_id: LanguageServerName, status: BinaryStatus) {
        let Some(proxy) = self.language_server_proxy.read().clone() else {
            return;
        };

        proxy.update_language_server_status(language_server_id, status)
    }
}

pub trait ExtensionSnippetProxy: Send + Sync + 'static {
    fn register_snippet(&self, path: &Path, snippet_contents: &str) -> Result<()>;
}

impl ExtensionSnippetProxy for ExtensionHostProxy {
    fn register_snippet(&self, path: &Path, snippet_contents: &str) -> Result<()> {
        let Some(proxy) = self.snippet_proxy.read().clone() else {
            return Ok(());
        };

        proxy.register_snippet(path, snippet_contents)
    }
}

pub trait ExtensionDebugAdapterProviderProxy: Send + Sync + 'static {
    fn register_debug_adapter(&self, extension: Arc<dyn Extension>, debug_adapter_name: Arc<str>, schema_path: &Path);
    fn register_debug_locator(&self, extension: Arc<dyn Extension>, locator_name: Arc<str>);
    fn unregister_debug_adapter(&self, debug_adapter_name: Arc<str>);
    fn unregister_debug_locator(&self, locator_name: Arc<str>);
}

impl ExtensionDebugAdapterProviderProxy for ExtensionHostProxy {
    fn register_debug_adapter(&self, extension: Arc<dyn Extension>, debug_adapter_name: Arc<str>, schema_path: &Path) {
        let Some(proxy) = self.debug_adapter_provider_proxy.read().clone() else {
            return;
        };

        proxy.register_debug_adapter(extension, debug_adapter_name, schema_path)
    }

    fn register_debug_locator(&self, extension: Arc<dyn Extension>, locator_name: Arc<str>) {
        let Some(proxy) = self.debug_adapter_provider_proxy.read().clone() else {
            return;
        };

        proxy.register_debug_locator(extension, locator_name)
    }
    fn unregister_debug_adapter(&self, debug_adapter_name: Arc<str>) {
        let Some(proxy) = self.debug_adapter_provider_proxy.read().clone() else {
            return;
        };

        proxy.unregister_debug_adapter(debug_adapter_name)
    }
    fn unregister_debug_locator(&self, locator_name: Arc<str>) {
        let Some(proxy) = self.debug_adapter_provider_proxy.read().clone() else {
            return;
        };

        proxy.unregister_debug_locator(locator_name)
    }
}

pub trait ExtensionLocaleProxy: Send + Sync + 'static {
    fn register_locale_pack(&self, extension_id: Arc<str>, locale: Arc<str>, locale_path: PathBuf) -> Result<()>;

    fn unregister_locale_pack(&self, extension_id: &str);
}

impl ExtensionLocaleProxy for ExtensionHostProxy {
    fn register_locale_pack(&self, extension_id: Arc<str>, locale: Arc<str>, locale_path: PathBuf) -> Result<()> {
        let Some(proxy) = self.locale_proxy.read().clone() else {
            return Ok(());
        };

        proxy.register_locale_pack(extension_id, locale, locale_path)
    }

    fn unregister_locale_pack(&self, extension_id: &str) {
        let Some(proxy) = self.locale_proxy.read().clone() else {
            return;
        };

        proxy.unregister_locale_pack(extension_id)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionCommandDescriptor {
    pub id: Arc<str>,
    pub title: Arc<str>,
    pub description: Option<Arc<str>>,
}

pub trait ExtensionCommandProxy: Send + Sync + 'static {
    fn register_command(&self, extension_id: Arc<str>, command: ExtensionCommandDescriptor) -> Result<()>;

    fn unregister_commands(&self, extension_id: &str);

    fn execute_command(&self, command_id: &str, args_json: Option<String>) -> Task<Result<Option<String>>>;
}

impl ExtensionCommandProxy for ExtensionHostProxy {
    fn register_command(&self, extension_id: Arc<str>, command: ExtensionCommandDescriptor) -> Result<()> {
        let Some(proxy) = self.command_proxy.read().clone() else {
            return Ok(());
        };
        proxy.register_command(extension_id, command)
    }

    fn unregister_commands(&self, extension_id: &str) {
        let Some(proxy) = self.command_proxy.read().clone() else {
            return;
        };
        proxy.unregister_commands(extension_id)
    }

    fn execute_command(&self, command_id: &str, args_json: Option<String>) -> Task<Result<Option<String>>> {
        let Some(proxy) = self.command_proxy.read().clone() else {
            return Task::ready(Ok(None));
        };
        proxy.execute_command(command_id, args_json)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionStatusBarAlignment {
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionStatusBarItem {
    pub id: Arc<str>,
    pub text: Arc<str>,
    pub tooltip: Option<Arc<str>>,
    pub command: Option<Arc<str>>,
    pub alignment: ExtensionStatusBarAlignment,
    pub priority: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionNotificationLevel {
    Info,
    Warning,
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionNotification {
    pub level: ExtensionNotificationLevel,
    pub message: Arc<str>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionQuickPickItem {
    pub id: Arc<str>,
    pub label: Arc<str>,
    pub description: Option<Arc<str>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionQuickPickRequest {
    pub placeholder: Option<Arc<str>>,
    pub items: Vec<ExtensionQuickPickItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionInputBoxRequest {
    pub prompt: Option<Arc<str>>,
    pub placeholder: Option<Arc<str>>,
    pub value: Option<Arc<str>>,
    pub password: bool,
}

/// Declarative GPUI-native UI extension proxy.
/// Webview / embedded browser surfaces are permanently excluded by design.
pub trait ExtensionUiProxy: Send + Sync + 'static {
    fn set_status_bar_item(&self, extension_id: Arc<str>, item: ExtensionStatusBarItem);

    fn remove_status_bar_item(&self, extension_id: &str, item_id: &str);

    fn show_notification(&self, extension_id: Arc<str>, notification: ExtensionNotification);

    fn show_quick_pick(&self, extension_id: Arc<str>, request: ExtensionQuickPickRequest)
        -> Task<Result<Option<String>>>;

    fn show_input_box(&self, extension_id: Arc<str>, request: ExtensionInputBoxRequest) -> Task<Result<Option<String>>>;
}

impl ExtensionUiProxy for ExtensionHostProxy {
    fn set_status_bar_item(&self, extension_id: Arc<str>, item: ExtensionStatusBarItem) {
        let Some(proxy) = self.ui_proxy.read().clone() else {
            return;
        };
        proxy.set_status_bar_item(extension_id, item)
    }

    fn remove_status_bar_item(&self, extension_id: &str, item_id: &str) {
        let Some(proxy) = self.ui_proxy.read().clone() else {
            return;
        };
        proxy.remove_status_bar_item(extension_id, item_id)
    }

    fn show_notification(&self, extension_id: Arc<str>, notification: ExtensionNotification) {
        let Some(proxy) = self.ui_proxy.read().clone() else {
            return;
        };
        proxy.show_notification(extension_id, notification)
    }

    fn show_quick_pick(
        &self,
        extension_id: Arc<str>,
        request: ExtensionQuickPickRequest,
    ) -> Task<Result<Option<String>>> {
        let Some(proxy) = self.ui_proxy.read().clone() else {
            return Task::ready(Ok(None));
        };
        proxy.show_quick_pick(extension_id, request)
    }

    fn show_input_box(&self, extension_id: Arc<str>, request: ExtensionInputBoxRequest) -> Task<Result<Option<String>>> {
        let Some(proxy) = self.ui_proxy.read().clone() else {
            return Task::ready(Ok(None));
        };
        proxy.show_input_box(extension_id, request)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionDocumentSnapshot {
    pub uri: Arc<str>,
    pub language_id: Arc<str>,
    pub version: u64,
    pub text: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionDiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionDiagnostic {
    pub line: u32,
    pub character: u32,
    pub end_line: u32,
    pub end_character: u32,
    pub severity: ExtensionDiagnosticSeverity,
    pub message: Arc<str>,
    pub source: Option<Arc<str>>,
}

pub trait ExtensionEditorProxy: Send + Sync + 'static {
    fn active_document(&self) -> Task<Result<Option<ExtensionDocumentSnapshot>>>;

    fn publish_diagnostics(&self, extension_id: Arc<str>, uri: Arc<str>, diagnostics: Vec<ExtensionDiagnostic>);

    fn clear_diagnostics(&self, extension_id: &str);
}

impl ExtensionEditorProxy for ExtensionHostProxy {
    fn active_document(&self) -> Task<Result<Option<ExtensionDocumentSnapshot>>> {
        let Some(proxy) = self.editor_proxy.read().clone() else {
            return Task::ready(Ok(None));
        };
        proxy.active_document()
    }

    fn publish_diagnostics(&self, extension_id: Arc<str>, uri: Arc<str>, diagnostics: Vec<ExtensionDiagnostic>) {
        let Some(proxy) = self.editor_proxy.read().clone() else {
            return;
        };
        proxy.publish_diagnostics(extension_id, uri, diagnostics)
    }

    fn clear_diagnostics(&self, extension_id: &str) {
        let Some(proxy) = self.editor_proxy.read().clone() else {
            return;
        };
        proxy.clear_diagnostics(extension_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parking_lot::Mutex;

    struct RecordingCommandProxy(Arc<Mutex<Vec<ExtensionCommandDescriptor>>>);

    impl ExtensionCommandProxy for RecordingCommandProxy {
        fn register_command(&self, _extension_id: Arc<str>, command: ExtensionCommandDescriptor) -> Result<()> {
            self.0.lock().push(command);
            Ok(())
        }

        fn unregister_commands(&self, _extension_id: &str) {
            self.0.lock().clear();
        }

        fn execute_command(&self, command_id: &str, _args_json: Option<String>) -> Task<Result<Option<String>>> {
            Task::ready(Ok(Some(format!("ran:{command_id}"))))
        }
    }

    struct RecordingUiProxy(Arc<Mutex<Vec<ExtensionNotification>>>);

    impl ExtensionUiProxy for RecordingUiProxy {
        fn set_status_bar_item(&self, _extension_id: Arc<str>, _item: ExtensionStatusBarItem) {}
        fn remove_status_bar_item(&self, _extension_id: &str, _item_id: &str) {}
        fn show_notification(&self, _extension_id: Arc<str>, notification: ExtensionNotification) {
            self.0.lock().push(notification);
        }
        fn show_quick_pick(
            &self,
            _extension_id: Arc<str>,
            request: ExtensionQuickPickRequest,
        ) -> Task<Result<Option<String>>> {
            Task::ready(Ok(request.items.first().map(|i| i.id.to_string())))
        }
        fn show_input_box(
            &self,
            _extension_id: Arc<str>,
            request: ExtensionInputBoxRequest,
        ) -> Task<Result<Option<String>>> {
            Task::ready(Ok(request.value.map(|v| v.to_string())))
        }
    }

    struct RecordingEditorProxy(Arc<Mutex<Vec<(Arc<str>, Vec<ExtensionDiagnostic>)>>>);

    impl ExtensionEditorProxy for RecordingEditorProxy {
        fn active_document(&self) -> Task<Result<Option<ExtensionDocumentSnapshot>>> {
            Task::ready(Ok(Some(ExtensionDocumentSnapshot {
                uri: Arc::from("file:///main.rs"),
                language_id: Arc::from("rust"),
                version: 1,
                text: "fn main() {}".to_string(),
            })))
        }

        fn publish_diagnostics(&self, _extension_id: Arc<str>, uri: Arc<str>, diagnostics: Vec<ExtensionDiagnostic>) {
            self.0.lock().push((uri, diagnostics));
        }

        fn clear_diagnostics(&self, _extension_id: &str) {
            self.0.lock().clear();
        }
    }

    #[test]
    fn command_ui_editor_proxies_delegate_cleanly() {
        let host = ExtensionHostProxy::new();
        let commands = Arc::new(Mutex::new(Vec::new()));
        let notifications = Arc::new(Mutex::new(Vec::new()));
        let diagnostics = Arc::new(Mutex::new(Vec::new()));

        host.register_command_proxy(RecordingCommandProxy(commands.clone()));
        host.register_ui_proxy(RecordingUiProxy(notifications.clone()));
        host.register_editor_proxy(RecordingEditorProxy(diagnostics.clone()));

        host.register_command(
            Arc::from("ext-a"),
            ExtensionCommandDescriptor {
                id: Arc::from("ext-a.hello"),
                title: Arc::from("Hello"),
                description: Some(Arc::from("あいさつを実行")),
            },
        )
        .unwrap();
        assert_eq!(commands.lock().len(), 1);

        host.show_notification(
            Arc::from("ext-a"),
            ExtensionNotification {
                level: ExtensionNotificationLevel::Info,
                message: Arc::from("完了"),
            },
        );
        assert_eq!(notifications.lock().len(), 1);

        host.publish_diagnostics(
            Arc::from("ext-a"),
            Arc::from("file:///main.rs"),
            vec![ExtensionDiagnostic {
                line: 0,
                character: 0,
                end_line: 0,
                end_character: 2,
                severity: ExtensionDiagnosticSeverity::Warning,
                message: Arc::from("unused"),
                source: Some(Arc::from("ext-a")),
            }],
        );
        assert_eq!(diagnostics.lock().len(), 1);

        host.unregister_commands("ext-a");
        host.clear_diagnostics("ext-a");
        assert!(commands.lock().is_empty());
        assert!(diagnostics.lock().is_empty());
    }
}
