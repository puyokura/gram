pub mod archive;
pub mod converter;
pub mod manifest;

use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context as _, Result};
use extension::{ExtensionHostProxy, ExtensionLocaleProxy, ExtensionSnippetProxy};

pub use archive::{ExtractedVsix, MAX_VSIX_ENTRIES, MAX_VSIX_UNCOMPRESSED_BYTES, extract_vsix_safely};
pub use converter::{
    ConvertedKeybinding, KeybindingConversionReport, convert_vscode_icon_theme_file, convert_vscode_keybindings,
    convert_vscode_theme_file,
};
pub use manifest::{CapabilityRequirement, VsCodeManifest, extract_required_capabilities};

/// Result of importing a local `.vsix` archive into Gram's static resource layer.
#[derive(Debug, Clone)]
pub struct VsixStaticInstallReport {
    pub extension_id: String,
    pub version: String,
    pub is_static_only: bool,
    pub install_dir: PathBuf,
    pub required_capabilities: Vec<CapabilityRequirement>,
    pub generated_themes: Vec<PathBuf>,
    pub generated_icon_themes: Vec<PathBuf>,
    pub registered_snippets: usize,
    pub registered_locales: Vec<(String, PathBuf)>,
    pub keybindings: KeybindingConversionReport,
}

/// Resolves a relative asset path declared inside `package.json` while preventing
/// escape outside `extension_dir`.
fn resolve_extension_asset_path(extension_dir: &Path, relative: &str) -> Result<PathBuf> {
    let trimmed = relative.trim_start_matches("./").trim_start_matches(".\\");
    let clean = archive::sanitize_zip_entry_path(trimmed)?;
    Ok(extension_dir.join(clean))
}

/// Inspects a local `.vsix` file, safely extracts it into `install_root/<extension_id>`,
/// converts all static resources (themes, iconThemes, snippets, keybindings, localizations),
/// and registers snippets & locale packs into `proxy` when provided.
///
/// Static-only `.vsix` packages require zero resident processes and zero Node.js runtime.
pub async fn install_local_vsix_static(
    vsix_path: &Path,
    install_root: &Path,
    proxy: Option<&ExtensionHostProxy>,
) -> Result<VsixStaticInstallReport> {
    let staging_dir = install_root.join(".staging-vsix");
    if staging_dir.exists() {
        let _ = std::fs::remove_dir_all(&staging_dir);
    }

    let extracted = extract_vsix_safely(vsix_path, &staging_dir).await?;
    let manifest_path = extracted.extension_dir.join("package.json");
    let manifest_raw = std::fs::read_to_string(&manifest_path)
        .with_context(|| format!("failed to read {}", manifest_path.display()))?;
    let manifest: VsCodeManifest = serde_json_lenient::from_str(&manifest_raw)
        .with_context(|| format!("failed to parse {}", manifest_path.display()))?;

    let extension_id = manifest.extension_id();
    let safe_dir_name = extension_id
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>();
    let final_dir = install_root.join(&safe_dir_name);
    if final_dir.exists() {
        std::fs::remove_dir_all(&final_dir)
            .with_context(|| format!("failed to replace existing {}", final_dir.display()))?;
    }
    std::fs::rename(&staging_dir, &final_dir)
        .with_context(|| format!("failed to move staging dir to {}", final_dir.display()))?;

    let extension_dir = if final_dir.join("extension").join("package.json").is_file() {
        final_dir.join("extension")
    } else {
        final_dir.clone()
    };

    let gram_generated_dir = final_dir.join("gram_converted");
    std::fs::create_dir_all(&gram_generated_dir)?;

    // 1. Themes
    let mut generated_themes = Vec::new();
    for (idx, theme_contrib) in manifest.contributes.themes.iter().enumerate() {
        let src = resolve_extension_asset_path(&extension_dir, &theme_contrib.path)?;
        let family = convert_vscode_theme_file(&manifest.publisher, theme_contrib, &src)?;
        let out_path = gram_generated_dir.join(format!("theme_{idx}.json"));
        let json = serde_json::to_string_pretty(&family)?;
        std::fs::write(&out_path, json)?;
        generated_themes.push(out_path);
    }

    // 2. Icon Themes
    let mut generated_icon_themes = Vec::new();
    for (idx, icon_contrib) in manifest.contributes.icon_themes.iter().enumerate() {
        let src = resolve_extension_asset_path(&extension_dir, &icon_contrib.path)?;
        let family = convert_vscode_icon_theme_file(&manifest.publisher, icon_contrib, &src)?;
        let out_path = gram_generated_dir.join(format!("icon_theme_{idx}.json"));
        let json = serde_json::to_string_pretty(&family)?;
        std::fs::write(&out_path, json)?;
        generated_icon_themes.push(out_path);
    }

    // 3. Snippets
    let mut registered_snippets = 0;
    for snippet_contrib in &manifest.contributes.snippets {
        let src = resolve_extension_asset_path(&extension_dir, &snippet_contrib.path)?;
        let content = std::fs::read_to_string(&src)
            .with_context(|| format!("failed to read snippet {}", src.display()))?;
        if let Some(proxy) = proxy {
            proxy.register_snippet(&src, &content)?;
        }
        registered_snippets += 1;
    }

    // 4. Keybindings
    let keybindings = convert_vscode_keybindings(&manifest.contributes.keybindings);
    if !keybindings.converted.is_empty() {
        let out_path = gram_generated_dir.join("keybindings.json");
        std::fs::write(&out_path, serde_json::to_string_pretty(&keybindings.converted)?)?;
    }

    // 5. Localizations (lazy file-backed registration: 0 RAM for inactive locales)
    let mut registered_locales = Vec::new();
    for loc_contrib in &manifest.contributes.localizations {
        let locale_id = i18n::LocaleId::new(&loc_contrib.language_id);
        for trans in &loc_contrib.translations {
            let src = resolve_extension_asset_path(&extension_dir, &trans.path)?;
            if let Some(proxy) = proxy {
                proxy.register_locale_pack(
                    Arc::from(extension_id.as_str()),
                    Arc::from(locale_id.as_str()),
                    src.clone(),
                )?;
            }
            registered_locales.push((locale_id.to_string(), src));
        }
    }

    let required_capabilities = extract_required_capabilities(&manifest);

    Ok(VsixStaticInstallReport {
        extension_id,
        version: manifest.version.clone(),
        is_static_only: manifest.is_static_only(),
        install_dir: final_dir,
        required_capabilities,
        generated_themes,
        generated_icon_themes,
        registered_snippets,
        registered_locales,
        keybindings,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_zip::ZipEntryBuilder;
    use async_zip::base::write::ZipFileWriter;
    use extension::ExtensionSnippetProxy;
    use std::sync::Mutex;

    async fn build_test_vsix(path: &Path, files: &[(&str, &str)]) {
        use futures::AsyncWriteExt as _;
        let mut file = smol::fs::File::create(path).await.unwrap();
        let mut writer = ZipFileWriter::new(&mut file);
        for (name, content) in files {
            let builder = ZipEntryBuilder::new((*name).into(), async_zip::Compression::Deflate);
            writer
                .write_entry_whole(builder, content.as_bytes())
                .await
                .unwrap();
        }
        writer.close().await.unwrap();
        file.flush().await.unwrap();
        file.close().await.unwrap();
    }

    struct RecordingSnippetProxy(Arc<Mutex<Vec<(PathBuf, String)>>>);

    impl ExtensionSnippetProxy for RecordingSnippetProxy {
        fn register_snippet(&self, path: &Path, snippet_contents: &str) -> Result<()> {
            self.0
                .lock()
                .unwrap()
                .push((path.to_path_buf(), snippet_contents.to_string()));
            Ok(())
        }
    }

    struct RecordingLocaleProxy(Arc<i18n::LocaleRegistry>);

    impl ExtensionLocaleProxy for RecordingLocaleProxy {
        fn register_locale_pack(
            &self,
            extension_id: Arc<str>,
            locale: Arc<str>,
            locale_path: PathBuf,
        ) -> Result<()> {
            self.0
                .register_locale_pack_file(extension_id, i18n::LocaleId::new(locale.as_ref()), locale_path)
        }

        fn unregister_locale_pack(&self, extension_id: &str) {
            self.0.unregister_locale_pack(extension_id);
        }
    }

    #[test]
    fn rejects_path_traversal_entries() {
        smol::block_on(async {
            let tmp = tempfile::tempdir().unwrap();
            let vsix = tmp.path().join("malicious.vsix");
            build_test_vsix(&vsix, &[("../escape.txt", "boom")]).await;

            let err = extract_vsix_safely(&vsix, &tmp.path().join("out"))
                .await
                .unwrap_err();
            assert!(
                err.to_string().contains("path traversal"),
                "unexpected error: {err}"
            );
        });
    }

    #[test]
    fn rejects_oversized_vsix_archive() {
        smol::block_on(async {
            let tmp = tempfile::tempdir().unwrap();
            let vsix = tmp.path().join("big.vsix");
            build_test_vsix(&vsix, &[("extension/package.json", &"a".repeat(2048))]).await;

            let err = archive::extract_vsix_with_limits(&vsix, &tmp.path().join("out"), 10, 512)
                .await
                .unwrap_err();
            assert!(
                err.to_string().contains("maximum uncompressed size"),
                "unexpected error: {err}"
            );
        });
    }

    #[test]
    fn installs_static_vsix_with_all_five_resource_types() {
        smol::block_on(async {
            let tmp = tempfile::tempdir().unwrap();
            let vsix = tmp.path().join("sample-pack.vsix");

            let package_json = r#"{
                "name": "sample-static-pack",
                "publisher": "gram-test",
                "version": "1.0.0",
                "contributes": {
                    "themes": [
                        { "label": "Sample Dark", "uiTheme": "vs-dark", "path": "./themes/dark.json" }
                    ],
                    "iconThemes": [
                        { "id": "sample-icons", "label": "Sample Icons", "path": "./icons/theme.json" }
                    ],
                    "snippets": [
                        { "language": "rust", "path": "./snippets/rust.json" }
                    ],
                    "keybindings": [
                        { "command": "workbench.action.files.save", "key": "ctrl+s", "when": "editorTextFocus" },
                        { "command": "unknown.extension.noop", "key": "ctrl+alt+z" }
                    ],
                    "localizations": [
                        {
                            "languageId": "ja",
                            "translations": [
                                { "id": "gram", "path": "./l10n/ja.json" }
                            ]
                        }
                    ]
                }
            }"#;

            let dark_theme = r##"{
                "name": "Sample Dark",
                "colors": {
                    "editor.background": "#1e1e1e",
                    "editor.foreground": "#d4d4d4"
                },
                "tokenColors": [
                    { "scope": "comment", "settings": { "foreground": "#6a9955", "fontStyle": "italic" } },
                    { "scope": "keyword", "settings": { "foreground": "#569cd6", "fontStyle": "bold" } }
                ]
            }"##;

            let icon_theme = r#"{
                "iconDefinitions": {
                    "_file_rs": { "iconPath": "./rust.svg" },
                    "_folder": { "iconPath": "./folder.svg" }
                },
                "folder": "_folder",
                "fileExtensions": { "rs": "_file_rs" }
            }"#;

            let rust_snippet = r#"{
                "Println": { "prefix": "pln", "body": ["println!(\"$1\");"] }
            }"#;

            let ja_l10n = r#"{
                "command_palette.no_matches": "該当コマンドなし（VSIX言語パック）"
            }"#;

            build_test_vsix(
                &vsix,
                &[
                    ("extension/package.json", package_json),
                    ("extension/themes/dark.json", dark_theme),
                    ("extension/icons/theme.json", icon_theme),
                    ("extension/snippets/rust.json", rust_snippet),
                    ("extension/l10n/ja.json", ja_l10n),
                ],
            )
            .await;

            let proxy = ExtensionHostProxy::new();
            let snippets_recorded = Arc::new(Mutex::new(Vec::new()));
            proxy.register_snippet_proxy(RecordingSnippetProxy(snippets_recorded.clone()));
            let locale_registry = Arc::new(i18n::LocaleRegistry::default());
            proxy.register_locale_proxy(RecordingLocaleProxy(locale_registry.clone()));

            let install_root = tmp.path().join("installed");
            let report = install_local_vsix_static(&vsix, &install_root, Some(&proxy))
                .await
                .unwrap();

            assert_eq!(report.extension_id, "gram-test.sample-static-pack");
            assert!(report.is_static_only);
            assert!(report.required_capabilities.is_empty());
            assert_eq!(report.generated_themes.len(), 1);
            assert_eq!(report.generated_icon_themes.len(), 1);
            assert_eq!(report.registered_snippets, 1);
            assert_eq!(report.registered_locales.len(), 1);
            assert_eq!(report.keybindings.converted.len(), 1);
            assert_eq!(report.keybindings.converted[0].gram_action, "workspace::Save");
            assert_eq!(report.keybindings.skipped_commands, vec!["unknown.extension.noop"]);

            // Verify theme JSON deserializes cleanly as Gram ThemeFamilyContent
            let theme_json = std::fs::read_to_string(&report.generated_themes[0]).unwrap();
            let parsed_theme: theme::ThemeFamilyContent = serde_json::from_str(&theme_json).unwrap();
            assert_eq!(parsed_theme.name, "Sample Dark");
            assert_eq!(parsed_theme.themes[0].style.colors.editor_background.as_deref(), Some("#1e1e1e"));

            // Verify icon theme JSON deserializes cleanly as Gram IconThemeFamilyContent
            let icon_json = std::fs::read_to_string(&report.generated_icon_themes[0]).unwrap();
            let parsed_icon: theme::IconThemeFamilyContent = serde_json::from_str(&icon_json).unwrap();
            assert_eq!(parsed_icon.themes[0].file_suffixes.get("rs").map(String::as_str), Some("_file_rs"));

            // Verify snippet proxy received the snippet
            assert_eq!(snippets_recorded.lock().unwrap().len(), 1);

            // Verify locale pack immediately took effect on active `ja` locale
            assert_eq!(
                locale_registry.translate("command_palette.no_matches"),
                "該当コマンドなし（VSIX言語パック）"
            );
        });
    }

    #[test]
    fn dynamic_vsix_reports_required_capabilities() {
        let manifest: VsCodeManifest = serde_json::from_str(
            r#"{
                "name": "dynamic-ext",
                "publisher": "acme",
                "version": "0.1.0",
                "main": "./out/extension.js",
                "activationEvents": ["onCommand:acme.hello"],
                "contributes": {
                    "commands": [{ "command": "acme.hello", "title": "Hello" }]
                }
            }"#,
        )
        .unwrap();

        assert!(!manifest.is_static_only());
        let caps = extract_required_capabilities(&manifest);
        assert!(caps.iter().any(|c| c.kind == "runtime:nodejs"));
        assert!(caps.iter().any(|c| c.kind == "command:register"));
        assert!(caps.iter().any(|c| c.kind == "workspace:read"));
    }
}
