use serde::{Deserialize, Serialize};

/// Subset of VS Code extension `package.json` relevant to Gram compatibility.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VsCodeManifest {
    pub name: String,
    #[serde(default)]
    pub publisher: String,
    pub version: String,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    /// Node.js entry point (if present, the extension has dynamic JS code).
    #[serde(default)]
    pub main: Option<String>,
    /// Browser entry point (not executed; Gram does not embed a browser engine).
    #[serde(default)]
    pub browser: Option<String>,
    #[serde(default)]
    pub activation_events: Vec<String>,
    #[serde(default)]
    pub contributes: VsCodeContributes,
}

impl VsCodeManifest {
    /// Canonical extension identifier (`publisher.name` or `name` if publisher is empty).
    pub fn extension_id(&self) -> String {
        if self.publisher.is_empty() {
            self.name.clone()
        } else {
            format!("{}.{}", self.publisher, self.name)
        }
    }

    /// Returns true if this `.vsix` contains only static resources and requires
    /// neither Node.js nor any resident background process (0 MB overhead).
    pub fn is_static_only(&self) -> bool {
        self.main.as_deref().map(str::trim).unwrap_or("").is_empty()
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VsCodeContributes {
    #[serde(default)]
    pub themes: Vec<VsCodeThemeContribution>,
    #[serde(default)]
    pub icon_themes: Vec<VsCodeIconThemeContribution>,
    #[serde(default)]
    pub snippets: Vec<VsCodeSnippetContribution>,
    #[serde(default)]
    pub keybindings: Vec<VsCodeKeybindingContribution>,
    #[serde(default)]
    pub localizations: Vec<VsCodeLocalizationContribution>,
    #[serde(default)]
    pub commands: Vec<VsCodeCommandContribution>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VsCodeThemeContribution {
    #[serde(default)]
    pub id: Option<String>,
    pub label: String,
    pub ui_theme: String,
    pub path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VsCodeIconThemeContribution {
    pub id: String,
    pub label: String,
    pub path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VsCodeSnippetContribution {
    #[serde(default)]
    pub language: Option<String>,
    pub path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VsCodeKeybindingContribution {
    pub command: String,
    pub key: String,
    #[serde(default)]
    pub mac: Option<String>,
    #[serde(default)]
    pub win: Option<String>,
    #[serde(default)]
    pub linux: Option<String>,
    #[serde(default)]
    pub when: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VsCodeLocalizationContribution {
    pub language_id: String,
    #[serde(default)]
    pub language_name: Option<String>,
    #[serde(default)]
    pub localized_language_name: Option<String>,
    #[serde(default)]
    pub translations: Vec<VsCodeTranslationEntry>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VsCodeTranslationEntry {
    pub id: String,
    pub path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VsCodeCommandContribution {
    pub command: String,
    pub title: String,
    #[serde(default)]
    pub category: Option<String>,
}

/// Permission summary presented to the user for explicit approval before installing a `.vsix`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityRequirement {
    pub kind: String,
    pub reason: String,
}

/// Extracts required permissions from a VS Code manifest so the user can review
/// and explicitly approve them before installation.
pub fn extract_required_capabilities(manifest: &VsCodeManifest) -> Vec<CapabilityRequirement> {
    let mut caps = Vec::new();
    if manifest.is_static_only() {
        return caps;
    }

    caps.push(CapabilityRequirement {
        kind: "runtime:nodejs".to_string(),
        reason: format!(
            "動的エントリポイント '{}' を共有 Node.js ホスト（256 MB 上限・15 分アイドル終了）でオンデマンド実行します",
            manifest.main.as_deref().unwrap_or("")
        ),
    });

    if !manifest.contributes.commands.is_empty() {
        caps.push(CapabilityRequirement {
            kind: "command:register".to_string(),
            reason: format!(
                "{} 件のコマンドをコマンドパレットへ登録します",
                manifest.contributes.commands.len()
            ),
        });
    }

    caps.push(CapabilityRequirement {
        kind: "workspace:read".to_string(),
        reason: "開いている文書・保存イベントへの読み取りアクセス（内蔵ターミナル PTY・Webview は既定禁止）"
            .to_string(),
    });

    caps
}
