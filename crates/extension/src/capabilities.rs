mod download_file_capability;
mod npm_install_package_capability;
mod process_exec_capability;

pub use download_file_capability::*;
pub use npm_install_package_capability::*;
pub use process_exec_capability::*;

use serde::{Deserialize, Serialize};

/// Unified capability model shared across Wasm extensions and local `.vsix` plugins.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ExtensionCapability {
    #[serde(rename = "process:exec")]
    ProcessExec(ProcessExecCapability),
    DownloadFile(DownloadFileCapability),
    #[serde(rename = "npm:install")]
    NpmInstallPackage(NpmInstallPackageCapability),
    /// Read workspace documents and file metadata (excluding ignored build dirs).
    #[serde(rename = "workspace:read")]
    WorkspaceRead,
    /// Modify workspace files or document buffers.
    #[serde(rename = "workspace:write")]
    WorkspaceWrite,
    /// Show declarative GPUI notifications.
    #[serde(rename = "ui:notify")]
    UiNotify,
    /// Update declarative GPUI status bar items.
    #[serde(rename = "ui:status_bar")]
    UiStatusBar,
    /// Show declarative GPUI QuickPick / InputBox prompts.
    #[serde(rename = "ui:prompt")]
    UiPrompt,
    /// Register commands in the Command Palette.
    #[serde(rename = "command:register")]
    CommandRegister,
    /// Outbound HTTP access restricted to a specific host (denied by default).
    #[serde(rename = "network:http")]
    NetworkHttp { host: String },
    /// Integrated terminal PTY access (strictly denied by default to protect AI agents).
    #[serde(rename = "terminal:pty")]
    TerminalPty,
}

impl ExtensionCapability {
    /// Returns true if this capability is denied unless explicitly approved by the user.
    pub fn is_denied_by_default(&self) -> bool {
        matches!(
            self,
            Self::TerminalPty | Self::NetworkHttp { .. } | Self::WorkspaceWrite
        )
    }

    /// Human-readable Japanese label for capability approval prompts.
    pub fn display_label_ja(&self) -> String {
        match self {
            Self::ProcessExec(cap) => format!("外部プロセス実行 ({})", cap.command),
            Self::DownloadFile(cap) => format!("ファイルダウンロード ({})", cap.host),
            Self::NpmInstallPackage(cap) => format!("npm パッケージ導入 ({})", cap.package),
            Self::WorkspaceRead => "ワークスペース文書の読み取り".to_string(),
            Self::WorkspaceWrite => "ワークスペース文書の書き込み".to_string(),
            Self::UiNotify => "通知メッセージの表示".to_string(),
            Self::UiStatusBar => "ステータスバー項目の表示".to_string(),
            Self::UiPrompt => "選択リスト・入力ボックスの表示".to_string(),
            Self::CommandRegister => "コマンドパレットへのコマンド登録".to_string(),
            Self::NetworkHttp { host } => format!("HTTP 通信 ({host})"),
            Self::TerminalPty => "内蔵ターミナル PTY アクセス（既定禁止）".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_pty_is_denied_by_default_and_roundtrips_json() {
        let cap = ExtensionCapability::TerminalPty;
        assert!(cap.is_denied_by_default());
        let json = serde_json::to_string(&cap).unwrap();
        assert_eq!(json, r#"{"kind":"terminal:pty"}"#);
        let parsed: ExtensionCapability = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, cap);

        assert!(!ExtensionCapability::WorkspaceRead.is_denied_by_default());
        assert!(!ExtensionCapability::UiNotify.is_denied_by_default());
        assert!(!ExtensionCapability::CommandRegister.is_denied_by_default());
        assert!(ExtensionCapability::WorkspaceWrite.is_denied_by_default());
    }
}
