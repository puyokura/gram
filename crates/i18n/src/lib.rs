mod actions;
mod extension_locale;

use std::borrow::Cow;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context as _, Result};
use gpui::{App, Global, ReadGlobal};
use parking_lot::RwLock;

pub use actions::ActionLocalization;
pub use extension_locale::init;

/// Normalized BCP-47 locale identifier (lowercase, e.g. `"ja"`, `"en"`, `"zh-cn"`).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LocaleId(Arc<str>);

impl LocaleId {
    pub fn new(raw: &str) -> Self {
        let normalized = raw.trim().replace('_', "-").to_ascii_lowercase();
        if normalized.is_empty() {
            Self::ja()
        } else {
            Self(Arc::from(normalized))
        }
    }

    pub fn ja() -> Self {
        Self(Arc::from("ja"))
    }

    pub fn en() -> Self {
        Self(Arc::from("en"))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the primary language subtag (e.g. `"ja"` for `"ja-jp"`).
    pub fn primary_language(&self) -> &str {
        self.0.split('-').next().unwrap_or(&self.0)
    }

    /// Returns true if a pack targeting `pack_locale` applies to this active locale.
    pub fn matches_pack(&self, pack_locale: &LocaleId) -> bool {
        self.0 == pack_locale.0 || self.primary_language() == pack_locale.0.as_ref()
    }
}

impl Default for LocaleId {
    fn default() -> Self {
        Self::ja()
    }
}

impl From<&str> for LocaleId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for LocaleId {
    fn from(value: String) -> Self {
        Self::new(&value)
    }
}

impl std::fmt::Display for LocaleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Clone)]
enum LocalePackSource {
    /// Lazy on-disk JSON file: NOT parsed or kept in RAM unless its locale is active.
    File(PathBuf),
    /// Direct in-memory map (used for tests or dynamically constructed packs).
    Memory(Arc<HashMap<Arc<str>, Arc<str>>>),
}

#[derive(Clone)]
struct RegisteredLocalePack {
    extension_id: Arc<str>,
    locale: LocaleId,
    source: LocalePackSource,
}

struct LocaleState {
    active_locale: LocaleId,
    /// Built-in fallback English strings (always kept for fallback).
    fallback_en: HashMap<Arc<str>, Arc<str>>,
    /// Built-in strings for `active_locale` (empty when `active_locale` is `en`).
    active_builtin: HashMap<Arc<str>, Arc<str>>,
    /// Merged plugin overrides for `active_locale` ONLY.
    active_overrides: HashMap<Arc<str>, Arc<str>>,
    /// Registered language pack descriptors (non-active packs stay unloaded on disk).
    packs: Vec<RegisteredLocalePack>,
}

impl LocaleState {
    fn new(initial_locale: LocaleId) -> Self {
        let fallback_en = builtin_strings("en");
        let active_builtin = if initial_locale.primary_language() == "en" {
            HashMap::new()
        } else {
            builtin_strings(initial_locale.primary_language())
        };
        Self {
            active_locale: initial_locale,
            fallback_en,
            active_builtin,
            active_overrides: HashMap::new(),
            packs: Vec::new(),
        }
    }

    fn reload_active_overrides(&mut self) -> Result<()> {
        self.active_overrides.clear();
        let active = self.active_locale.clone();
        for pack in &self.packs {
            if !active.matches_pack(&pack.locale) {
                continue;
            }
            match &pack.source {
                LocalePackSource::File(path) => {
                    let parsed = load_locale_json_file(path)?;
                    for (key, value) in parsed {
                        self.active_overrides.insert(Arc::from(key), Arc::from(value));
                    }
                }
                LocalePackSource::Memory(entries) => {
                    for (key, value) in entries.iter() {
                        self.active_overrides.insert(key.clone(), value.clone());
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Default)]
struct GlobalLocaleRegistry(Arc<LocaleRegistry>);

impl Global for GlobalLocaleRegistry {}

/// Central localization registry.
///
/// Design invariant: only the active locale (+ English fallback) is expanded in memory.
/// Unselected language packs remain as lightweight path descriptors on disk.
pub struct LocaleRegistry {
    state: RwLock<LocaleState>,
}

impl Default for LocaleRegistry {
    fn default() -> Self {
        Self::new(LocaleId::ja())
    }
}

impl LocaleRegistry {
    pub fn new(initial_locale: LocaleId) -> Self {
        Self {
            state: RwLock::new(LocaleState::new(initial_locale)),
        }
    }

    pub fn global(cx: &App) -> Arc<Self> {
        GlobalLocaleRegistry::global(cx).0.clone()
    }

    pub fn try_global(cx: &App) -> Option<Arc<Self>> {
        cx.try_global::<GlobalLocaleRegistry>().map(|g| g.0.clone())
    }

    pub fn default_global(cx: &mut App) -> Arc<Self> {
        cx.default_global::<GlobalLocaleRegistry>().0.clone()
    }

    pub fn active_locale(&self) -> LocaleId {
        self.state.read().active_locale.clone()
    }

    /// Switches the active UI locale without rebuilding.
    ///
    /// Drops the previous active locale's dictionaries and loads only the newly
    /// selected locale (+ retains `en` fallback).
    pub fn set_locale(&self, locale: impl Into<LocaleId>) -> Result<()> {
        let locale = locale.into();
        let mut state = self.state.write();
        if state.active_locale == locale {
            return Ok(());
        }
        state.active_locale = locale.clone();
        state.active_builtin = if locale.primary_language() == "en" {
            HashMap::new()
        } else {
            builtin_strings(locale.primary_language())
        };
        state.reload_active_overrides()?;
        Ok(())
    }

    /// Registers a file-backed language pack from an extension.
    ///
    /// If `locale` does not match the active locale, the file is NOT parsed into RAM.
    pub fn register_locale_pack_file(
        &self,
        extension_id: Arc<str>,
        locale: LocaleId,
        path: PathBuf,
    ) -> Result<()> {
        let mut state = self.state.write();
        state
            .packs
            .retain(|p| !(p.extension_id == extension_id && p.locale == locale));
        let matches_active = state.active_locale.matches_pack(&locale);
        state.packs.push(RegisteredLocalePack {
            extension_id,
            locale,
            source: LocalePackSource::File(path),
        });
        if matches_active {
            state.reload_active_overrides()?;
        }
        Ok(())
    }

    /// Registers an in-memory language pack (primarily for tests or generated packs).
    pub fn register_locale_pack_entries(
        &self,
        extension_id: Arc<str>,
        locale: LocaleId,
        entries: HashMap<String, String>,
    ) {
        let mapped: HashMap<Arc<str>, Arc<str>> = entries
            .into_iter()
            .map(|(k, v)| (Arc::from(k), Arc::from(v)))
            .collect();
        let mut state = self.state.write();
        state
            .packs
            .retain(|p| !(p.extension_id == extension_id && p.locale == locale));
        let matches_active = state.active_locale.matches_pack(&locale);
        state.packs.push(RegisteredLocalePack {
            extension_id,
            locale,
            source: LocalePackSource::Memory(Arc::new(mapped)),
        });
        if matches_active {
            let _ = state.reload_active_overrides();
        }
    }

    /// Removes all locale packs contributed by `extension_id`.
    pub fn unregister_locale_pack(&self, extension_id: &str) {
        let mut state = self.state.write();
        let before = state.packs.len();
        state.packs.retain(|p| p.extension_id.as_ref() != extension_id);
        if state.packs.len() != before {
            let _ = state.reload_active_overrides();
        }
    }

    /// Translates `key` using the resolution order:
    /// 1. active locale plugin override
    /// 2. active locale built-in string
    /// 3. fallback `en` built-in string
    /// 4. `key` itself
    pub fn translate<'a>(&self, key: &'a str) -> Cow<'a, str> {
        let state = self.state.read();
        if let Some(val) = state.active_overrides.get(key) {
            return Cow::Owned(val.to_string());
        }
        if let Some(val) = state.active_builtin.get(key) {
            return Cow::Owned(val.to_string());
        }
        if let Some(val) = state.fallback_en.get(key) {
            return Cow::Owned(val.to_string());
        }
        Cow::Borrowed(key)
    }

    /// Translates `key`, falling back to `default` if neither active locale nor `en` has it.
    pub fn translate_or<'a>(&self, key: &str, default: &'a str) -> Cow<'a, str> {
        let state = self.state.read();
        if let Some(val) = state.active_overrides.get(key) {
            return Cow::Owned(val.to_string());
        }
        if let Some(val) = state.active_builtin.get(key) {
            return Cow::Owned(val.to_string());
        }
        if let Some(val) = state.fallback_en.get(key) {
            return Cow::Owned(val.to_string());
        }
        Cow::Borrowed(default)
    }

    /// Returns localized action metadata for Command Palette display & bilingual search.
    ///
    /// Language packs can override any action via keys:
    /// - `action.<action_id>.label`
    /// - `action.<action_id>.description`
    /// - `action.<action_id>.keywords` (comma-separated)
    pub fn action_localization(&self, action_id: &str) -> Option<ActionLocalization> {
        let state = self.state.read();
        let label_key = format!("action.{action_id}.label");
        let desc_key = format!("action.{action_id}.description");
        let kw_key = format!("action.{action_id}.keywords");

        let mut base = actions::builtin_action_localization(action_id, state.active_locale.as_str());

        let override_label = state.active_overrides.get(label_key.as_str()).cloned();
        let override_desc = state.active_overrides.get(desc_key.as_str()).cloned();
        let override_kw = state.active_overrides.get(kw_key.as_str()).map(|raw| {
            raw.split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(Arc::from)
                .collect::<Vec<Arc<str>>>()
        });

        match (&mut base, override_label, override_desc, override_kw) {
            (Some(loc), label, desc, kw) => {
                if let Some(label) = label {
                    loc.label = label;
                }
                if let Some(desc) = desc {
                    loc.description = Some(desc);
                }
                if let Some(kw) = kw {
                    loc.keywords = kw;
                }
                base
            }
            (None, Some(label), desc, kw) => Some(ActionLocalization {
                label,
                description: desc,
                keywords: kw.unwrap_or_default(),
            }),
            (None, None, Some(desc), kw) => Some(ActionLocalization {
                label: desc.clone(),
                description: Some(desc),
                keywords: kw.unwrap_or_default(),
            }),
            (None, None, None, _) => None,
        }
    }

    /// Returns the number of locale override entries currently expanded in memory.
    pub fn active_override_count(&self) -> usize {
        self.state.read().active_overrides.len()
    }

    /// Returns the number of distinct locales currently expanded in memory (at most 2: active + `en`).
    pub fn loaded_locale_count(&self) -> usize {
        let state = self.state.read();
        if state.active_locale.primary_language() == "en" {
            1
        } else {
            2
        }
    }
}

/// Parses a locale pack JSON file.
///
/// Supports both flat key-value JSON (`{ "key": "value" }`) and VS Code language pack
/// format (`{ "contents": { "section": { "key": "value" } } }`).
pub fn load_locale_json_file(path: &Path) -> Result<HashMap<String, String>> {
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read locale file {}", path.display()))?;
    parse_locale_json(&raw)
}

pub fn parse_locale_json(raw: &str) -> Result<HashMap<String, String>> {
    let value: serde_json::Value =
        serde_json_lenient::from_str(raw).context("failed to parse locale JSON")?;
    let mut out = HashMap::new();
    match value {
        serde_json::Value::Object(map) => {
            if let Some(serde_json::Value::Object(contents)) = map.get("contents") {
                for (section, section_val) in contents {
                    if let serde_json::Value::Object(entries) = section_val {
                        for (k, v) in entries {
                            if let Some(s) = v.as_str() {
                                out.insert(format!("{section}.{k}"), s.to_string());
                            }
                        }
                    }
                }
            } else {
                flatten_json_object("", &map, &mut out);
            }
        }
        _ => anyhow::bail!("locale JSON root must be an object"),
    }
    Ok(out)
}

fn flatten_json_object(
    prefix: &str,
    map: &serde_json::Map<String, serde_json::Value>,
    out: &mut HashMap<String, String>,
) {
    for (k, v) in map {
        let full_key = if prefix.is_empty() {
            k.clone()
        } else {
            format!("{prefix}.{k}")
        };
        match v {
            serde_json::Value::String(s) => {
                out.insert(full_key, s.clone());
            }
            serde_json::Value::Object(nested) => {
                flatten_json_object(&full_key, nested, out);
            }
            _ => {}
        }
    }
}

fn builtin_strings(primary_lang: &str) -> HashMap<Arc<str>, Arc<str>> {
    let entries: &[(&str, &str)] = match primary_lang {
        "ja" => &[
            ("command_palette.placeholder", "コマンドを実行（英語・日本語で検索）…"),
            ("command_palette.no_matches", "一致するコマンドがありません"),
            ("command_palette.change_keybinding", "キーバインドを変更…"),
            ("command_palette.add_keybinding", "キーバインドを追加…"),
            ("command_palette.run_action", "実行"),
            ("extensions.install_local_vsix", "ローカル .vsix から導入…"),
            ("extensions.capabilities_prompt", "このプラグインが要求する権限"),
        ],
        "en" => &[
            ("command_palette.placeholder", "Execute a command…"),
            ("command_palette.no_matches", "No matching commands"),
            ("command_palette.change_keybinding", "Change Keybinding…"),
            ("command_palette.add_keybinding", "Add Keybinding…"),
            ("command_palette.run_action", "Run"),
            ("extensions.install_local_vsix", "Install from Local .vsix…"),
            ("extensions.capabilities_prompt", "Permissions requested by this plugin"),
        ],
        _ => &[],
    };
    entries
        .iter()
        .map(|(k, v)| (Arc::from(*k), Arc::from(*v)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_locale_is_ja_with_en_fallback() {
        let registry = LocaleRegistry::default();
        assert_eq!(registry.active_locale(), LocaleId::ja());
        assert_eq!(registry.loaded_locale_count(), 2);
        assert_eq!(
            registry.translate("command_palette.no_matches"),
            "一致するコマンドがありません"
        );
        // Unknown key falls back to key itself.
        assert_eq!(registry.translate("unknown.key"), "unknown.key");
    }

    #[test]
    fn switching_locale_drops_previous_and_keeps_only_active_plus_en() {
        let registry = LocaleRegistry::default();
        registry.set_locale("en").unwrap();
        assert_eq!(registry.active_locale(), LocaleId::en());
        assert_eq!(registry.loaded_locale_count(), 1);
        assert_eq!(
            registry.translate("command_palette.no_matches"),
            "No matching commands"
        );

        // Switching to a locale without built-in strings falls back to `en`.
        registry.set_locale("fr").unwrap();
        assert_eq!(registry.loaded_locale_count(), 2);
        assert_eq!(
            registry.translate("command_palette.no_matches"),
            "No matching commands"
        );
    }

    #[test]
    fn inactive_locale_pack_file_is_not_parsed_until_selected() {
        let dir = tempfile::tempdir().unwrap();
        // Deliberately invalid JSON for an inactive locale `de`:
        // registering it while `ja` is active must NOT parse or load it!
        let de_invalid = dir.path().join("de.json");
        std::fs::write(&de_invalid, "{ not valid json").unwrap();

        let fr_valid = dir.path().join("fr.json");
        std::fs::write(
            &fr_valid,
            r#"{ "command_palette": { "no_matches": "Aucune commande correspondante" } }"#,
        )
        .unwrap();

        let registry = LocaleRegistry::default();
        registry
            .register_locale_pack_file(Arc::from("pack-de"), LocaleId::new("de"), de_invalid)
            .unwrap();
        registry
            .register_locale_pack_file(Arc::from("pack-fr"), LocaleId::new("fr"), fr_valid)
            .unwrap();
        assert_eq!(registry.active_override_count(), 0);

        // Selecting `fr` parses only `fr.json`.
        registry.set_locale("fr").unwrap();
        assert_eq!(registry.active_override_count(), 1);
        assert_eq!(
            registry.translate("command_palette.no_matches"),
            "Aucune commande correspondante"
        );

        // Unregistering clears overrides and restores `en` fallback.
        registry.unregister_locale_pack("pack-fr");
        assert_eq!(registry.active_override_count(), 0);
        assert_eq!(
            registry.translate("command_palette.no_matches"),
            "No matching commands"
        );
    }

    #[test]
    fn vscode_contents_format_and_action_override() {
        let parsed = parse_locale_json(
            r#"{
                "contents": {
                    "action.workspace::Save": {
                        "label": "カスタム保存",
                        "description": "プラグインで上書きされた説明"
                    }
                }
            }"#,
        )
        .unwrap();
        assert_eq!(
            parsed.get("action.workspace::Save.label").map(String::as_str),
            Some("カスタム保存")
        );

        let registry = LocaleRegistry::default();
        let builtin = registry.action_localization("workspace::Save").unwrap();
        assert_eq!(builtin.label.as_ref(), "ファイルを保存");

        registry.register_locale_pack_entries(Arc::from("custom-ja"), LocaleId::ja(), parsed);
        let overridden = registry.action_localization("workspace::Save").unwrap();
        assert_eq!(overridden.label.as_ref(), "カスタム保存");
        assert_eq!(
            overridden.description.as_deref(),
            Some("プラグインで上書きされた説明")
        );
    }
}
