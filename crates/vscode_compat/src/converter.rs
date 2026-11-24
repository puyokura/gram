use std::collections::HashMap;
use std::path::Path;

use anyhow::{Context as _, Result};
use collections::IndexMap;
use gpui::SharedString;
use serde::{Deserialize, Serialize};
use theme::{
    AppearanceContent, DirectoryIconsContent, FontStyleContent, FontWeightContent, HighlightStyleContent,
    IconDefinitionContent, IconThemeContent, IconThemeFamilyContent, StatusColorsContent, ThemeColorsContent,
    ThemeContent, ThemeFamilyContent, ThemeStyleContent, WindowBackgroundContent,
};
use vscode_theme::Colors;

use crate::manifest::{VsCodeIconThemeContribution, VsCodeKeybindingContribution, VsCodeThemeContribution};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawVsCodeTheme {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    colors: Option<Colors>,
    #[serde(default)]
    token_colors: Vec<RawTokenColor>,
}

#[derive(Debug, Deserialize)]
struct RawTokenColor {
    #[serde(default)]
    scope: Option<TokenScope>,
    #[serde(default)]
    settings: RawTokenSettings,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum TokenScope {
    Single(String),
    Many(Vec<String>),
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawTokenSettings {
    #[serde(default)]
    foreground: Option<String>,
    #[serde(default)]
    font_style: Option<String>,
}

pub fn appearance_from_ui_theme(ui_theme: &str) -> AppearanceContent {
    match ui_theme {
        "vs" | "hc-light" => AppearanceContent::Light,
        _ => AppearanceContent::Dark,
    }
}

/// Converts a VS Code theme JSON file into a Gram [`ThemeFamilyContent`].
pub fn convert_vscode_theme_file(
    extension_author: &str,
    contrib: &VsCodeThemeContribution,
    theme_path: &Path,
) -> Result<ThemeFamilyContent> {
    let raw = std::fs::read_to_string(theme_path)
        .with_context(|| format!("failed to read VS Code theme {}", theme_path.display()))?;
    let parsed: RawVsCodeTheme =
        serde_json_lenient::from_str(&raw).with_context(|| format!("failed to parse VS Code theme {}", theme_path.display()))?;

    let appearance = appearance_from_ui_theme(&contrib.ui_theme);
    let theme_name = if !contrib.label.trim().is_empty() {
        contrib.label.clone()
    } else {
        parsed.name.unwrap_or_else(|| "Imported Theme".to_string())
    };

    let (theme_colors, status) = if let Some(c) = &parsed.colors {
        let colors = ThemeColorsContent {
            background: c.editor.background.clone().or_else(|| c.side_bar.background.clone()),
            editor_background: c.editor.background.clone(),
            editor_foreground: c.editor.foreground.clone().or_else(|| c.foreground.clone()),
            editor_gutter_background: c.editor.background.clone(),
            editor_active_line_background: c.editor.line_highlight_background.clone(),
            editor_line_number: c.editor_line_number.foreground.clone(),
            editor_active_line_number: c
                .editor_line_number
                .active_foreground
                .clone()
                .or_else(|| c.editor.foreground.clone()),
            text: c.foreground.clone().or_else(|| c.editor.foreground.clone()),
            text_muted: c
                .description_foreground
                .clone()
                .or_else(|| c.tab.inactive_foreground.clone()),
            border: c.panel.border.clone().or_else(|| c.side_bar.border.clone()),
            surface_background: c.side_bar.background.clone(),
            elevated_surface_background: c.dropdown.background.clone(),
            panel_background: c.panel.background.clone().or_else(|| c.side_bar.background.clone()),
            status_bar_background: c.status_bar.background.clone(),
            title_bar_background: c.title_bar.active_background.clone(),
            tab_bar_background: c.editor_group_header.tabs_background.clone(),
            tab_active_background: c.tab.active_background.clone(),
            tab_inactive_background: c.tab.inactive_background.clone(),
            terminal_background: c.terminal.background.clone().or_else(|| c.editor.background.clone()),
            terminal_foreground: c.terminal.foreground.clone().or_else(|| c.editor.foreground.clone()),
            terminal_ansi_black: c.terminal.ansi_black.clone(),
            terminal_ansi_red: c.terminal.ansi_red.clone(),
            terminal_ansi_green: c.terminal.ansi_green.clone(),
            terminal_ansi_yellow: c.terminal.ansi_yellow.clone(),
            terminal_ansi_blue: c.terminal.ansi_blue.clone(),
            terminal_ansi_magenta: c.terminal.ansi_magenta.clone(),
            terminal_ansi_cyan: c.terminal.ansi_cyan.clone(),
            terminal_ansi_white: c.terminal.ansi_white.clone(),
            ..Default::default()
        };

        let status = StatusColorsContent {
            error: c.editor_error.foreground.clone(),
            warning: c.editor_warning.foreground.clone(),
            info: c.editor_info.foreground.clone(),
            hint: c.editor_hint.foreground.clone(),
            created: c.editor_gutter.added_background.clone(),
            modified: c.editor_gutter.modified_background.clone(),
            deleted: c.editor_gutter.deleted_background.clone(),
            conflict: c.git_decoration.conflicting_resource_foreground.clone(),
            ..Default::default()
        };

        (colors, status)
    } else {
        (ThemeColorsContent::default(), StatusColorsContent::default())
    };

    let mut syntax: IndexMap<String, HighlightStyleContent> = IndexMap::default();
    for rule in &parsed.token_colors {
        let scopes: Vec<&str> = match &rule.scope {
            Some(TokenScope::Single(s)) => s.split(',').map(str::trim).collect(),
            Some(TokenScope::Many(list)) => list.iter().map(String::as_str).collect(),
            None => continue,
        };
        let font_style = rule.settings.font_style.as_deref().and_then(|fs| {
            if fs.contains("italic") {
                Some(FontStyleContent::Italic)
            } else if fs.contains("oblique") {
                Some(FontStyleContent::Oblique)
            } else {
                None
            }
        });
        let font_weight = rule.settings.font_style.as_deref().and_then(|fs| {
            if fs.contains("bold") {
                Some(FontWeightContent::Bold)
            } else {
                None
            }
        });
        let style = HighlightStyleContent {
            color: rule.settings.foreground.clone(),
            font_style,
            font_weight,
            ..Default::default()
        };
        for scope in scopes {
            if let Some(gram_token) = map_textmate_scope_to_gram(scope) {
                syntax.entry(gram_token.to_string()).or_insert_with(|| style.clone());
            }
        }
    }

    Ok(ThemeFamilyContent {
        name: theme_name.clone(),
        author: if extension_author.is_empty() {
            "VS Code Extension".to_string()
        } else {
            extension_author.to_string()
        },
        themes: vec![ThemeContent {
            name: theme_name,
            appearance,
            style: ThemeStyleContent {
                window_background_appearance: Some(WindowBackgroundContent::Opaque),
                accents: Vec::new(),
                colors: theme_colors,
                status,
                players: Vec::new(),
                syntax,
            },
        }],
    })
}

fn map_textmate_scope_to_gram(scope: &str) -> Option<&'static str> {
    if scope.starts_with("comment") {
        Some("comment")
    } else if scope.starts_with("string") {
        Some("string")
    } else if scope.starts_with("constant.numeric") {
        Some("number")
    } else if scope.starts_with("constant.language") || scope.starts_with("constant.character") {
        Some("constant")
    } else if scope.starts_with("keyword") || scope.starts_with("storage") {
        Some("keyword")
    } else if scope.starts_with("entity.name.function") || scope.starts_with("support.function") {
        Some("function")
    } else if scope.starts_with("entity.name.type")
        || scope.starts_with("entity.name.class")
        || scope.starts_with("support.type")
    {
        Some("type")
    } else if scope.starts_with("variable") {
        Some("variable")
    } else if scope.starts_with("punctuation") {
        Some("punctuation")
    } else if scope.starts_with("entity.name.tag") {
        Some("tag")
    } else if scope.starts_with("entity.other.attribute-name") {
        Some("attribute")
    } else {
        None
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawVsCodeIconTheme {
    #[serde(default)]
    icon_definitions: HashMap<String, RawIconDefinition>,
    #[serde(default)]
    folder: Option<String>,
    #[serde(default)]
    folder_expanded: Option<String>,
    #[serde(default)]
    folder_names: HashMap<String, String>,
    #[serde(default)]
    folder_names_expanded: HashMap<String, String>,
    #[serde(default)]
    file_extensions: HashMap<String, String>,
    #[serde(default)]
    file_names: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawIconDefinition {
    #[serde(default)]
    icon_path: Option<String>,
}

/// Converts a VS Code icon theme JSON file into a Gram [`IconThemeFamilyContent`].
/// Font-based icon definitions (without `iconPath`) are safely skipped.
pub fn convert_vscode_icon_theme_file(
    extension_author: &str,
    contrib: &VsCodeIconThemeContribution,
    icon_theme_path: &Path,
) -> Result<IconThemeFamilyContent> {
    let raw = std::fs::read_to_string(icon_theme_path)
        .with_context(|| format!("failed to read VS Code icon theme {}", icon_theme_path.display()))?;
    let parsed: RawVsCodeIconTheme = serde_json_lenient::from_str(&raw)
        .with_context(|| format!("failed to parse VS Code icon theme {}", icon_theme_path.display()))?;

    let mut file_icons = HashMap::new();
    for (id, def) in &parsed.icon_definitions {
        if let Some(path) = &def.icon_path {
            file_icons.insert(
                id.clone(),
                IconDefinitionContent {
                    path: SharedString::from(path.clone()),
                },
            );
        }
    }

    let resolve_icon_path = |icon_id: &Option<String>| -> Option<SharedString> {
        icon_id
            .as_ref()
            .and_then(|id| parsed.icon_definitions.get(id))
            .and_then(|def| def.icon_path.clone())
            .map(SharedString::from)
    };

    let directory_icons = DirectoryIconsContent {
        collapsed: resolve_icon_path(&parsed.folder),
        expanded: resolve_icon_path(&parsed.folder_expanded),
    };

    let mut named_directory_icons: HashMap<String, DirectoryIconsContent> = HashMap::new();
    for (dir_name, icon_id) in &parsed.folder_names {
        let collapsed = resolve_icon_path(&Some(icon_id.clone()));
        let expanded = parsed
            .folder_names_expanded
            .get(dir_name)
            .and_then(|id| resolve_icon_path(&Some(id.clone())));
        if collapsed.is_some() || expanded.is_some() {
            named_directory_icons.insert(dir_name.clone(), DirectoryIconsContent { collapsed, expanded });
        }
    }

    let file_suffixes = parsed
        .file_extensions
        .into_iter()
        .filter(|(_, icon_id)| file_icons.contains_key(icon_id))
        .collect();

    let file_stems = parsed
        .file_names
        .into_iter()
        .filter(|(_, icon_id)| file_icons.contains_key(icon_id))
        .collect();

    Ok(IconThemeFamilyContent {
        name: contrib.label.clone(),
        author: if extension_author.is_empty() {
            "VS Code Extension".to_string()
        } else {
            extension_author.to_string()
        },
        themes: vec![IconThemeContent {
            name: contrib.label.clone(),
            appearance: AppearanceContent::Dark,
            directory_icons,
            named_directory_icons,
            chevron_icons: Default::default(),
            file_stems,
            file_suffixes,
            file_icons,
        }],
    })
}

/// Translated Gram keybinding entry produced from a VS Code `contributes.keybindings` item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConvertedKeybinding {
    pub keystroke: String,
    pub gram_action: String,
    pub context: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct KeybindingConversionReport {
    pub converted: Vec<ConvertedKeybinding>,
    pub skipped_commands: Vec<String>,
}

/// Translates VS Code keybindings into Gram actions for known commands.
/// Unknown commands are recorded in `skipped_commands` without failing or breaking existing keymaps.
pub fn convert_vscode_keybindings(items: &[VsCodeKeybindingContribution]) -> KeybindingConversionReport {
    let mut report = KeybindingConversionReport::default();
    for item in items {
        let Some(gram_action) = map_vscode_command_to_gram(&item.command) else {
            report.skipped_commands.push(item.command.clone());
            continue;
        };
        let raw_key = select_platform_key(item);
        let keystroke = normalize_vscode_keystroke(raw_key);
        let context = item.when.as_deref().and_then(map_vscode_when_to_context);
        report.converted.push(ConvertedKeybinding {
            keystroke,
            gram_action: gram_action.to_string(),
            context,
        });
    }
    report
}

fn select_platform_key(item: &VsCodeKeybindingContribution) -> &str {
    #[cfg(target_os = "macos")]
    if let Some(mac) = &item.mac {
        return mac;
    }
    #[cfg(target_os = "windows")]
    if let Some(win) = &item.win {
        return win;
    }
    #[cfg(target_os = "linux")]
    if let Some(linux) = &item.linux {
        return linux;
    }
    &item.key
}

fn normalize_vscode_keystroke(raw: &str) -> String {
    raw.split_whitespace()
        .map(|chord| {
            chord
                .split('+')
                .map(|part| match part.to_ascii_lowercase().as_str() {
                    "cmd" | "meta" | "super" => "cmd",
                    "ctrl" | "control" => "ctrl",
                    "alt" | "option" => "alt",
                    "shift" => "shift",
                    _ => part,
                })
                .collect::<Vec<_>>()
                .join("-")
                .to_ascii_lowercase()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn map_vscode_when_to_context(when: &str) -> Option<String> {
    if when.contains("editorTextFocus") {
        Some("Editor".to_string())
    } else if when.contains("terminalFocus") {
        Some("Terminal".to_string())
    } else {
        None
    }
}

fn map_vscode_command_to_gram(command: &str) -> Option<&'static str> {
    match command {
        "workbench.action.files.save" => Some("workspace::Save"),
        "workbench.action.files.saveAs" => Some("workspace::SaveAs"),
        "workbench.action.files.saveAll" => Some("workspace::SaveAll"),
        "workbench.action.quickOpen" => Some("file_finder::Toggle"),
        "workbench.action.showCommands" => Some("command_palette::Toggle"),
        "workbench.action.terminal.toggleTerminal" => Some("terminal_panel::ToggleFocus"),
        "workbench.action.splitEditor" => Some("pane::SplitRight"),
        "workbench.action.closeActiveEditor" => Some("pane::CloseActiveItem"),
        "editor.action.commentLine" => Some("editor::ToggleComments"),
        "editor.action.formatDocument" => Some("editor::Format"),
        "editor.action.revealDefinition" => Some("editor::GoToDefinition"),
        "editor.action.rename" => Some("editor::Rename"),
        "actions.find" => Some("buffer_search::Deploy"),
        "workbench.action.findInFiles" => Some("project_search::ToggleFocus"),
        "workbench.action.gotoLine" => Some("go_to_line::Toggle"),
        "workbench.action.gotoSymbol" => Some("outline::Toggle"),
        _ => None,
    }
}
