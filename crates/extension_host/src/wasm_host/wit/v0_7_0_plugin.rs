use std::sync::Arc;

use extension::{
    ExtensionCapability, ExtensionCommandDescriptor, ExtensionCommandProxy, ExtensionDiagnostic,
    ExtensionDiagnosticSeverity, ExtensionEditorProxy, ExtensionLocaleProxy, ExtensionNotification,
    ExtensionNotificationLevel, ExtensionQuickPickItem, ExtensionQuickPickRequest, ExtensionInputBoxRequest,
    ExtensionStatusBarAlignment, ExtensionStatusBarItem, ExtensionUiProxy,
};
use semver::Version as SemanticVersion;

use crate::wasm_host::WasmState;

pub const MIN_VERSION: SemanticVersion = SemanticVersion::new(0, 7, 0);

wasmtime::component::bindgen!({
    imports: {
        default: async | trappable,
    },
    exports: {
        default: async,
    },
    path: "../extension_api/wit/since_v0.7.0",
    world: "plugin",
    anyhow: true,
});

impl gram::plugin::commands::Host for WasmState {
    async fn register_command(
        &mut self,
        command: gram::plugin::commands::CommandDescriptor,
    ) -> anyhow::Result<Result<(), String>> {
        if let Err(err) = self
            .capability_granter
            .grant_capability(&ExtensionCapability::CommandRegister)
        {
            return Ok(Err(err.to_string()));
        }
        let res = self.host.proxy.register_command(
            self.manifest.id.clone(),
            ExtensionCommandDescriptor {
                id: Arc::from(command.id),
                title: Arc::from(command.title),
                description: command.description.map(Arc::from),
            },
        );
        Ok(res.map_err(|e| e.to_string()))
    }

    async fn unregister_commands(&mut self) -> anyhow::Result<()> {
        self.host.proxy.unregister_commands(&self.manifest.id);
        Ok(())
    }
}

impl gram::plugin::ui::Host for WasmState {
    async fn set_status_bar_item(
        &mut self,
        item: gram::plugin::ui::StatusBarItem,
    ) -> anyhow::Result<Result<(), String>> {
        if let Err(err) = self
            .capability_granter
            .grant_capability(&ExtensionCapability::UiStatusBar)
        {
            return Ok(Err(err.to_string()));
        }
        let alignment = match item.alignment {
            gram::plugin::ui::StatusBarAlignment::Left => ExtensionStatusBarAlignment::Left,
            gram::plugin::ui::StatusBarAlignment::Right => ExtensionStatusBarAlignment::Right,
        };
        self.host.proxy.set_status_bar_item(
            self.manifest.id.clone(),
            ExtensionStatusBarItem {
                id: Arc::from(item.id),
                text: Arc::from(item.text),
                tooltip: item.tooltip.map(Arc::from),
                command: item.command.map(Arc::from),
                alignment,
                priority: item.priority,
            },
        );
        Ok(Ok(()))
    }

    async fn remove_status_bar_item(&mut self, item_id: String) -> anyhow::Result<()> {
        self.host
            .proxy
            .remove_status_bar_item(&self.manifest.id, &item_id);
        Ok(())
    }

    async fn show_notification(
        &mut self,
        level: gram::plugin::ui::NotificationLevel,
        message: String,
    ) -> anyhow::Result<Result<(), String>> {
        if let Err(err) = self
            .capability_granter
            .grant_capability(&ExtensionCapability::UiNotify)
        {
            return Ok(Err(err.to_string()));
        }
        let level = match level {
            gram::plugin::ui::NotificationLevel::Info => ExtensionNotificationLevel::Info,
            gram::plugin::ui::NotificationLevel::Warning => ExtensionNotificationLevel::Warning,
            gram::plugin::ui::NotificationLevel::Error => ExtensionNotificationLevel::Error,
        };
        self.host.proxy.show_notification(
            self.manifest.id.clone(),
            ExtensionNotification {
                level,
                message: Arc::from(message),
            },
        );
        Ok(Ok(()))
    }

    async fn show_quick_pick(
        &mut self,
        request: gram::plugin::ui::QuickPickRequest,
    ) -> anyhow::Result<Result<Option<String>, String>> {
        if let Err(err) = self
            .capability_granter
            .grant_capability(&ExtensionCapability::UiPrompt)
        {
            return Ok(Err(err.to_string()));
        }
        let req = ExtensionQuickPickRequest {
            placeholder: request.placeholder.map(Arc::from),
            items: request
                .items
                .into_iter()
                .map(|i| ExtensionQuickPickItem {
                    id: Arc::from(i.id),
                    label: Arc::from(i.label),
                    description: i.description.map(Arc::from),
                })
                .collect(),
        };
        let res = self
            .host
            .proxy
            .show_quick_pick(self.manifest.id.clone(), req)
            .await;
        Ok(res.map_err(|e| e.to_string()))
    }

    async fn show_input_box(
        &mut self,
        request: gram::plugin::ui::InputBoxRequest,
    ) -> anyhow::Result<Result<Option<String>, String>> {
        if let Err(err) = self
            .capability_granter
            .grant_capability(&ExtensionCapability::UiPrompt)
        {
            return Ok(Err(err.to_string()));
        }
        let req = ExtensionInputBoxRequest {
            prompt: request.prompt.map(Arc::from),
            placeholder: request.placeholder.map(Arc::from),
            value: request.value.map(Arc::from),
            password: request.password,
        };
        let res = self
            .host
            .proxy
            .show_input_box(self.manifest.id.clone(), req)
            .await;
        Ok(res.map_err(|e| e.to_string()))
    }
}

impl gram::plugin::editor::Host for WasmState {
    async fn active_document(
        &mut self,
    ) -> anyhow::Result<Result<Option<gram::plugin::editor::DocumentSnapshot>, String>> {
        if let Err(err) = self
            .capability_granter
            .grant_capability(&ExtensionCapability::WorkspaceRead)
        {
            return Ok(Err(err.to_string()));
        }
        let res = self.host.proxy.active_document().await;
        Ok(res
            .map(|opt| {
                opt.map(|doc| gram::plugin::editor::DocumentSnapshot {
                    uri: doc.uri.to_string(),
                    language_id: doc.language_id.to_string(),
                    version: doc.version,
                    text: doc.text,
                })
            })
            .map_err(|e| e.to_string()))
    }

    async fn publish_diagnostics(
        &mut self,
        uri: String,
        diagnostics: Vec<gram::plugin::editor::Diagnostic>,
    ) -> anyhow::Result<Result<(), String>> {
        if let Err(err) = self
            .capability_granter
            .grant_capability(&ExtensionCapability::WorkspaceRead)
        {
            return Ok(Err(err.to_string()));
        }
        let converted = diagnostics
            .into_iter()
            .map(|d| ExtensionDiagnostic {
                line: d.line,
                character: d.character,
                end_line: d.end_line,
                end_character: d.end_character,
                severity: match d.severity {
                    gram::plugin::editor::DiagnosticSeverity::Error => ExtensionDiagnosticSeverity::Error,
                    gram::plugin::editor::DiagnosticSeverity::Warning => ExtensionDiagnosticSeverity::Warning,
                    gram::plugin::editor::DiagnosticSeverity::Information => ExtensionDiagnosticSeverity::Information,
                    gram::plugin::editor::DiagnosticSeverity::Hint => ExtensionDiagnosticSeverity::Hint,
                },
                message: Arc::from(d.message),
                source: d.source.map(Arc::from),
            })
            .collect();
        self.host
            .proxy
            .publish_diagnostics(self.manifest.id.clone(), Arc::from(uri), converted);
        Ok(Ok(()))
    }

    async fn clear_diagnostics(&mut self) -> anyhow::Result<()> {
        self.host.proxy.clear_diagnostics(&self.manifest.id);
        Ok(())
    }
}

impl gram::plugin::locale::Host for WasmState {
    async fn register_locale_pack(
        &mut self,
        locale_id: String,
        relative_path: String,
    ) -> anyhow::Result<Result<(), String>> {
        let full_path = self
            .host
            .work_dir
            .join(self.manifest.id.as_ref())
            .join(relative_path);
        let res = self.host.proxy.register_locale_pack(
            self.manifest.id.clone(),
            Arc::from(locale_id),
            full_path,
        );
        Ok(res.map_err(|e| e.to_string()))
    }

    async fn unregister_locale_pack(&mut self) -> anyhow::Result<()> {
        self.host.proxy.unregister_locale_pack(&self.manifest.id);
        Ok(())
    }
}
