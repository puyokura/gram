use gpui::{Action as _, App};
use settings::{LanguageSettingsContent, SettingsContent};
use std::sync::Arc;
use strum::IntoDiscriminant as _;
use ui::{IntoElement, SharedString};

use crate::{
    ActionLink, DynamicItem, PROJECT, SettingField, SettingItem, SettingsFieldMetadata, SettingsPage, SettingsPageItem,
    SubPageLink, USER, all_language_names, sub_page_stack,
};

const DEFAULT_SHARED_STRING: SharedString = SharedString::new_static("");
/// A default empty string reference. Useful in `pick` functions for cases either in dynamic item fields, or when dealing with `settings::Maybe`
/// to avoid the "NO DEFAULT" case.
const DEFAULT_EMPTY_SHARED_STRING: Option<&SharedString> = Some(&DEFAULT_SHARED_STRING);

pub(crate) fn settings_data(cx: &App) -> Vec<SettingsPage> {
    vec![
        SettingsPage {
            title: "一般",
            items: vec![
                SettingsPageItem::SectionHeader("一般設定"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "タブがない状態での終了時の動作",
                    description: "タブが開いていない状態で「アクティブな項目を閉じる」を実行したときの動作を設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("when_closing_with_no_tabs"),
                        pick: |settings_content| {
                            settings_content
                                .workspace
                                .when_closing_with_no_tabs
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.workspace.when_closing_with_no_tabs = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "最後のウィンドウを閉じたときの動作",
                    description: "最後のウィンドウを閉じたときの動作を設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("on_last_window_closed"),
                        pick: |settings_content| {
                            settings_content.workspace.on_last_window_closed.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.workspace.on_last_window_closed = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "システム標準のパスプロンプトを使用",
                    description: "「開く」や「名前を付けて保存」にOS標準のダイアログを使用します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("use_system_path_prompts"),
                        pick: |settings_content| {
                            settings_content.workspace.use_system_path_prompts.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.workspace.use_system_path_prompts = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "システム標準のプロンプトを使用",
                    description: "確認ダイアログにOS標準のダイアログを使用します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("use_system_prompts"),
                        pick: |settings_content| {
                            settings_content.workspace.use_system_prompts.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.workspace.use_system_prompts = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "プライベートな値を伏字化",
                    description: "プライベートファイル内の変数の値を非表示にします。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("redact_private_values"),
                        pick: |settings_content| {
                            settings_content.editor.redact_private_values.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.editor.redact_private_values = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "プライベートファイル",
                    description: "ファイルがプライベートであるかを判定するためのパス照合用Globパターンを指定します。",
                    aliases: None,
                    field: Box::new(
                        SettingField {
                            json_path: Some("worktree.private_files"),
                            pick: |settings_content| {
                                settings_content.project.worktree.private_files.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.project.worktree.private_files = value;
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("ワークスペースの復元"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "未保存バッファーの復元",
                    description: "再起動時に未保存のバッファーを復元するかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("session.restore_unsaved_buffers"),
                        pick: |settings_content| {
                            settings_content
                                .session
                                .as_ref()
                                .and_then(|session| session.restore_unsaved_buffers.as_ref())
                        },
                        write: |settings_content, value| {
                            settings_content
                                .session
                                .get_or_insert_default()
                                .restore_unsaved_buffers = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "起動時の復元",
                    description: "Gramの起動時に前回のセッションから何を復元するかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("restore_on_startup"),
                        pick: |settings_content| {
                            settings_content.workspace.restore_on_startup.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.workspace.restore_on_startup = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("プロファイル"),
                SettingsPageItem::SettingItem(SettingItem {
                    files: USER,
                    title: "設定プロファイル",
                    description: "既存のユーザー設定の上に一時的に適用する設定プロファイルを指定します。",
                    aliases: None,
                    field: Box::new(
                        SettingField {
                            json_path: Some("settings_profiles"),
                            pick: |settings_content| {
                                Some(settings_content)
                            },
                            write: |_settings_content, _value| {
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                }),
            ],
        },
        SettingsPage {
            title: "外観",
            items: vec![
                SettingsPageItem::SectionHeader("テーマ"),
                SettingsPageItem::DynamicItem(DynamicItem {
                    discriminant: SettingItem {
                        files: USER,
                        title: "テーマモード",
                        description: "固定テーマを使用するか、外観やライト/ダークモードに応じて動的に選択するかを設定します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("theme$"),
                            pick: |settings_content| {
                                Some(&dynamic_variants::<settings::ThemeSelection>()[
                                    settings_content
                                        .theme
                                        .theme
                                        .as_ref()?
                                        .discriminant() as usize])
                            },
                            write: |settings_content, value| {
                                let Some(value) = value else {
                                    settings_content.theme.theme = None;
                                    return;
                                };
                                let settings_value = settings_content.theme.theme.get_or_insert_with(|| {
                                    settings::ThemeSelection::Static(theme::ThemeName(theme::default_theme(theme::SystemAppearance::default().0).into()))
                                });
                                *settings_value = match value {
                                    settings::ThemeSelectionDiscriminants::Static => {
                                        let name = match settings_value {
                                            settings::ThemeSelection::Static(_) => return,
                                            settings::ThemeSelection::Dynamic { mode, light, dark } => {
                                                match mode {
                                                    theme::ThemeAppearanceMode::Light => light.clone(),
                                                    theme::ThemeAppearanceMode::Dark => dark.clone(),
                                                    theme::ThemeAppearanceMode::System => dark.clone(), // no cx, can't determine correct choice
                                                }
                                            },
                                        };
                                        settings::ThemeSelection::Static(name)
                                    },
                                    settings::ThemeSelectionDiscriminants::Dynamic => {
                                        let static_name = match settings_value {
                                            settings::ThemeSelection::Static(theme_name) => theme_name.clone(),
                                            settings::ThemeSelection::Dynamic {..} => return,
                                        };

                                        settings::ThemeSelection::Dynamic {
                                            mode: settings::ThemeAppearanceMode::System,
                                            light: static_name.clone(),
                                            dark: static_name,
                                        }
                                    },
                                };
                            },
                        }),
                        metadata: None,
                    },
                    pick_discriminant: |settings_content| {
                        Some(settings_content.theme.theme.as_ref()?.discriminant() as usize)
                    },
                    fields: dynamic_variants::<settings::ThemeSelection>().iter().map(|variant| {
                        match variant {
                            settings::ThemeSelectionDiscriminants::Static => vec![
                                SettingItem {
                                    files: USER,
                                    title: "テーマ名",
                                    description: "選択中のテーマ名です。",
                                    aliases: None,
                                    field: Box::new(SettingField {
                                        json_path: Some("theme"),
                                        pick: |settings_content| {
                                            match settings_content.theme.theme.as_ref() {
                                                Some(settings::ThemeSelection::Static(name)) => Some(name),
                                                _ => None
                                            }
                                        },
                                        write: |settings_content, value| {
                                            let Some(value) = value else {
                                                return;
                                            };
                                            if let Some(settings::ThemeSelection::Static(theme_name)) = settings_content
                                                .theme
                                                .theme.as_mut() { *theme_name = value }
                                        },
                                    }),
                                    metadata: None,
                                }
                            ],
                            settings::ThemeSelectionDiscriminants::Dynamic => vec![
                                SettingItem {
                                    files: USER,
                                    title: "モード",
                                    description: "選択したライト/ダークテーマを使用するか、OSの外観設定に従うかを設定します。",
                                    aliases: None,
                                    field: Box::new(SettingField {
                                        json_path: Some("theme.mode"),
                                        pick: |settings_content| {
                                            match settings_content.theme.theme.as_ref() {
                                                Some(settings::ThemeSelection::Dynamic { mode, ..}) => Some(mode),
                                                _ => None
                                            }
                                        },
                                        write: |settings_content, value| {
                                            let Some(value) = value else {
                                                return;
                                            };
                                            if let Some(settings::ThemeSelection::Dynamic{ mode, ..}) = settings_content
                                                .theme
                                                .theme.as_mut() { *mode = value }
                                        },
                                    }),
                                    metadata: None,
                                },
                                SettingItem {
                                    files: USER,
                                    title: "ライトテーマ",
                                    description: "モードがライトのとき、またはシステム連動のライトモード時に使用するテーマです。",
                                    aliases: None,
                                    field: Box::new(SettingField {
                                        json_path: Some("theme.light"),
                                        pick: |settings_content| {
                                            match settings_content.theme.theme.as_ref() {
                                                Some(settings::ThemeSelection::Dynamic { light, ..}) => Some(light),
                                                _ => None
                                            }
                                        },
                                        write: |settings_content, value| {
                                            let Some(value) = value else {
                                                return;
                                            };
                                            if let Some(settings::ThemeSelection::Dynamic{ light, ..}) = settings_content
                                                .theme
                                                .theme.as_mut() { *light = value }
                                        },
                                    }),
                                    metadata: None,
                                },
                                SettingItem {
                                    files: USER,
                                    title: "ダークテーマ",
                                    description: "モードがダークのとき、またはシステム連動のダークモード時に使用するテーマです。",
                                    aliases: None,
                                    field: Box::new(SettingField {
                                        json_path: Some("theme.dark"),
                                        pick: |settings_content| {
                                            match settings_content.theme.theme.as_ref() {
                                                Some(settings::ThemeSelection::Dynamic { dark, ..}) => Some(dark),
                                                _ => None
                                            }
                                        },
                                        write: |settings_content, value| {
                                            let Some(value) = value else {
                                                return;
                                            };
                                            if let Some(settings::ThemeSelection::Dynamic{ dark, ..}) = settings_content
                                                .theme
                                                .theme.as_mut() { *dark = value }
                                        },
                                    }),
                                    metadata: None,
                                }
                            ],
                        }
                    }).collect(),
                }),
                SettingsPageItem::DynamicItem(DynamicItem {
                    discriminant: SettingItem {
                        files: USER,
                        title: "アイコンテーマ",
                        description: "ファイルやフォルダーに関連付けるカスタムアイコンセットです。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("icon_theme$"),
                            pick: |settings_content| {
                                Some(&dynamic_variants::<settings::IconThemeSelection>()[
                                    settings_content
                                        .theme
                                        .icon_theme
                                        .as_ref()?
                                        .discriminant() as usize])
                            },
                            write: |settings_content, value| {
                                let Some(value) = value else {
                                    settings_content.theme.icon_theme = None;
                                    return;
                                };
                                let settings_value = settings_content.theme.icon_theme.get_or_insert_with(|| {
                                    settings::IconThemeSelection::Static(settings::IconThemeName(theme::default_icon_theme().name.clone().into()))
                                });
                                *settings_value = match value {
                                    settings::IconThemeSelectionDiscriminants::Static => {
                                        let name = match settings_value {
                                            settings::IconThemeSelection::Static(_) => return,
                                            settings::IconThemeSelection::Dynamic { mode, light, dark } => {
                                                match mode {
                                                    theme::ThemeAppearanceMode::Light => light.clone(),
                                                    theme::ThemeAppearanceMode::Dark => dark.clone(),
                                                    theme::ThemeAppearanceMode::System => dark.clone(), // no cx, can't determine correct choice
                                                }
                                            },
                                        };
                                        settings::IconThemeSelection::Static(name)
                                    },
                                    settings::IconThemeSelectionDiscriminants::Dynamic => {
                                        let static_name = match settings_value {
                                            settings::IconThemeSelection::Static(theme_name) => theme_name.clone(),
                                            settings::IconThemeSelection::Dynamic {..} => return,
                                        };

                                        settings::IconThemeSelection::Dynamic {
                                            mode: settings::ThemeAppearanceMode::System,
                                            light: static_name.clone(),
                                            dark: static_name,
                                        }
                                    },
                                };
                            },
                        }),
                        metadata: None,
                    },
                    pick_discriminant: |settings_content| {
                        Some(settings_content.theme.icon_theme.as_ref()?.discriminant() as usize)
                    },
                    fields: dynamic_variants::<settings::IconThemeSelection>().iter().map(|variant| {
                        match variant {
                            settings::IconThemeSelectionDiscriminants::Static => vec![
                                SettingItem {
                                    files: USER,
                                    title: "アイコンテーマ名",
                                    description: "選択中のアイコンテーマ名です。",
                                    aliases: None,
                                    field: Box::new(SettingField {
                                        json_path: Some("icon_theme$string"),
                                        pick: |settings_content| {
                                            match settings_content.theme.icon_theme.as_ref() {
                                                Some(settings::IconThemeSelection::Static(name)) => Some(name),
                                                _ => None
                                            }
                                        },
                                        write: |settings_content, value| {
                                            let Some(value) = value else {
                                                return;
                                            };
                                            if let Some(settings::IconThemeSelection::Static(theme_name)) = settings_content
                                                .theme
                                                .icon_theme.as_mut() { *theme_name = value }
                                        },
                                    }),
                                    metadata: None,
                                }
                            ],
                            settings::IconThemeSelectionDiscriminants::Dynamic => vec![
                                SettingItem {
                                    files: USER,
                                    title: "モード",
                                    description: "選択したライト/ダークアイコンテーマを使用するか、OSの外観設定に従うかを設定します。",
                                    aliases: None,
                                    field: Box::new(SettingField {
                                        json_path: Some("icon_theme"),
                                        pick: |settings_content| {
                                            match settings_content.theme.icon_theme.as_ref() {
                                                Some(settings::IconThemeSelection::Dynamic { mode, ..}) => Some(mode),
                                                _ => None
                                            }
                                        },
                                        write: |settings_content, value| {
                                            let Some(value) = value else {
                                                return;
                                            };
                                            if let Some(settings::IconThemeSelection::Dynamic{ mode, ..}) = settings_content
                                                .theme
                                                .icon_theme.as_mut() { *mode = value }
                                        },
                                    }),
                                    metadata: None,
                                },
                                SettingItem {
                                    files: USER,
                                    title: "ライトアイコンテーマ",
                                    description: "モードがライトのとき、またはシステム連動のライトモード時に使用するアイコンテーマです。",
                                    aliases: None,
                                    field: Box::new(SettingField {
                                        json_path: Some("icon_theme.light"),
                                        pick: |settings_content| {
                                            match settings_content.theme.icon_theme.as_ref() {
                                                Some(settings::IconThemeSelection::Dynamic { light, ..}) => Some(light),
                                                _ => None
                                            }
                                        },
                                        write: |settings_content, value| {
                                            let Some(value) = value else {
                                                return;
                                            };
                                            if let Some(settings::IconThemeSelection::Dynamic{ light, ..}) = settings_content
                                                .theme
                                                .icon_theme.as_mut() { *light = value }
                                        },
                                    }),
                                    metadata: None,
                                },
                                SettingItem {
                                    files: USER,
                                    title: "ダークアイコンテーマ",
                                    description: "モードがダークのとき、またはシステム連動のダークモード時に使用するアイコンテーマです。",
                                    aliases: None,
                                    field: Box::new(SettingField {
                                        json_path: Some("icon_theme.dark"),
                                        pick: |settings_content| {
                                            match settings_content.theme.icon_theme.as_ref() {
                                                Some(settings::IconThemeSelection::Dynamic { dark, ..}) => Some(dark),
                                                _ => None
                                            }
                                        },
                                        write: |settings_content, value| {
                                            let Some(value) = value else {
                                                return;
                                            };
                                            if let Some(settings::IconThemeSelection::Dynamic{ dark, ..}) = settings_content
                                                .theme
                                                .icon_theme.as_mut() { *dark = value }
                                        },
                                    }),
                                    metadata: None,
                                }
                            ],
                        }
                    }).collect(),
                }),
                SettingsPageItem::SectionHeader("バッファーフォント"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "フォントファミリー",
                    description: "エディターテキストのフォントファミリーです。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("buffer_font_family"),
                        pick: |settings_content| settings_content.theme.buffer_font_family.as_ref(),
                        write: |settings_content, value|{  settings_content.theme.buffer_font_family = value;},
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "フォントサイズ",
                    description: "エディターテキストのフォントサイズです。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("buffer_font_size"),
                        pick: |settings_content| settings_content.theme.buffer_font_size.as_ref(),
                        write: |settings_content, value|{  settings_content.theme.buffer_font_size = value;},
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "フォントウェイト",
                    description: "エディターテキストのフォントの太さ（100-900）です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("buffer_font_weight"),
                        pick: |settings_content| settings_content.theme.buffer_font_weight.as_ref(),
                        write: |settings_content, value|{  settings_content.theme.buffer_font_weight = value;},
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::DynamicItem(DynamicItem {
                    discriminant: SettingItem {
                        files: USER,
                        title: "行の高さ",
                        description: "エディターテキストの行の高さです。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("buffer_line_height$"),
                            pick: |settings_content| {
                                Some(&dynamic_variants::<settings::BufferLineHeight>()[
                                    settings_content
                                        .theme
                                        .buffer_line_height
                                        .as_ref()?
                                        .discriminant() as usize])
                            },
                            write: |settings_content, value| {
                                let Some(value) = value else {
                                    settings_content.theme.buffer_line_height = None;
                                    return;
                                };
                                let settings_value = settings_content.theme.buffer_line_height.get_or_insert_with(|| {
                                    settings::BufferLineHeight::default()
                                });
                                *settings_value = match value {
                                    settings::BufferLineHeightDiscriminants::Comfortable => {
                                        settings::BufferLineHeight::Comfortable
                                    },
                                    settings::BufferLineHeightDiscriminants::Standard => {
                                        settings::BufferLineHeight::Standard
                                    },
                                    settings::BufferLineHeightDiscriminants::Custom => {
                                        let custom_value = theme::BufferLineHeight::from(*settings_value).value();
                                        settings::BufferLineHeight::Custom(custom_value)
                                    },
                                };
                            },
                        }),
                        metadata: None,
                    },
                    pick_discriminant: |settings_content| {
                        Some(settings_content.theme.buffer_line_height.as_ref()?.discriminant() as usize)
                    },
                    fields: dynamic_variants::<settings::BufferLineHeight>().iter().map(|variant| {
                        match variant {
                            settings::BufferLineHeightDiscriminants::Comfortable => vec![],
                            settings::BufferLineHeightDiscriminants::Standard => vec![],
                            settings::BufferLineHeightDiscriminants::Custom => vec![
                                SettingItem {
                                    files: USER,
                                    title: "カスタムの行の高さ",
                                    description: "行の高さのカスタム値（1.0以上）です。",
                                    aliases: None,
                                    field: Box::new(SettingField {
                                        json_path: Some("buffer_line_height"),
                                        pick: |settings_content| {
                                            match settings_content.theme.buffer_line_height.as_ref() {
                                                Some(settings::BufferLineHeight::Custom(value)) => Some(value),
                                                _ => None
                                            }
                                        },
                                        write: |settings_content, value| {
                                            let Some(value) = value else {
                                                return;
                                            };
                                            if let Some(settings::BufferLineHeight::Custom(line_height)) = settings_content
                                                .theme
                                                .buffer_line_height.as_mut() { *line_height = f32::max(value, 1.0) }
                                        },
                                    }),
                                    metadata: None,
                                }
                            ],
                        }
                    }).collect(),
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    files: USER,
                    title: "フォント機能",
                    description: "テキストバッファーの描画で有効にするOpenType機能です。",
                    aliases: None,
                    field: Box::new(
                        SettingField {
                            json_path: Some("buffer_font_features"),
                            pick: |settings_content| {
                                settings_content.theme.buffer_font_features.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.theme.buffer_font_features = value;

                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    files: USER,
                    title: "フォールバックフォント",
                    description: "テキストバッファーの描画で使用するフォールバックフォントです。",
                    aliases: None,
                    field: Box::new(
                        SettingField {
                            json_path: Some("buffer_font_fallbacks"),
                            pick: |settings_content| {
                                settings_content.theme.buffer_font_fallbacks.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.theme.buffer_font_fallbacks = value;

                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("UIフォント"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "フォントファミリー",
                    description: "UI要素のフォントファミリーです。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("ui_font_family"),
                        pick: |settings_content| settings_content.theme.ui_font_family.as_ref(),
                        write: |settings_content, value|{  settings_content.theme.ui_font_family = value;},
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "フォントサイズ",
                    description: "UI要素のフォントサイズです。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("ui_font_size"),
                        pick: |settings_content| settings_content.theme.ui_font_size.as_ref(),
                        write: |settings_content, value|{  settings_content.theme.ui_font_size = value;},
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "フォントウェイト",
                    description: "UI要素のフォントの太さ（100-900）です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("ui_font_weight"),
                        pick: |settings_content| settings_content.theme.ui_font_weight.as_ref(),
                        write: |settings_content, value|{  settings_content.theme.ui_font_weight = value;},
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    files: USER,
                    title: "フォント機能",
                    description: "UI要素の描画で有効にするOpenType機能です。",
                    aliases: None,
                    field: Box::new(
                        SettingField {
                            json_path: Some("ui_font_features"),
                            pick: |settings_content| {
                                settings_content.theme.ui_font_features.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.theme.ui_font_features = value;

                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    files: USER,
                    title: "フォールバックフォント",
                    description: "UIの描画で使用するフォールバックフォントです。",
                    aliases: None,
                    field: Box::new(
                        SettingField {
                            json_path: Some("ui_font_fallbacks"),
                            pick: |settings_content| {
                                settings_content.theme.ui_font_fallbacks.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.theme.ui_font_fallbacks = value;

                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("テキスト描画"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "テキスト描画モード",
                    description: "使用するテキスト描画モードです。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("text_rendering_mode"),
                        pick: |settings_content| {
                            settings_content.workspace.text_rendering_mode.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.workspace.text_rendering_mode = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("カーソル"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "マルチカーソル修飾キー",
                    description: "複数カーソルを追加するための修飾キーです。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("multi_cursor_modifier"),
                        pick: |settings_content| {
                            settings_content.editor.multi_cursor_modifier.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.editor.multi_cursor_modifier = value;

                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "カーソルの点滅",
                    description: "エディター内でカーソルを点滅させるかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("cursor_blink"),
                        pick: |settings_content| settings_content.editor.cursor_blink.as_ref(),
                        write: |settings_content, value|{  settings_content.editor.cursor_blink = value;},
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "カーソル形状",
                    description: "エディターのカーソル形状です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("cursor_shape"),
                        pick: |settings_content| settings_content.editor.cursor_shape.as_ref(),
                        write: |settings_content, value|{  settings_content.editor.cursor_shape = value;},
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "スムーズカーソル",
                    description: "エディター内を移動する際にカーソルを滑らかにアニメーションさせるかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("smooth_cursor.enabled"),
                        pick: |settings_content| {
                            settings_content
                                .editor
                                .smooth_cursor
                                .as_ref()?
                                .enabled
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .editor
                                .smooth_cursor
                                .get_or_insert_default()
                                .enabled = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "スムーズカーソルの所要時間",
                    description: "カーソル移動アニメーションの所要時間（ミリ秒）です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("smooth_cursor.duration_ms"),
                        pick: |settings_content| {
                            settings_content
                                .editor
                                .smooth_cursor
                                .as_ref()?
                                .duration_ms
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .editor
                                .smooth_cursor
                                .get_or_insert_default()
                                .duration_ms = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "マウスの非表示",
                    description: "マウスカーソルを非表示にするタイミングを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("hide_mouse"),
                        pick: |settings_content| settings_content.editor.hide_mouse.as_ref(),
                        write: |settings_content, value|{  settings_content.editor.hide_mouse = value;},
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("ハイライト"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "不要なコードのフェード",
                    description: "未使用のコードをどの程度薄く表示するか（0.0 - 0.9）を設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("unnecessary_code_fade"),
                        pick: |settings_content| {
                            settings_content.theme.unnecessary_code_fade.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.theme.unnecessary_code_fade = value;

                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "現在の行のハイライト",
                    description: "カーソルがある行をハイライトする方法を設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("current_line_highlight"),
                        pick: |settings_content| {
                            settings_content.editor.current_line_highlight.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.editor.current_line_highlight = value;

                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "選択テキストのハイライト",
                    description: "選択したテキストと一致するすべての箇所をハイライトします。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("selection_highlight"),
                        pick: |settings_content| {
                            settings_content.editor.selection_highlight.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.editor.selection_highlight = value;

                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "角丸の選択範囲",
                    description: "テキスト選択範囲の角を丸くするかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("rounded_selection"),
                        pick: |settings_content| settings_content.editor.rounded_selection.as_ref(),
                        write: |settings_content, value|{  settings_content.editor.rounded_selection = value;},
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "ハイライトの最小コントラスト",
                    description: "ハイライト背景の上にテキストを描画する際に維持するAPCA知覚コントラストの最小値です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("minimum_contrast_for_highlights"),
                        pick: |settings_content| {
                            settings_content
                                .editor
                                .minimum_contrast_for_highlights
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.editor.minimum_contrast_for_highlights = value;

                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("ガイド"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "折り返しガイドを表示",
                    description: "折り返しガイド（垂直ルーラー）を表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("show_wrap_guides"),
                        pick: |settings_content| {
                            settings_content
                                .project
                                .all_languages
                                .defaults
                                .show_wrap_guides
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content

                                .project
                                .all_languages
                                .defaults
                                .show_wrap_guides = value;
                        },
                    }),
                    metadata: None,
                    files: USER | PROJECT,
                }),
                // todo(settings_ui): This needs a custom component
                SettingsPageItem::SettingItem(SettingItem {
                    title: "折り返しガイド",
                    description: "折り返しガイドを表示する文字数列を指定します。",
                    aliases: None,
                    field: Box::new(
                        SettingField {
                            json_path: Some("wrap_guides"),
                            pick: |settings_content| {
                                settings_content
                                    .project
                                    .all_languages
                                    .defaults
                                    .wrap_guides
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.project.all_languages.defaults.wrap_guides = value;
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                    files: USER | PROJECT,
                }),
                SettingsPageItem::SectionHeader("装飾"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "クライアントサイド装飾の角丸",
                    description: "クライアントサイド装飾を使用するプラットフォームでのウィンドウ枠の角丸半径を設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("client_side_decoration_rounding"),
                        pick: |settings_content| {
                            settings_content.theme.client_side_decoration_rounding.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.theme.client_side_decoration_rounding = value;

                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "クライアントサイド装飾の影",
                    description: "クライアントサイド装飾を使用するプラットフォームでのウィンドウの影の大きさを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("client_side_decoration_shadow"),
                        pick: |settings_content| {
                            settings_content.theme.client_side_decoration_shadow.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.theme.client_side_decoration_shadow = value;

                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
            ],
        },
        SettingsPage {
            title: "キーマップ",
            items: vec![
                SettingsPageItem::SectionHeader("キーバインド"),
                SettingsPageItem::ActionLink(ActionLink {
                    title: "キーバインドの編集".into(),
                    description: Some("キーマップエディターでキーバインドをカスタマイズします。".into()),
                    button_text: "キーマップを開く".into(),
                    on_click: Arc::new(|settings_window, window, cx| {
                        let Some(original_window) = settings_window.original_window else {
                            return;
                        };
                        original_window
                            .update(cx, |_workspace, original_window, cx| {
                                original_window
                                    .dispatch_action(app_actions::OpenKeymap.boxed_clone(), cx);
                                original_window.activate_window();
                            })
                            .ok();
                        window.remove_window();
                    }),
                }),
                SettingsPageItem::SectionHeader("ベースキーマップ"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "ベースキーマップ",
                    description: "基本セットとして使用するキーバインドの名称です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("base_keymap"),
                        pick: |settings_content| settings_content.base_keymap.as_ref(),
                        write: |settings_content, value| {
                            settings_content.base_keymap = value;
                        },
                    }),
                    metadata: Some(Box::new(SettingsFieldMetadata {
                        should_do_titlecase: Some(false),
                        ..Default::default()
                    })),
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("モーダル編集"),
                // todo(settings_ui): Vim/Helix Mode should be apart of one type because it's undefined
                // behavior to have them both enabled at the same time
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Vimモード",
                    description: "Vimモードとキーバインドを有効にします。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("vim_mode"),
                        pick: |settings_content| settings_content.vim_mode.as_ref(),
                        write: |settings_content, value| {
                            settings_content.vim_mode = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Helixモード",
                    description: "Helixモードとキーバインドを有効にします。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("helix_mode"),
                        pick: |settings_content| settings_content.helix_mode.as_ref(),
                        write: |settings_content, value| {
                            settings_content.helix_mode = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
            ],
        },
        SettingsPage {
            title: "エディター",
            items: {
                let mut items = vec![
                    SettingsPageItem::SectionHeader("自動保存"),
                    SettingsPageItem::DynamicItem(DynamicItem {
                        discriminant: SettingItem {
                            files: USER,
                            title: "自動保存モード",
                            description: "バッファーの変更を自動保存するタイミングを設定します。",
                            aliases: None,
                            field: Box::new(SettingField {
                                json_path: Some("autosave$"),
                                pick: |settings_content| {
                                    Some(&dynamic_variants::<settings::AutosaveSetting>()[
                                        settings_content
                                            .workspace
                                            .autosave
                                            .as_ref()?
                                            .discriminant() as usize])
                                },
                                write: |settings_content, value| {
                                    let Some(value) = value else {
                                        settings_content.workspace.autosave = None;
                                        return;
                                    };
                                    let settings_value = settings_content.workspace.autosave.get_or_insert({
                                        settings::AutosaveSetting::Off
                                    });
                                    *settings_value = match value {
                                        settings::AutosaveSettingDiscriminants::Off => {
                                            settings::AutosaveSetting::Off
                                        },
                                        settings::AutosaveSettingDiscriminants::AfterDelay => {
                                            let milliseconds = match settings_value {
                                                settings::AutosaveSetting::AfterDelay { milliseconds } => *milliseconds,
                                                _ => settings::DelayMs(1000),
                                            };
                                            settings::AutosaveSetting::AfterDelay { milliseconds }
                                        },
                                        settings::AutosaveSettingDiscriminants::OnFocusChange => {
                                            settings::AutosaveSetting::OnFocusChange
                                        },
                                        settings::AutosaveSettingDiscriminants::OnWindowChange => {
                                            settings::AutosaveSetting::OnWindowChange
                                        },
                                    };
                                },
                            }),
                            metadata: None,
                        },
                        pick_discriminant: |settings_content| {
                            Some(settings_content.workspace.autosave.as_ref()?.discriminant() as usize)
                        },
                        fields: dynamic_variants::<settings::AutosaveSetting>().iter().map(|variant| {
                            match variant {
                                settings::AutosaveSettingDiscriminants::Off => vec![],
                                settings::AutosaveSettingDiscriminants::AfterDelay => vec![
                                    SettingItem {
                                        files: USER,
                                        title: "遅延時間（ミリ秒）",
                                        description: "操作が行われなくなってから保存するまでの待機時間（ミリ秒）です。",
                                        aliases: None,
                                        field: Box::new(SettingField {
                                            json_path: Some("autosave.after_delay.milliseconds"),
                                            pick: |settings_content| {
                                                match settings_content.workspace.autosave.as_ref() {
                                                    Some(settings::AutosaveSetting::AfterDelay { milliseconds }) => Some(milliseconds),
                                                    _ => None
                                                }
                                            },
                                            write: |settings_content, value| {
                                                let Some(value) = value else {
                                                    settings_content.workspace.autosave = None;
                                                    return;
                                                };
                                                if let Some(settings::AutosaveSetting::AfterDelay { milliseconds }) = settings_content
                                                    .workspace
                                                    .autosave.as_mut() { *milliseconds = value }
                                            },
                                        }),
                                        metadata: None,
                                    }
                                ],
                                settings::AutosaveSettingDiscriminants::OnFocusChange => vec![],
                                settings::AutosaveSettingDiscriminants::OnWindowChange => vec![],
                            }
                        }).collect(),
                    }),
                    SettingsPageItem::SectionHeader("Which-Key"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Which-Key",
                        description: "連続キー入力の途中で一致するキーバインドの一覧メニューを表示します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("which_key.enabled"),
                            pick: |settings_content| {
                                settings_content
                                    .which_key
                                    .as_ref()
                                    .and_then(|settings| settings.enabled.as_ref())
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .which_key
                                    .get_or_insert_default()
                                    .enabled = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "メニュー表示遅延",
                        description: "Which-Keyメニューが表示されるまでの遅延時間（ミリ秒）です。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("which_key.delay_ms"),
                            pick: |settings_content| {
                                settings_content
                                    .which_key
                                    .as_ref()
                                    .and_then(|settings| settings.delay_ms.as_ref())
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .which_key
                                    .get_or_insert_default()
                                    .delay_ms = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SectionHeader("マルチバッファー"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "マルチバッファーのダブルクリック動作",
                        description: "マルチバッファーの抜粋部分をダブルクリックしたときの動作を設定します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("double_click_in_multibuffer"),
                            pick: |settings_content| {
                                settings_content.editor.double_click_in_multibuffer.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.double_click_in_multibuffer = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "抜粋の展開行数",
                        description: "マルチバッファーの抜粋を展開するデフォルトの行数です。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("expand_excerpt_lines"),
                            pick: |settings_content| {
                                settings_content.editor.expand_excerpt_lines.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.expand_excerpt_lines = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "抜粋のコンテキスト行数",
                        description: "マルチバッファーの抜粋でデフォルトで表示する前後のコンテキスト行数です。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("excerpt_context_lines"),
                            pick: |settings_content| {
                                settings_content.editor.excerpt_context_lines.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.excerpt_context_lines = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "アウトライン展開の深さ",
                        description: "現在のファイルのアウトライン項目をデフォルトで展開する深さです。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("outline_panel.expand_outlines_with_depth"),
                            pick: |settings_content| {
                                settings_content
                                    .outline_panel
                                    .as_ref()
                                    .and_then(|outline_panel| {
                                        outline_panel.expand_outlines_with_depth.as_ref()
                                    })
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .outline_panel
                                    .get_or_insert_default()
                                    .expand_outlines_with_depth = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SectionHeader("スクロール"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "最終行を超えてスクロール",
                        description: "エディターで最終行を超えてスクロールできるかどうかを設定します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("scroll_beyond_last_line"),
                            pick: |settings_content| {
                                settings_content.editor.scroll_beyond_last_line.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.scroll_beyond_last_line = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "垂直スクロールマージン",
                        description: "自動スクロール時にカーソルの上下に確保する行数です。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("vertical_scroll_margin"),
                            pick: |settings_content| {
                                settings_content.editor.vertical_scroll_margin.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.vertical_scroll_margin = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "水平スクロールマージン",
                        description: "マウススクロール時にカーソルの左右に確保する文字数です。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("horizontal_scroll_margin"),
                            pick: |settings_content| {
                                settings_content.editor.horizontal_scroll_margin.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.horizontal_scroll_margin = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "スクロール感度",
                        description: "水平および垂直スクロールの感度倍率です。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("scroll_sensitivity"),
                            pick: |settings_content| {
                                settings_content.editor.scroll_sensitivity.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.scroll_sensitivity = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "高速スクロール感度",
                        description: "水平および垂直の高速スクロール感度倍率です。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("fast_scroll_sensitivity"),
                            pick: |settings_content| {
                                settings_content.editor.fast_scroll_sensitivity.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.fast_scroll_sensitivity = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "クリック時の自動スクロール",
                        description: "表示されているテキスト領域の端付近をクリックしたときにスクロールするかどうかを設定します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("autoscroll_on_clicks"),
                            pick: |settings_content| {
                                settings_content.editor.autoscroll_on_clicks.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.autoscroll_on_clicks = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "スティッキースクロール",
                        description: "現在のスコープをエディター上部に固定表示するかどうかを設定します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("sticky_scroll.enabled"),
                            pick: |settings_content| {
                                settings_content.editor.sticky_scroll.as_ref().and_then(|sticky_scroll| sticky_scroll.enabled.as_ref())
                            },
                            write: |settings_content, value| {
                                settings_content.editor.sticky_scroll.get_or_insert_default().enabled = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "スムーズスクロール",
                        description: "滑らかなイージング効果を付けてスクロールをアニメーション表示するかどうかを設定します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("smooth_scroll.enabled"),
                            pick: |settings_content| {
                                settings_content.editor.smooth_scroll.as_ref().and_then(|smooth_scroll| smooth_scroll.enabled.as_ref())
                            },
                            write: |settings_content, value| {
                                settings_content.editor.smooth_scroll.get_or_insert_default().enabled = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "スクロールアニメーション時間",
                        description: "スクロールアニメーションの所要時間です。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("smooth_scroll.duration"),
                            pick: |settings_content| {
                                settings_content.editor.smooth_scroll.as_ref().and_then(|smooth_scroll| smooth_scroll.duration.as_ref())
                            },
                            write: |settings_content, value| {
                                settings_content.editor.smooth_scroll.get_or_insert_default().duration = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),

                    SettingsPageItem::SectionHeader("シグネチャヘルプ"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "自動シグネチャヘルプ",
                        description: "シグネチャヘルプのポップアップを自動的に表示します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("auto_signature_help"),
                            pick: |settings_content| {
                                settings_content.editor.auto_signature_help.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.auto_signature_help = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "編集後にシグネチャヘルプを表示",
                        description: "補完の決定時や括弧ペアの挿入後にシグネチャヘルプのポップアップを表示します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("show_signature_help_after_edits"),
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .show_signature_help_after_edits
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.show_signature_help_after_edits = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "スニペットの並び順",
                        description: "他の補完項目に対するスニペットの並び順を決定します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("snippet_sort_order"),
                            pick: |settings_content| {
                                settings_content.editor.snippet_sort_order.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.snippet_sort_order = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SectionHeader("ホバーポップオーバー"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "有効",
                        description: "エディター内のシンボル上にマウスを移動したときに情報ホバーボックスを表示します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("hover_popover_enabled"),
                            pick: |settings_content| {
                                settings_content.editor.hover_popover_enabled.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.hover_popover_enabled = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    // todo(settings ui): add units to this number input
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "遅延時間",
                        description: "情報ホバーボックスを表示するまでの待機時間（ミリ秒）です。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("hover_popover_enabled"),
                            pick: |settings_content| {
                                settings_content.editor.hover_popover_delay.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.hover_popover_delay = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SectionHeader("ドラッグ＆ドロップ選択"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "有効",
                        description: "ドラッグ＆ドロップによるテキスト選択を有効にします。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("drag_and_drop_selection.enabled"),
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .drag_and_drop_selection
                                    .as_ref()
                                    .and_then(|drag_and_drop| drag_and_drop.enabled.as_ref())
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .drag_and_drop_selection
                                    .get_or_insert_default()
                                    .enabled = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "遅延時間",
                        description: "ドラッグ＆ドロップ選択が開始されるまでの遅延時間（ミリ秒）です。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("drag_and_drop_selection.delay"),
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .drag_and_drop_selection
                                    .as_ref()
                                    .and_then(|drag_and_drop| drag_and_drop.delay.as_ref())
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .drag_and_drop_selection
                                    .get_or_insert_default()
                                    .delay = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SectionHeader("ガター"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "行番号を表示",
                        description: "ガターに行番号を表示します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("gutter.line_numbers"),
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .gutter
                                    .as_ref()
                                    .and_then(|gutter| gutter.line_numbers.as_ref())
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .gutter
                                    .get_or_insert_default()
                                    .line_numbers = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "相対行番号",
                        description: "エディターのガターにおける行番号の表示方法を制御します。「disabled」は絶対行番号を表示し、「enabled」は各絶対行に対する相対行番号を表示し、「wrapped」は折り返し行を含むすべての行の相対行番号を表示します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("relative_line_numbers"),
                            pick: |settings_content| {
                                settings_content.editor.relative_line_numbers.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.relative_line_numbers = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "行番号の倍率",
                        description: "エディターに表示される行番号の相対サイズです。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("editor.line_number_scale"),
                            pick: |settings_content| settings_content.editor.line_number_scale.as_ref(),
                            write: |settings_content, value| {
                                settings_content.editor.line_number_scale = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "実行可能ボタンを表示",
                        description: "ガターに実行ボタンを表示します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("gutter.runnables"),
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .gutter
                                    .as_ref()
                                    .and_then(|gutter| gutter.runnables.as_ref())
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .gutter
                                    .get_or_insert_default()
                                    .runnables = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "ブレークポイントを表示",
                        description: "ガターにブレークポイントを表示します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("gutter.breakpoints"),
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .gutter
                                    .as_ref()
                                    .and_then(|gutter| gutter.breakpoints.as_ref())
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .gutter
                                    .get_or_insert_default()
                                    .breakpoints = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "コード折りたたみを表示",
                        description: "ガターにコード折りたたみコントロールを表示します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("gutter.folds"),
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .gutter
                                    .as_ref()
                                    .and_then(|gutter| gutter.folds.as_ref())
                            },
                            write: |settings_content, value| {
                                settings_content.editor.gutter.get_or_insert_default().folds =
                                    value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "行番号の最小桁数",
                        description: "ガターで行番号用に確保する最小の文字幅です。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("gutter.min_line_number_digits"),
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .gutter
                                    .as_ref()
                                    .and_then(|gutter| gutter.min_line_number_digits.as_ref())
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .gutter
                                    .get_or_insert_default()
                                    .min_line_number_digits = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "インラインコードアクション",
                        description: "バッファー行の先頭にコードアクションボタンを表示します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("inline_code_actions"),
                            pick: |settings_content| {
                                settings_content.editor.inline_code_actions.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.inline_code_actions = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SectionHeader("スクロールバー"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "表示",
                        description: "エディターでスクロールバーを表示するタイミングを設定します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("scrollbar"),
                            pick: |settings_content| {
                                settings_content.editor.scrollbar.as_ref()?.show.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .get_or_insert_default()
                                    .show = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "カーソル",
                        description: "スクロールバー上にカーソル位置を表示します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("scrollbar.cursors"),
                            pick: |settings_content| {
                                settings_content.editor.scrollbar.as_ref()?.cursors.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .get_or_insert_default()
                                    .cursors = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "Git差分",
                        description: "スクロールバー上にGitの差分インジケーターを表示します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("scrollbar.git_diff"),
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .as_ref()?
                                    .git_diff
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .get_or_insert_default()
                                    .git_diff = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "検索結果",
                        description: "スクロールバー上にバッファー内検索結果のインジケーターを表示します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("scrollbar.search_results"),
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .as_ref()?
                                    .search_results
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .get_or_insert_default()
                                    .search_results = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "選択テキスト",
                        description: "スクロールバー上に選択テキストの一致箇所を表示します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("scrollbar.selected_text"),
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .as_ref()?
                                    .selected_text
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .get_or_insert_default()
                                    .selected_text = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "選択シンボル",
                        description: "スクロールバー上に選択シンボルの一致箇所を表示します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("scrollbar.selected_symbol"),
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .as_ref()?
                                    .selected_symbol
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .get_or_insert_default()
                                    .selected_symbol = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "診断",
                        description: "スクロールバー上にどの診断インジケーターを表示するかを設定します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("scrollbar.diagnostics"),
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .as_ref()?
                                    .diagnostics
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .get_or_insert_default()
                                    .diagnostics = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "水平スクロールバー",
                        description: "falseの場合、水平スクロールバーを強制的に無効化します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("scrollbar.axes.horizontal"),
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .as_ref()?
                                    .axes
                                    .as_ref()?
                                    .horizontal
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .get_or_insert_default()
                                    .axes
                                    .get_or_insert_default()
                                    .horizontal = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "垂直スクロールバー",
                        description: "falseの場合、垂直スクロールバーを強制的に無効化します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("scrollbar.axes.vertical"),
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .as_ref()?
                                    .axes
                                    .as_ref()?
                                    .vertical
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .scrollbar
                                    .get_or_insert_default()
                                    .axes
                                    .get_or_insert_default()
                                    .vertical = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SectionHeader("ミニマップ"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "表示",
                        description: "エディターでミニマップを表示するタイミングを設定します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("minimap.show"),
                            pick: |settings_content| {
                                settings_content.editor.minimap.as_ref()?.show.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.editor.minimap.get_or_insert_default().show =
                                    value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "表示位置",
                        description: "エディターのどこにミニマップを表示するかを設定します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("minimap.display_in"),
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .minimap
                                    .as_ref()?
                                    .display_in
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .minimap
                                    .get_or_insert_default()
                                    .display_in = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "つまみ",
                        description: "ミニマップのつまみ（スライダー）を表示するタイミングを設定します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("minimap.thumb"),
                            pick: |settings_content| {
                                settings_content.editor.minimap.as_ref()?.thumb.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .minimap
                                    .get_or_insert_default()
                                    .thumb = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "つまみの枠線",
                        description: "ミニマップのスクロールバーつまみの枠線スタイルです。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("minimap.thumb_border"),
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .minimap
                                    .as_ref()?
                                    .thumb_border
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .minimap
                                    .get_or_insert_default()
                                    .thumb_border = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "現在の行のハイライト",
                        description: "ミニマップで現在の行をハイライトする方法を設定します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("minimap.current_line_highlight"),
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .minimap
                                    .as_ref()
                                    .and_then(|minimap| minimap.current_line_highlight.as_ref())
                                    .or(settings_content.editor.current_line_highlight.as_ref())
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .minimap
                                    .get_or_insert_default()
                                    .current_line_highlight = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "最大列幅",
                        description: "ミニマップに表示する最大列数（文字数幅）です。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("minimap.max_width_columns"),
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .minimap
                                    .as_ref()?
                                    .max_width_columns
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .minimap
                                    .get_or_insert_default()
                                    .max_width_columns = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SectionHeader("ツールバー"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "パンくずリスト",
                        description: "パンくずリストを表示します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("toolbar.breadcrumbs"),
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .toolbar
                                    .as_ref()?
                                    .breadcrumbs
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .toolbar
                                    .get_or_insert_default()
                                    .breadcrumbs = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "クイックアクション",
                        description: "クイックアクションボタン（検索、選択、エディター制御など）を表示します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("toolbar.quick_actions"),
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .toolbar
                                    .as_ref()?
                                    .quick_actions
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .toolbar
                                    .get_or_insert_default()
                                    .quick_actions = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "選択メニュー",
                        description: "エディターのツールバーに選択メニューを表示します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("toolbar.selections_menu"),
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .toolbar
                                    .as_ref()?
                                    .selections_menu
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .toolbar
                                    .get_or_insert_default()
                                    .selections_menu = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "コードアクション",
                        description: "エディターのツールバーにコードアクションボタンを表示します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("toolbar.code_actions"),
                            pick: |settings_content| {
                                settings_content
                                    .editor
                                    .toolbar
                                    .as_ref()?
                                    .code_actions
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .editor
                                    .toolbar
                                    .get_or_insert_default()
                                    .code_actions = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                ];
                items.extend(language_settings_data());
                items
            },
        },
        SettingsPage {
            title: "言語とツール",
            items: {
                let mut items = vec![];
                items.extend(non_editor_language_settings_data());
                items.extend([
                    SettingsPageItem::SectionHeader("ファイルの種類"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "ファイルの関連付け",
                        description: "言語と、その言語として扱うファイルや拡張子の対応付けを設定します。",
                        aliases: None,
                        field: Box::new(
                            SettingField {
                                json_path: Some("file_type_associations"),
                                pick: |settings_content| {
                                    settings_content.project.all_languages.file_types.as_ref()
                                },
                                write: |settings_content, value| {
                                    settings_content.project.all_languages.file_types = value;

                                },
                            }
                            .unimplemented(),
                        ),
                        metadata: None,
                        files: USER | PROJECT,
                    }),
                ]);

                items.extend([
                    SettingsPageItem::SectionHeader("診断"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "最大重要度",
                        description: "エディターに表示する診断をフィルタリングする重要度レベルを設定します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("diagnostics_max_severity"),
                            pick: |settings_content| settings_content.editor.diagnostics_max_severity.as_ref(),
                            write: |settings_content, value| {
                                settings_content.editor.diagnostics_max_severity = value;

                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "警告を含める",
                        description: "デフォルトで警告を表示するかどうかを設定します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("diagnostics.include_warnings"),
                            pick: |settings_content| {
                                settings_content.diagnostics.as_ref()?.include_warnings.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content

                                    .diagnostics
                                    .get_or_insert_default()
                                    .include_warnings
                                    = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SectionHeader("インライン診断"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "有効",
                        description: "診断をインラインで表示するかどうかを設定します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("diagnostics.inline.enabled"),
                            pick: |settings_content| {
                                settings_content.diagnostics.as_ref()?.inline.as_ref()?.enabled.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content

                                    .diagnostics
                                    .get_or_insert_default()
                                    .inline
                                    .get_or_insert_default()
                                    .enabled
                                    = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "更新デバウンス",
                        description: "診断の最新更新後にインライン診断を表示するまでの遅延時間（ミリ秒）です。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("diagnostics.inline.update_debounce_ms"),
                            pick: |settings_content| {
                                settings_content.diagnostics.as_ref()?.inline.as_ref()?.update_debounce_ms.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content

                                    .diagnostics
                                    .get_or_insert_default()
                                    .inline
                                    .get_or_insert_default()
                                    .update_debounce_ms
                                    = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "余白",
                        description: "ソースコード行末とインライン診断表示開始位置との間の余白量です。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("diagnostics.inline.padding"),
                            pick: |settings_content| {
                                settings_content.diagnostics.as_ref()?.inline.as_ref()?.padding.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content

                                    .diagnostics
                                    .get_or_insert_default()
                                    .inline
                                    .get_or_insert_default()
                                    .padding
                                    = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "最小列",
                        description: "インライン診断を表示する最小列位置です。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("diagnostics.inline.min_column"),
                            pick: |settings_content| {
                                settings_content.diagnostics.as_ref()?.inline.as_ref()?.min_column.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content

                                    .diagnostics
                                    .get_or_insert_default()
                                    .inline
                                    .get_or_insert_default()
                                    .min_column
                                    = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SectionHeader("LSPプル診断"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "有効",
                        description: "言語サーバーによる診断を取得（プル）するかどうかを設定します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("diagnostics.lsp_pull_diagnostics.enabled"),
                            pick: |settings_content| {
                                settings_content.diagnostics.as_ref()?.lsp_pull_diagnostics.as_ref()?.enabled.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content

                                    .diagnostics
                                    .get_or_insert_default()
                                    .lsp_pull_diagnostics
                                    .get_or_insert_default()
                                    .enabled
                                    = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    // todo(settings_ui): Needs unit
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "デバウンス",
                        description: "言語サーバーから診断を取得するまでの最小待機時間です。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("diagnostics.lsp_pull_diagnostics.debounce_ms"),
                            pick: |settings_content| {
                                settings_content.diagnostics.as_ref()?.lsp_pull_diagnostics.as_ref()?.debounce_ms.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content

                                    .diagnostics
                                    .get_or_insert_default()
                                    .lsp_pull_diagnostics
                                    .get_or_insert_default()
                                    .debounce_ms
                                    = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                    SettingsPageItem::SectionHeader("LSPハイライト"),
                    SettingsPageItem::SettingItem(SettingItem {
                        title: "デバウンス",
                        description: "言語からハイライトを問い合わせるまでのデバウンス遅延時間です。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("lsp_highlight_debounce"),
                            pick: |settings_content| settings_content.editor.lsp_highlight_debounce.as_ref(),
                            write: |settings_content, value| {
                                settings_content.editor.lsp_highlight_debounce = value;
                            },
                        }),
                        metadata: None,
                        files: USER,
                    }),
                ]);

                // todo(settings_ui): Refresh on extension (un)/installed
                // Note that `crates/json_schema_store` solves the same problem, there is probably a way to unify the two
                items.push(SettingsPageItem::SectionHeader(LANGUAGES_SECTION_HEADER));
                items.extend(all_language_names(cx).into_iter().map(|language_name| {
                    let link = format!("languages.{language_name}");
                    SettingsPageItem::SubPageLink(SubPageLink {
                        title: language_name,
                        description: None,
                        json_path: Some(link.leak()),
                        in_json: true,
                        files: USER | PROJECT,
                        render: Arc::new(|this, window, cx| {
                            this.render_sub_page_items(
                                language_settings_data()
                                    .iter()
                                    .chain(non_editor_language_settings_data().iter())
                                    .enumerate(),
                                None,
                                window,
                                cx,
                            )
                            .into_any_element()
                        }),
                    })
                }));
                items
            },
        },
        SettingsPage {
            title: "検索とファイル",
            items: vec![
                SettingsPageItem::SectionHeader("検索"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "単語全体",
                    description: "デフォルトで単語全体に一致する文字列のみを検索します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("search.whole_word"),
                        pick: |settings_content| {
                            settings_content.editor.search.as_ref()?.whole_word.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .editor
                                .search
                                .get_or_insert_default()
                                .whole_word = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "大文字と小文字を区別",
                    description: "デフォルトで大文字と小文字を区別して検索します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("search.case_sensitive"),
                        pick: |settings_content| {
                            settings_content
                                .editor
                                .search
                                .as_ref()?
                                .case_sensitive
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .editor
                                .search
                                .get_or_insert_default()
                                .case_sensitive = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "スマートケース検索を使用",
                    description: "検索文字列に応じて大文字と小文字の区別を自動的に切り替えるかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("use_smartcase_search"),
                        pick: |settings_content| {
                            settings_content.editor.use_smartcase_search.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.editor.use_smartcase_search = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "無視ファイルを含める",
                    description: "デフォルトで検索結果に無視対象のファイルを含めるかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("search.include_ignored"),
                        pick: |settings_content| {
                            settings_content
                                .editor
                                .search
                                .as_ref()?
                                .include_ignored
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .editor
                                .search
                                .get_or_insert_default()
                                .include_ignored = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "正規表現",
                    description: "デフォルトで正規表現検索を使用します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("search.regex"),
                        pick: |settings_content| {
                            settings_content.editor.search.as_ref()?.regex.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.editor.search.get_or_insert_default().regex = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "折り返し検索",
                    description: "エディターでの検索が末尾に達したときに先頭に戻ってループするかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("search_wrap"),
                        pick: |settings_content| settings_content.editor.search_wrap.as_ref(),
                        write: |settings_content, value| {
                            settings_content.editor.search_wrap = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "一致箇所を中央に表示",
                    description: "エディター内で現在の一致項目を画面中央に配置するかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("editor.search.center_on_match"),
                        pick: |settings_content| {
                            settings_content
                                .editor
                                .search
                                .as_ref()
                                .and_then(|search| search.center_on_match.as_ref())
                        },
                        write: |settings_content, value| {
                            settings_content
                                .editor
                                .search
                                .get_or_insert_default()
                                .center_on_match = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "カーソル位置のテキストから検索文字列を生成",
                    description: "カーソル下のテキストを新規検索のクエリとして自動入力するタイミングを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("seed_search_query_from_cursor"),
                        pick: |settings_content| {
                            settings_content
                                .editor
                                .seed_search_query_from_cursor
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.editor.seed_search_query_from_cursor = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("ファイルファインダー"),
                // todo: null by default
                SettingsPageItem::SettingItem(SettingItem {
                    title: "検索に無視ファイルを含める",
                    description: "ファイル検索時に.gitignoreで無視されたファイルを含めます。",
                    aliases: None,
                    field: Box::new(
                        SettingField {
                            json_path: Some("file_finder.include_ignored"),
                            pick: |settings_content| {
                                settings_content
                                    .file_finder
                                    .as_ref()?
                                    .include_ignored
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .file_finder
                                    .get_or_insert_default()
                                    .include_ignored = value;
                            },
                        }
                    ),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "ファイルアイコン",
                    description: "ファイルファインダーにファイルアイコンを表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("file_finder.file_icons"),
                        pick: |settings_content| {
                            settings_content.file_finder.as_ref()?.file_icons.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .file_finder
                                .get_or_insert_default()
                                .file_icons = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "モーダルの最大幅",
                    description: "利用可能なウィンドウ幅に対してファイルファインダーが占有できる最大幅を設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("file_finder.modal_max_width"),
                        pick: |settings_content| {
                            settings_content
                                .file_finder
                                .as_ref()?
                                .modal_max_width
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .file_finder
                                .get_or_insert_default()
                                .modal_max_width = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "検索結果のアクティブファイルのフォーカスをスキップ",
                    description: "ファイルファインダーの検索結果において、現在アクティブなファイルへのフォーカスをスキップするかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("file_finder.skip_focus_for_active_in_search"),
                        pick: |settings_content| {
                            settings_content
                                .file_finder
                                .as_ref()?
                                .skip_focus_for_active_in_search
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .file_finder
                                .get_or_insert_default()
                                .skip_focus_for_active_in_search = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Gitステータス",
                    description: "ファイルファインダーにGitのステータスを表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("file_finder.git_status"),
                        pick: |settings_content| {
                            settings_content.file_finder.as_ref()?.git_status.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .file_finder
                                .get_or_insert_default()
                                .git_status = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("ファイルスキャン"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "ファイルスキャン除外設定",
                    description: "Gramが完全に除外するファイルまたはGlobパターンです。ファイルスキャンやファイル検索からスキップされ、プロジェクトファイルツリーにも表示されません。「ファイルスキャン包含設定」より優先されます。",
                    aliases: None,
                    field: Box::new(
                        SettingField {
                            json_path: Some("file_scan_exclusions"),
                            pick: |settings_content| {
                                settings_content
                                    .project
                                    .worktree
                                    .file_scan_exclusions
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.project.worktree.file_scan_exclusions = value;
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "ファイルスキャン包含設定",
                    description: "Gitで無視されている場合でもGramに含めるファイルまたはGlobパターンです。Gitで追跡されていないがプロジェクトにとって重要なファイルに有用です。広すぎるGlobパターンはGramのファイルスキャンを遅くする可能性があります。「ファイルスキャン除外設定」がこれらより優先されます。",
                    aliases: None,
                    field: Box::new(
                        SettingField {
                            json_path: Some("file_scan_inclusions"),
                            pick: |settings_content| {
                                settings_content
                                    .project
                                    .worktree
                                    .file_scan_inclusions
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.project.worktree.file_scan_inclusions = value;
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "ファイル状態の復元",
                    description: "ファイルを再度開いたときに以前の状態を復元します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("restore_on_file_reopen"),
                        pick: |settings_content| {
                            settings_content.workspace.restore_on_file_reopen.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.workspace.restore_on_file_reopen = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "ファイル削除時に閉じる",
                    description: "削除されたファイルを自動的に閉じます。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("close_on_file_delete"),
                        pick: |settings_content| {
                            settings_content.workspace.close_on_file_delete.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.workspace.close_on_file_delete = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
            ],
        },
        SettingsPage {
            title: "ウィンドウとレイアウト",
            items: vec![
                SettingsPageItem::SectionHeader("ステータスバー"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "表示",
                    description: "ステータスバーを表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("status_bar.show"),
                        pick: |settings_content| {
                            settings_content
                                .status_bar
                                .as_ref()?
                                .show
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .status_bar
                                .get_or_insert_default()
                                .show = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "配置位置",
                    description: "ステータスバーをウィンドウの上部または下部に表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("status_bar.position"),
                        pick: |settings_content| {
                            settings_content
                                .status_bar
                                .as_ref()?
                                .position
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .status_bar
                                .get_or_insert_default()
                                .position = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "アイコンサイズ",
                    description: "ステータスバー内のアイコンのサイズです。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("status_bar.icon_size"),
                        pick: |settings_content| {
                            settings_content
                                .status_bar
                                .as_ref()?
                                .icon_size
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .status_bar
                                .get_or_insert_default()
                                .icon_size = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "アクティブファイル",
                    description: "ステータスバーにアクティブなファイル名を表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("status_bar.active_file"),
                        pick: |settings_content| {
                            settings_content
                                .status_bar
                                .as_ref()?
                                .active_file
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .status_bar
                                .get_or_insert_default()
                                .active_file = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "プロジェクトパネルボタン",
                    description: "ステータスバーにプロジェクトパネルボタンを表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("project_panel.button"),
                        pick: |settings_content| {
                            settings_content.project_panel.as_ref()?.button.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .button = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "アクティブ言語ボタン",
                    description: "ステータスバーにアクティブ言語ボタンを表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("status_bar.active_language_button"),
                        pick: |settings_content| {
                            settings_content
                                .status_bar
                                .as_ref()?
                                .active_language_button
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .status_bar
                                .get_or_insert_default()
                                .active_language_button = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "アクティブエンコーディングボタン",
                    description: "ステータスバーにアクティブなエンコーディングを表示するタイミングを制御します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("status_bar.active_encoding_button"),
                        pick: |settings_content| {
                            settings_content
                                .status_bar
                                .as_ref()?
                                .active_encoding_button
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .status_bar
                                .get_or_insert_default()
                                .active_encoding_button = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "カーソル位置ボタン",
                    description: "ステータスバーにカーソル位置ボタンを表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("status_bar.cursor_position_button"),
                        pick: |settings_content| {
                            settings_content
                                .status_bar
                                .as_ref()?
                                .cursor_position_button
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .status_bar
                                .get_or_insert_default()
                                .cursor_position_button = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "ターミナルボタン",
                    description: "ステータスバーにターミナルボタンを表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("terminal.button"),
                        pick: |settings_content| {
                            settings_content.terminal.as_ref()?.button.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.terminal.get_or_insert_default().button = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "診断ボタン",
                    description: "ステータスバーにプロジェクトの診断ボタンを表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("diagnostics.button"),
                        pick: |settings_content| {
                            settings_content.diagnostics.as_ref()?.button.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.diagnostics.get_or_insert_default().button = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "プロジェクト検索ボタン",
                    description: "ステータスバーにプロジェクト検索ボタンを表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("search.button"),
                        pick: |settings_content| {
                            settings_content.editor.search.as_ref()?.button.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .editor
                                .search
                                .get_or_insert_default()
                                .button = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "デバッガーボタン",
                    description: "ステータスバーにデバッガーボタンを表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("debugger.button"),
                        pick: |settings_content| {
                            settings_content.debugger.as_ref()?.button.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.debugger.get_or_insert_default().button = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("タイトルバー"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "タイトルバーを表示",
                    description: "タイトルバー全体を表示または非表示にします。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("title_bar.show"),
                        pick: |settings_content| {
                            settings_content
                                .title_bar
                                .as_ref()?
                                .show
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .title_bar
                                .get_or_insert_default()
                                .show = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "ブランチアイコンを表示",
                    description: "タイトルバーのブランチ切り替えの横にブランチアイコンを表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("title_bar.show_branch_icon"),
                        pick: |settings_content| {
                            settings_content
                                .title_bar
                                .as_ref()?
                                .show_branch_icon
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .title_bar
                                .get_or_insert_default()
                                .show_branch_icon = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "ブランチ名を表示",
                    description: "タイトルバーにブランチ名ボタンを表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("title_bar.show_branch_name"),
                        pick: |settings_content| {
                            settings_content
                                .title_bar
                                .as_ref()?
                                .show_branch_name
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .title_bar
                                .get_or_insert_default()
                                .show_branch_name = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "プロジェクト項目を表示",
                    description: "タイトルバーにプロジェクトのホストと名前を表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("title_bar.show_project_items"),
                        pick: |settings_content| {
                            settings_content
                                .title_bar
                                .as_ref()?
                                .show_project_items
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .title_bar
                                .get_or_insert_default()
                                .show_project_items = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "オンボーディングバナーを表示",
                    description: "タイトルバーに新機能を通知するバナーを表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("title_bar.show_onboarding_banner"),
                        pick: |settings_content| {
                            settings_content
                                .title_bar
                                .as_ref()?
                                .show_onboarding_banner
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .title_bar
                                .get_or_insert_default()
                                .show_onboarding_banner = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "メニューを表示",
                    description: "タイトルバーにメニューを表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("title_bar.show_menus"),
                        pick: |settings_content| {
                            settings_content.title_bar.as_ref()?.show_menus.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .title_bar
                                .get_or_insert_default()
                                .show_menus = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("タブバー"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "タブバーを表示",
                    description: "エディターにタブバーを表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("tab_bar.show"),
                        pick: |settings_content| settings_content.tab_bar.as_ref()?.show.as_ref(),
                        write: |settings_content, value| {
                            settings_content.tab_bar.get_or_insert_default().show = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "タブにGitステータスを表示",
                    description: "タブ項目にGitのファイルステータスを表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("tabs.git_status"),
                        pick: |settings_content| {
                            settings_content.tabs.as_ref()?.git_status.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.tabs.get_or_insert_default().git_status = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "タブにファイルアイコンを表示",
                    description: "タブにファイルアイコンを表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("tabs.file_icons"),
                        pick: |settings_content| {
                            settings_content.tabs.as_ref()?.file_icons.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.tabs.get_or_insert_default().file_icons = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "タブの閉じるボタンの位置",
                    description: "タブ内の閉じるボタンの配置位置です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("tabs.close_position"),
                        pick: |settings_content| {
                            settings_content.tabs.as_ref()?.close_position.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.tabs.get_or_insert_default().close_position = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    files: USER,
                    title: "最大タブ数",
                    description: "ペイン内で開ける最大タブ数です。未保存のタブは閉じられません。",
                    aliases: None,
                    // todo(settings_ui): The default for this value is null and it's use in code
                    // is complex, so I'm going to come back to this later
                    field: Box::new(
                        SettingField {
                            json_path: Some("max_tabs"),
                            pick: |settings_content| settings_content.workspace.max_tabs.as_ref(),
                            write: |settings_content, value| {
                                settings_content.workspace.max_tabs = value;
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "固定タブのレイアウト",
                    description: "固定されたタブを通常のタブの上に別行として表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("tab_bar.show_pinned_tabs_in_separate_row"),
                        pick: |settings_content| {
                            settings_content
                                .tab_bar
                                .as_ref()?
                                .show_pinned_tabs_in_separate_row
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .tab_bar
                                .get_or_insert_default()
                                .show_pinned_tabs_in_separate_row = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "タブの折り返し",
                    description: "タブバーがあふれたときにタブを次の行へ折り返して表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("tab_bar.wrap_tabs"),
                        pick: |settings_content| {
                            settings_content.tab_bar.as_ref()?.wrap_tabs.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.tab_bar.get_or_insert_default().wrap_tabs = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "ナビゲーション履歴ボタンを表示",
                    description: "タブバーにナビゲーション履歴ボタンを表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("tab_bar.show_nav_history_buttons"),
                        pick: |settings_content| {
                            settings_content
                                .tab_bar
                                .as_ref()?
                                .show_nav_history_buttons
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .tab_bar
                                .get_or_insert_default()
                                .show_nav_history_buttons = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "タブバーボタンを表示",
                    description: "タブバーボタン（新規作成、ペイン分割、ズーム）を表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("tab_bar.show_tab_bar_buttons"),
                        pick: |settings_content| {
                            settings_content
                                .tab_bar
                                .as_ref()?
                                .show_tab_bar_buttons
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .tab_bar
                                .get_or_insert_default()
                                .show_tab_bar_buttons = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("タブ設定"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "閉じたときのアクティブ化対象",
                    description: "現在のタブを閉じた後にアクティブにするタブを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("tabs.activate_on_close"),
                        pick: |settings_content| {
                            settings_content.tabs.as_ref()?.activate_on_close.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .tabs
                                .get_or_insert_default()
                                .activate_on_close = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "タブの診断表示",
                    description: "診断エラーや警告を含むファイルをタブ上でどのようにマークするかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("tabs.show_diagnostics"),
                        pick: |settings_content| {
                            settings_content.tabs.as_ref()?.show_diagnostics.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .tabs
                                .get_or_insert_default()
                                .show_diagnostics = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "閉じるボタンを表示",
                    description: "タブの閉じるボタンの表示動作を設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("tabs.show_close_button"),
                        pick: |settings_content| {
                            settings_content.tabs.as_ref()?.show_close_button.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .tabs
                                .get_or_insert_default()
                                .show_close_button = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("プレビュータブ"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "プレビュータブを有効化",
                    description: "開いたエディターをプレビュータブとして表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("preview_tabs.enabled"),
                        pick: |settings_content| {
                            settings_content.preview_tabs.as_ref()?.enabled.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .preview_tabs
                                .get_or_insert_default()
                                .enabled = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "プロジェクトパネルからのプレビューを有効化",
                    description: "プロジェクトパネルからシングルクリックで開いたときにプレビューモードで開くかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("preview_tabs.enable_preview_from_project_panel"),
                        pick: |settings_content| {
                            settings_content
                                .preview_tabs
                                .as_ref()?
                                .enable_preview_from_project_panel
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .preview_tabs
                                .get_or_insert_default()
                                .enable_preview_from_project_panel = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "ファイルファインダーからのプレビューを有効化",
                    description: "ファイルファインダーから選択したときにプレビューモードで開くかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("preview_tabs.enable_preview_from_file_finder"),
                        pick: |settings_content| {
                            settings_content
                                .preview_tabs
                                .as_ref()?
                                .enable_preview_from_file_finder
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .preview_tabs
                                .get_or_insert_default()
                                .enable_preview_from_file_finder = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "マルチバッファーからのプレビューを有効化",
                    description: "マルチバッファーから開いたときにプレビューモードで開くかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("preview_tabs.enable_preview_from_multibuffer"),
                        pick: |settings_content| {
                            settings_content
                                .preview_tabs
                                .as_ref()?
                                .enable_preview_from_multibuffer
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .preview_tabs
                                .get_or_insert_default()
                                .enable_preview_from_multibuffer = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "コードナビゲーションからのマルチバッファープレビューを有効化",
                    description: "コードナビゲーションを使用してマルチバッファーを開いたときにプレビューモードで開くかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("preview_tabs.enable_preview_multibuffer_from_code_navigation"),
                        pick: |settings_content| {
                            settings_content
                                .preview_tabs
                                .as_ref()?
                                .enable_preview_multibuffer_from_code_navigation
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .preview_tabs
                                .get_or_insert_default()
                                .enable_preview_multibuffer_from_code_navigation = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "コードナビゲーションからのファイルプレビューを有効化",
                    description: "コードナビゲーションを使用して単一ファイルを開いたときにプレビューモードで開くかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("preview_tabs.enable_preview_file_from_code_navigation"),
                        pick: |settings_content| {
                            settings_content
                                .preview_tabs
                                .as_ref()?
                                .enable_preview_file_from_code_navigation
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .preview_tabs
                                .get_or_insert_default()
                                .enable_preview_file_from_code_navigation = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "コードナビゲーション時にプレビュー状態を維持",
                    description: "コードナビゲーションで別のファイルへ移動する際にタブのプレビューモードを維持するかどうかを設定します。ファイルやマルチバッファーのプレビューが有効な場合、既存のタブが新しいタブに置き換わる可能性があります。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("preview_tabs.enable_keep_preview_on_code_navigation"),
                        pick: |settings_content| {
                            settings_content
                                .preview_tabs
                                .as_ref()?
                                .enable_keep_preview_on_code_navigation
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .preview_tabs
                                .get_or_insert_default()
                                .enable_keep_preview_on_code_navigation = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("レイアウト"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "ボトムドックのレイアウト",
                    description: "ボトムドックのレイアウトモードです。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("bottom_dock_layout"),
                        pick: |settings_content| {
                            settings_content.workspace.bottom_dock_layout.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.workspace.bottom_dock_layout = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    files: USER,
                    title: "中央揃えレイアウトの左余白",
                    description: "中央揃えレイアウト時の左側の余白です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("centered_layout.left_padding"),
                        pick: |settings_content| {
                            settings_content
                                .workspace
                                .centered_layout
                                .as_ref()?
                                .left_padding
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .workspace
                                .centered_layout
                                .get_or_insert_default()
                                .left_padding = value;
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    files: USER,
                    title: "中央揃えレイアウトの右余白",
                    description: "中央揃えレイアウト時の右側の余白です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("centered_layout.right_padding"),
                        pick: |settings_content| {
                            settings_content
                                .workspace
                                .centered_layout
                                .as_ref()?
                                .right_padding
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .workspace
                                .centered_layout
                                .get_or_insert_default()
                                .right_padding = value;
                        },
                    }),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("ウィンドウ"),
                // todo(settings_ui): Should we filter by platform.as_ref()?
                SettingsPageItem::SettingItem(SettingItem {
                    title: "システムウィンドウタブを使用",
                    description: "（macOSのみ）ウィンドウ同士をタブとしてまとめることを許可するかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("use_system_window_tabs"),
                        pick: |settings_content| {
                            settings_content.workspace.use_system_window_tabs.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.workspace.use_system_window_tabs = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "ウィンドウ装飾",
                    description: "（Linuxのみ）ウィンドウの枠線や装飾をGram側で描画するかコンポジターに任せるかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("window_decorations"),
                        pick: |settings_content| {
                            settings_content.workspace.window_decorations.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.workspace.window_decorations = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "OptionキーをAltとして扱う",
                    description: "（macOSのみ）エディターでどちらのOptionキーをAltキーとして扱うかを設定します。反映には再起動が必要です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("option_as_alt"),
                        pick: |settings_content| {
                            settings_content.workspace.option_as_alt.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.workspace.option_as_alt = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("ペイン修飾"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "非アクティブ時の不透明度",
                    description: "非アクティブなパネルの不透明度（0.0 - 1.0）です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("active_pane_modifiers.inactive_opacity"),
                        pick: |settings_content| {
                            settings_content
                                .workspace
                                .active_pane_modifiers
                                .as_ref()?
                                .inactive_opacity
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .workspace
                                .active_pane_modifiers
                                .get_or_insert_default()
                                .inactive_opacity = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "枠線の太さ",
                    description: "アクティブなペインを囲む枠線の太さです。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("active_pane_modifiers.border_size"),
                        pick: |settings_content| {
                            settings_content
                                .workspace
                                .active_pane_modifiers
                                .as_ref()?
                                .border_size
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .workspace
                                .active_pane_modifiers
                                .get_or_insert_default()
                                .border_size = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "ズーム時の余白",
                    description: "ズーム表示されたペインの余白を表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("zoomed_padding"),
                        pick: |settings_content| settings_content.workspace.zoomed_padding.as_ref(),
                        write: |settings_content, value| {
                            settings_content.workspace.zoomed_padding = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("ペイン分割方向"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "垂直分割方向",
                    description: "垂直方向に分割する際の位置方向です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("pane_split_direction_vertical"),
                        pick: |settings_content| {
                            settings_content
                                .workspace
                                .pane_split_direction_vertical
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.workspace.pane_split_direction_vertical = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "水平分割方向",
                    description: "水平方向に分割する際の位置方向です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("pane_split_direction_horizontal"),
                        pick: |settings_content| {
                            settings_content
                                .workspace
                                .pane_split_direction_horizontal
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.workspace.pane_split_direction_horizontal = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
            ],
        },
        SettingsPage {
            title: "パネル",
            items: vec![
                SettingsPageItem::SectionHeader("プロジェクトパネル"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "プロジェクトパネルのドック位置",
                    description: "プロジェクトパネルをドックする位置です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("project_panel.dock"),
                        pick: |settings_content| {
                            settings_content.project_panel.as_ref()?.dock.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.project_panel.get_or_insert_default().dock = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "プロジェクトパネルのデフォルト幅",
                    description: "プロジェクトパネルのデフォルトの幅（ピクセル単位）です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("project_panel.default_width"),
                        pick: |settings_content| {
                            settings_content
                                .project_panel
                                .as_ref()?
                                .default_width
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .default_width = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: ".gitignoreを非表示",
                    description: "プロジェクトパネルでgitignore対象の項目を非表示にするかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("project_panel.hide_gitignore"),
                        pick: |settings_content| {
                            settings_content
                                .project_panel
                                .as_ref()?
                                .hide_gitignore
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .hide_gitignore = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "項目の間隔",
                    description: "プロジェクトパネル内のワークツリー項目の間隔です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("project_panel.entry_spacing"),
                        pick: |settings_content| {
                            settings_content
                                .project_panel
                                .as_ref()?
                                .entry_spacing
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .entry_spacing = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "ファイルアイコン",
                    description: "プロジェクトパネルにファイルアイコンを表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("project_panel.file_icons"),
                        pick: |settings_content| {
                            settings_content.project_panel.as_ref()?.file_icons.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .file_icons = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "フォルダーアイコン",
                    description: "プロジェクトパネルのフォルダーに対してアイコンを表示するかシェブロンを表示するかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("project_panel.folder_icons"),
                        pick: |settings_content| {
                            settings_content
                                .project_panel
                                .as_ref()?
                                .folder_icons
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .folder_icons = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Gitステータス",
                    description: "プロジェクトパネルにGitのステータスを表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("project_panel.git_status"),
                        pick: |settings_content| {
                            settings_content.project_panel.as_ref()?.git_status.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .git_status = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "インデント幅",
                    description: "階層化された項目のインデント幅です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("project_panel.indent_size"),
                        pick: |settings_content| {
                            settings_content
                                .project_panel
                                .as_ref()?
                                .indent_size
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .indent_size = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "項目の自動展開表示",
                    description: "対応するプロジェクトのファイルがアクティブになったときに、プロジェクトパネル内の項目を自動的に展開して表示するかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("project_panel.auto_reveal_entries"),
                        pick: |settings_content| {
                            settings_content
                                .project_panel
                                .as_ref()?
                                .auto_reveal_entries
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .auto_reveal_entries = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "起動時に開く",
                    description: "起動時にプロジェクトパネルを開いた状態にするかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("project_panel.starts_open"),
                        pick: |settings_content| {
                            settings_content
                                .project_panel
                                .as_ref()?
                                .starts_open
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .starts_open = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "フォルダーの自動折りたたみ",
                    description: "フォルダー内にサブフォルダーが1つしかない場合に、自動的に折りたたんでコンパクト表示にするかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("project_panel.auto_fold_dirs"),
                        pick: |settings_content| {
                            settings_content
                                .project_panel
                                .as_ref()?
                                .auto_fold_dirs
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .auto_fold_dirs = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "スクロールバーを表示",
                    description: "プロジェクトパネルにスクロールバーを表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("project_panel.scrollbar.show"),
                        pick: |settings_content| {
                            show_scrollbar_or_editor(settings_content, |settings_content| {
                                settings_content
                                    .project_panel
                                    .as_ref()?
                                    .scrollbar
                                    .as_ref()?
                                    .show
                                    .as_ref()
                            })
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .scrollbar
                                .get_or_insert_default()
                                .show = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "診断を表示",
                    description: "診断エラーや警告を含むファイルをプロジェクトパネルでどのようにマークするかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("project_panel.show_diagnostics"),
                        pick: |settings_content| {
                            settings_content
                                .project_panel
                                .as_ref()?
                                .show_diagnostics
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .show_diagnostics = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "スティッキースクロール",
                    description: "親フォルダーをプロジェクトパネルの上部に固定表示するかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("project_panel.sticky_scroll"),
                        pick: |settings_content| {
                            settings_content
                                .project_panel
                                .as_ref()?
                                .sticky_scroll
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .sticky_scroll = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    files: USER,
                    title: "インデントガイドを表示",
                    description: "プロジェクトパネルにインデントガイドを表示します。",
                    aliases: None,
                    field: Box::new(
                        SettingField {
                            json_path: Some("project_panel.indent_guides.show"),
                            pick: |settings_content| {
                                settings_content
                                    .project_panel
                                    .as_ref()?
                                    .indent_guides
                                    .as_ref()?
                                    .show
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .project_panel
                                    .get_or_insert_default()
                                    .indent_guides
                                    .get_or_insert_default()
                                    .show = value;
                            },
                        }
                    ),
                    metadata: None,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "ドラッグ＆ドロップ",
                    description: "プロジェクトパネルでのドラッグ＆ドロップ操作を有効にするかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("project_panel.drag_and_drop"),
                        pick: |settings_content| {
                            settings_content
                                .project_panel
                                .as_ref()?
                                .drag_and_drop
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .drag_and_drop = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "ルートフォルダーを非表示",
                    description: "ウィンドウにフォルダーが1つだけ開かれている場合に、ルートフォルダー項目を非表示にするかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("project_panel.drag_and_drop"),
                        pick: |settings_content| {
                            settings_content.project_panel.as_ref()?.hide_root.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .hide_root = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "隠し項目を非表示",
                    description: "プロジェクトパネルで隠し項目を非表示にするかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("project_panel.hide_hidden"),
                        pick: |settings_content| {
                            settings_content
                                .project_panel
                                .as_ref()?
                                .hide_hidden
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .hide_hidden = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "隠しファイル",
                    description: "「隠し項目」とみなされ、プロジェクトパネルから非表示にできるファイルを判定するためのGlobパターンを指定します。",
                    aliases: None,
                    field: Box::new(
                        SettingField {
                            json_path: Some("worktree.hidden_files"),
                            pick: |settings_content| {
                                settings_content.project.worktree.hidden_files.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content.project.worktree.hidden_files = value;
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("ファイル自動オープン"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "新規作成時",
                    description: "新しく作成されたファイルをエディターで自動的に開くかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("project_panel.auto_open.on_create"),
                        pick: |settings_content| {
                            settings_content.project_panel.as_ref()?.auto_open.as_ref()?.on_create.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.project_panel.get_or_insert_default().auto_open.get_or_insert_default().on_create = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "貼り付け時",
                    description: "ファイルの貼り付けや複製を行った後に自動的に開くかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("project_panel.auto_open.on_paste"),
                        pick: |settings_content| {
                            settings_content.project_panel.as_ref()?.auto_open.as_ref()?.on_paste.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.project_panel.get_or_insert_default().auto_open.get_or_insert_default().on_paste = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "ドロップ時",
                    description: "外部からドロップされたファイルを自動的に開くかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("project_panel.auto_open.on_drop"),
                        pick: |settings_content| {
                            settings_content.project_panel.as_ref()?.auto_open.as_ref()?.on_drop.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.project_panel.get_or_insert_default().auto_open.get_or_insert_default().on_drop = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "フォーカスを移動",
                    description: "自動的に開かれたファイルにフォーカスを合わせるかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("project_panel.auto_open.should_focus"),
                        pick: |settings_content| {
                            settings_content.project_panel.as_ref()?.auto_open.as_ref()?.should_focus.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.project_panel.get_or_insert_default().auto_open.get_or_insert_default().should_focus = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "並び替えモード",
                    description: "プロジェクトパネル内の項目の並び順を設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        pick: |settings_content| {
                            settings_content.project_panel.as_ref()?.sort_mode.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .project_panel
                                .get_or_insert_default()
                                .sort_mode = value;
                        },
                        json_path: Some("project_panel.sort_mode"),
                    }),
                    metadata: Some(Box::new(SettingsFieldMetadata {
                        should_do_titlecase: Some(false),
                        ..Default::default()
                    })),
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("ターミナルパネル"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "ターミナルのドック位置",
                    description: "ターミナルパネルをドックする位置です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("terminal.dock"),
                        pick: |settings_content| settings_content.terminal.as_ref()?.dock.as_ref(),
                        write: |settings_content, value| {
                            settings_content.terminal.get_or_insert_default().dock = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("アウトラインパネル"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "アウトラインパネルボタン",
                    description: "ステータスバーにアウトラインパネルボタンを表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("outline_panel.button"),
                        pick: |settings_content| {
                            settings_content.outline_panel.as_ref()?.button.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .outline_panel
                                .get_or_insert_default()
                                .button = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "アウトラインパネルのドック位置",
                    description: "アウトラインパネルをドックする位置です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("outline_panel.dock"),
                        pick: |settings_content| {
                            settings_content.outline_panel.as_ref()?.dock.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.outline_panel.get_or_insert_default().dock = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "アウトラインパネルのデフォルト幅",
                    description: "アウトラインパネルのデフォルトの幅（ピクセル単位）です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("outline_panel.default_width"),
                        pick: |settings_content| {
                            settings_content
                                .outline_panel
                                .as_ref()?
                                .default_width
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .outline_panel
                                .get_or_insert_default()
                                .default_width = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "ファイルアイコン",
                    description: "アウトラインパネルにファイルアイコンを表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("outline_panel.file_icons"),
                        pick: |settings_content| {
                            settings_content.outline_panel.as_ref()?.file_icons.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .outline_panel
                                .get_or_insert_default()
                                .file_icons = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "フォルダーアイコン",
                    description: "アウトラインパネルのフォルダーに対してアイコンを表示するかシェブロンを表示するかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("outline_panel.folder_icons"),
                        pick: |settings_content| {
                            settings_content
                                .outline_panel
                                .as_ref()?
                                .folder_icons
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .outline_panel
                                .get_or_insert_default()
                                .folder_icons = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Gitステータス",
                    description: "アウトラインパネルにGitのステータスを表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("outline_panel.git_status"),
                        pick: |settings_content| {
                            settings_content.outline_panel.as_ref()?.git_status.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .outline_panel
                                .get_or_insert_default()
                                .git_status = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "インデント幅",
                    description: "階層化された項目のインデント幅です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("outline_panel.indent_size"),
                        pick: |settings_content| {
                            settings_content
                                .outline_panel
                                .as_ref()?
                                .indent_size
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .outline_panel
                                .get_or_insert_default()
                                .indent_size = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "項目の自動展開表示",
                    description: "対応するアウトライン項目がアクティブになったときに自動的に展開して表示するかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("outline_panel.auto_reveal_entries"),
                        pick: |settings_content| {
                            settings_content
                                .outline_panel
                                .as_ref()?
                                .auto_reveal_entries
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .outline_panel
                                .get_or_insert_default()
                                .auto_reveal_entries = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "フォルダーの自動折りたたみ",
                    description: "フォルダー内にサブフォルダーが1つしかない場合に自動的に折りたたむかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("outline_panel.auto_fold_dirs"),
                        pick: |settings_content| {
                            settings_content
                                .outline_panel
                                .as_ref()?
                                .auto_fold_dirs
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .outline_panel
                                .get_or_insert_default()
                                .auto_fold_dirs = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    files: USER,
                    title: "インデントガイドを表示",
                    description: "アウトラインパネルにインデントガイドを表示するタイミングを設定します。",
                    aliases: None,
                    field: Box::new(
                        SettingField {
                            json_path: Some("outline_panel.indent_guides.show"),
                            pick: |settings_content| {
                                settings_content
                                    .outline_panel
                                    .as_ref()?
                                    .indent_guides
                                    .as_ref()?
                                    .show
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .outline_panel
                                    .get_or_insert_default()
                                    .indent_guides
                                    .get_or_insert_default()
                                    .show = value;
                            },
                        }
                    ),
                    metadata: None,
                }),
                SettingsPageItem::SectionHeader("Gitパネル"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Gitパネルボタン",
                    description: "ステータスバーにGitパネルボタンを表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("git_panel.button"),
                        pick: |settings_content| {
                            settings_content.git_panel.as_ref()?.button.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.git_panel.get_or_insert_default().button = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Gitパネルのドック位置",
                    description: "Gitパネルをドックする位置です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("git_panel.dock"),
                        pick: |settings_content| settings_content.git_panel.as_ref()?.dock.as_ref(),
                        write: |settings_content, value| {
                            settings_content.git_panel.get_or_insert_default().dock = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Gitパネルのデフォルト幅",
                    description: "Gitパネルのデフォルトの幅（ピクセル単位）です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("git_panel.default_width"),
                        pick: |settings_content| {
                            settings_content.git_panel.as_ref()?.default_width.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .git_panel
                                .get_or_insert_default()
                                .default_width = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "Gitパネルのステータススタイル",
                    description: "項目のステータスをどのように表示するかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("git_panel.status_style"),
                        pick: |settings_content| {
                            settings_content.git_panel.as_ref()?.status_style.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .git_panel
                                .get_or_insert_default()
                                .status_style = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "フォールバックブランチ名",
                    description: "Gitのinit.defaultbranchが未設定の場合に使用するデフォルトのブランチ名です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("git_panel.fallback_branch_name"),
                        pick: |settings_content| {
                            settings_content
                                .git_panel
                                .as_ref()?
                                .fallback_branch_name
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .git_panel
                                .get_or_insert_default()
                                .fallback_branch_name = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "パス順で並び替え",
                    description: "有効にするとパネル内の項目をパス順で並び替えます。無効にするとステータス順で並び替えます。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("git_panel.sort_by_path"),
                        pick: |settings_content| {
                            settings_content.git_panel.as_ref()?.sort_by_path.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .git_panel
                                .get_or_insert_default()
                                .sort_by_path = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "差分統計",
                    description: "各項目の追加行数および削除行数を表示するかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("git_panel.diff_stats"),
                        pick: |settings_content| settings_content.git_panel.as_ref()?.diff_stats.as_ref(),
                        write: |settings_content, value| {
                            settings_content.git_panel.get_or_insert_default().diff_stats = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "未追跡の差分を折りたたむ",
                    description: "差分パネルで未追跡ファイルを折りたたんで表示するかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("git_panel.collapse_untracked_diff"),
                        pick: |settings_content| {
                            settings_content
                                .git_panel
                                .as_ref()?
                                .collapse_untracked_diff
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .git_panel
                                .get_or_insert_default()
                                .collapse_untracked_diff = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "ツリー表示",
                    description: "有効にすると項目をツリー形式で一覧表示し、無効にするとフラットなリスト形式で表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("git_panel.tree_view"),
                        pick: |settings_content| {
                            settings_content.git_panel.as_ref()?.tree_view.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .git_panel
                                .get_or_insert_default()
                                .tree_view = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "スクロールバー",
                    description: "スクロールバーを表示する方法とタイミングを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("git_panel.scrollbar.show"),
                        pick: |settings_content| {
                            show_scrollbar_or_editor(settings_content, |settings_content| {
                                settings_content
                                    .git_panel
                                    .as_ref()?
                                    .scrollbar
                                    .as_ref()?
                                    .show
                                    .as_ref()
                            })
                        },
                        write: |settings_content, value| {
                            settings_content
                                .git_panel
                                .get_or_insert_default()
                                .scrollbar
                                .get_or_insert_default()
                                .show = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("デバッガーパネル"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "デバッガーパネルのドック位置",
                    description: "デバッガーパネルのドック位置です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("debugger.dock"),
                        pick: |settings_content| settings_content.debugger.as_ref()?.dock.as_ref(),
                        write: |settings_content, value| {
                            settings_content.debugger.get_or_insert_default().dock = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
            ],
        },
        SettingsPage {
            title: "デバッガー",
            items: vec![
                SettingsPageItem::SectionHeader("一般"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "ステップ実行の粒度",
                    description: "デバッグ操作時のステップ実行の細かさを決定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("debugger.stepping_granularity"),
                        pick: |settings_content| {
                            settings_content
                                .debugger
                                .as_ref()?
                                .stepping_granularity
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .debugger
                                .get_or_insert_default()
                                .stepping_granularity = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "ブレークポイントを保存",
                    description: "Gramのセッション間でブレークポイントを保持して再利用するかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("debugger.save_breakpoints"),
                        pick: |settings_content| {
                            settings_content
                                .debugger
                                .as_ref()?
                                .save_breakpoints
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .debugger
                                .get_or_insert_default()
                                .save_breakpoints = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "タイムアウト",
                    description: "TCPデバッグアダプターへの接続時にタイムアウトエラーとなるまでの時間（ミリ秒）です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("debugger.timeout"),
                        pick: |settings_content| {
                            settings_content.debugger.as_ref()?.timeout.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.debugger.get_or_insert_default().timeout = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "DAP通信のログ記録",
                    description: "アクティブなデバッグアダプターとGramの間のメッセージをログに記録するかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("debugger.log_dap_communications"),
                        pick: |settings_content| {
                            settings_content
                                .debugger
                                .as_ref()?
                                .log_dap_communications
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .debugger
                                .get_or_insert_default()
                                .log_dap_communications = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "DAPログメッセージの整形",
                    description: "デバッグアダプターロガーに追加する際にDAPメッセージを整形（フォーマット）するかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("debugger.format_dap_log_messages"),
                        pick: |settings_content| {
                            settings_content
                                .debugger
                                .as_ref()?
                                .format_dap_log_messages
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .debugger
                                .get_or_insert_default()
                                .format_dap_log_messages = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
            ],
        },
        SettingsPage {
            title: "ターミナル",
            items: vec![
                SettingsPageItem::SectionHeader("環境"),
                SettingsPageItem::DynamicItem(DynamicItem {
                    discriminant: SettingItem {
                        files: USER | PROJECT,
                        title: "シェル",
                        description: "ターミナルを開く際に使用するシェルを設定します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("terminal.shell$"),
                            pick: |settings_content| {
                                Some(&dynamic_variants::<settings::Shell>()[
                                    settings_content
                                        .terminal
                                        .as_ref()?
                                        .project
                                        .shell
                                        .as_ref()?
                                        .discriminant() as usize])
                            },
                            write: |settings_content, value| {
                                let Some(value) = value else {
                                    if let Some(terminal) = settings_content.terminal.as_mut() {
                                        terminal.project.shell = None;
                                    }
                                    return;
                                };
                                let settings_value = settings_content
                                    .terminal
                                    .get_or_insert_default()
                                    .project
                                    .shell
                                    .get_or_insert_with(settings::Shell::default);
                                let default_shell = if cfg!(target_os = "windows") {
                                    "powershell.exe"
                                } else {
                                    "sh"
                                };
                                *settings_value = match value {
                                    settings::ShellDiscriminants::System => {
                                        settings::Shell::System
                                    },
                                    settings::ShellDiscriminants::Program => {
                                        let program = match settings_value {
                                            settings::Shell::Program(p) => p.clone(),
                                            settings::Shell::WithArguments { program, .. } => program.clone(),
                                            _ => String::from(default_shell),
                                        };
                                        settings::Shell::Program(program)
                                    },
                                    settings::ShellDiscriminants::WithArguments => {
                                        let (program, args, title_override) = match settings_value {
                                            settings::Shell::Program(p) => (p.clone(), vec![], None),
                                            settings::Shell::WithArguments { program, args, title_override } => {
                                                (program.clone(), args.clone(), title_override.clone())
                                            },
                                            _ => (String::from(default_shell), vec![], None),
                                        };
                                        settings::Shell::WithArguments {
                                            program,
                                            args,
                                            title_override,
                                        }
                                    },
                                };
                            },
                        }),
                        metadata: None,
                    },
                    pick_discriminant: |settings_content| {
                        Some(settings_content.terminal.as_ref()?.project.shell.as_ref()?.discriminant() as usize)
                    },
                    fields: dynamic_variants::<settings::Shell>().iter().map(|variant| {
                        match variant {
                            settings::ShellDiscriminants::System => vec![],
                            settings::ShellDiscriminants::Program => vec![
                                SettingItem {
                                    files: USER | PROJECT,
                                    title: "プログラム",
                                    description: "使用するシェルプログラムです。",
                                    aliases: None,
                                    field: Box::new(SettingField {
                                        json_path: Some("terminal.shell"),
                                        pick: |settings_content| {
                                            match settings_content.terminal.as_ref()?.project.shell.as_ref() {
                                                Some(settings::Shell::Program(program)) => Some(program),
                                                _ => None
                                            }
                                        },
                                        write: |settings_content, value| {
                                            let Some(value) = value else {
                                                return;
                                            };
                                            if let Some(settings::Shell::Program(program)) = settings_content
                                                .terminal
                                                .get_or_insert_default()
                                                .project
                                                .shell.as_mut() { *program = value }
                                        },
                                    }),
                                    metadata: None,
                                }
                            ],
                            settings::ShellDiscriminants::WithArguments => vec![
                                SettingItem {
                                    files: USER | PROJECT,
                                    title: "プログラム",
                                    description: "実行するシェルプログラムです。",
                                    aliases: None,
                                    field: Box::new(SettingField {
                                        json_path: Some("terminal.shell.program"),
                                        pick: |settings_content| {
                                            match settings_content.terminal.as_ref()?.project.shell.as_ref() {
                                                Some(settings::Shell::WithArguments { program, .. }) => Some(program),
                                                _ => None
                                            }
                                        },
                                        write: |settings_content, value| {
                                            let Some(value) = value else {
                                                return;
                                            };
                                            if let Some(settings::Shell::WithArguments { program, .. }) = settings_content
                                                .terminal
                                                .get_or_insert_default()
                                                .project
                                                .shell.as_mut() { *program = value }
                                        },
                                    }),
                                    metadata: None,
                                },
                                SettingItem {
                                    files: USER | PROJECT,
                                    title: "引数",
                                    description: "シェルプログラムに渡す引数です。",
                                    aliases: None,
                                    field: Box::new(
                                        SettingField {
                                            json_path: Some("terminal.shell.args"),
                                            pick: |settings_content| {
                                                match settings_content.terminal.as_ref()?.project.shell.as_ref() {
                                                    Some(settings::Shell::WithArguments { args, .. }) => Some(args),
                                                    _ => None
                                                }
                                            },
                                            write: |settings_content, value| {
                                                let Some(value) = value else {
                                                    return;
                                                };
                                                if let Some(settings::Shell::WithArguments { args, .. }) = settings_content
                                                    .terminal
                                                    .get_or_insert_default()
                                                    .project
                                                    .shell.as_mut() { *args = value }
                                            },
                                        }
                                        .unimplemented(),
                                    ),
                                    metadata: None,
                                },
                                SettingItem {
                                    files: USER | PROJECT,
                                    title: "タイトルの上書き",
                                    description: "ターミナルタブのタイトルを上書きするための任意の文字列です。",
                                    aliases: None,
                                    field: Box::new(SettingField {
                                        json_path: Some("terminal.shell.title_override"),
                                        pick: |settings_content| {
                                            match settings_content.terminal.as_ref()?.project.shell.as_ref() {
                                                Some(settings::Shell::WithArguments { title_override, .. }) => title_override.as_ref().or(DEFAULT_EMPTY_SHARED_STRING),
                                                _ => None
                                            }
                                        },
                                        write: |settings_content, value| {
                                            if let Some(settings::Shell::WithArguments { title_override, .. }) = settings_content
                                                .terminal
                                                .get_or_insert_default()
                                                .project
                                                .shell.as_mut() { *title_override = value.filter(|s| !s.is_empty()) }
                                        },
                                    }),
                                    metadata: None,
                                }
                            ],
                        }
                    }).collect(),
                }),
                SettingsPageItem::DynamicItem(DynamicItem {
                    discriminant: SettingItem {
                        files: USER | PROJECT,
                        title: "作業ディレクトリ",
                        description: "ターミナル起動時に使用する作業ディレクトリを設定します。",
                        aliases: None,
                        field: Box::new(SettingField {
                            json_path: Some("terminal.working_directory$"),
                            pick: |settings_content| {
                                Some(&dynamic_variants::<settings::WorkingDirectory>()[
                                    settings_content
                                        .terminal
                                        .as_ref()?
                                        .project
                                        .working_directory
                                        .as_ref()?
                                        .discriminant() as usize])
                            },
                            write: |settings_content, value| {
                                let Some(value) = value else {
                                    if let Some(terminal) = settings_content.terminal.as_mut() {
                                        terminal.project.working_directory = None;
                                    }
                                    return;
                                };
                                let settings_value = settings_content
                                    .terminal
                                    .get_or_insert_default()
                                    .project
                                    .working_directory
                                    .get_or_insert(settings::WorkingDirectory::CurrentProjectDirectory);
                                *settings_value = match value {
                                    settings::WorkingDirectoryDiscriminants::CurrentProjectDirectory => {
                                        settings::WorkingDirectory::CurrentProjectDirectory
                                    },
                                    settings::WorkingDirectoryDiscriminants::FirstProjectDirectory => {
                                        settings::WorkingDirectory::FirstProjectDirectory
                                    },
                                    settings::WorkingDirectoryDiscriminants::AlwaysHome => {
                                        settings::WorkingDirectory::AlwaysHome
                                    },
                                    settings::WorkingDirectoryDiscriminants::Always => {
                                        let directory = match settings_value {
                                            settings::WorkingDirectory::Always { .. } => return,
                                            _ => String::new(),
                                        };
                                        settings::WorkingDirectory::Always { directory }
                                    },
                                };
                            },
                        }),
                        metadata: None,
                    },
                    pick_discriminant: |settings_content| {
                        Some(settings_content.terminal.as_ref()?.project.working_directory.as_ref()?.discriminant() as usize)
                    },
                    fields: dynamic_variants::<settings::WorkingDirectory>().iter().map(|variant| {
                        match variant {
                            settings::WorkingDirectoryDiscriminants::CurrentProjectDirectory => vec![],
                            settings::WorkingDirectoryDiscriminants::FirstProjectDirectory => vec![],
                            settings::WorkingDirectoryDiscriminants::AlwaysHome => vec![],
                            settings::WorkingDirectoryDiscriminants::Always => vec![
                                SettingItem {
                                    files: USER | PROJECT,
                                    title: "ディレクトリ",
                                    description: "使用するディレクトリパスです（シェル展開されます）。",
                                    aliases: None,
                                    field: Box::new(SettingField {
                                        json_path: Some("terminal.working_directory.always"),
                                        pick: |settings_content| {
                                            match settings_content.terminal.as_ref()?.project.working_directory.as_ref() {
                                                Some(settings::WorkingDirectory::Always { directory }) => Some(directory),
                                                _ => None
                                            }
                                        },
                                        write: |settings_content, value| {
                                            let value = value.unwrap_or_default();
                                            if let Some(settings::WorkingDirectory::Always { directory }) = settings_content
                                                .terminal
                                                .get_or_insert_default()
                                                .project
                                                .working_directory.as_mut() { *directory = value }
                                        },
                                    }),
                                    metadata: None,
                                }
                            ],
                        }
                    }).collect(),
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "環境変数",
                    description: "ターミナルの環境に追加するキーと値のペアです。",
                    aliases: None,
                    field: Box::new(
                        SettingField {
                            json_path: Some("terminal.env"),
                            pick: |settings_content| {
                                settings_content.terminal.as_ref()?.project.env.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .terminal
                                    .get_or_insert_default()
                                    .project
                                    .env = value;
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                    files: USER | PROJECT,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "仮想環境の検出",
                    description: "ターミナルの作業ディレクトリ内にPython仮想環境が見つかった場合、自動的に有効化します。",
                    aliases: None,
                    field: Box::new(
                        SettingField {
                            json_path: Some("terminal.detect_venv"),
                            pick: |settings_content| {
                                settings_content
                                    .terminal
                                    .as_ref()?
                                    .project
                                    .detect_venv
                                    .as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .terminal
                                    .get_or_insert_default()
                                    .project
                                    .detect_venv = value;
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                    files: USER | PROJECT,
                }),
                SettingsPageItem::SectionHeader("フォント"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "フォントサイズ",
                    description: "ターミナルテキストのフォントサイズです。未設定の場合はバッファーフォントサイズを継承します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("terminal.font_size"),
                        pick: |settings_content| {
                            settings_content
                                .terminal
                                .as_ref()
                                .and_then(|terminal| terminal.font_size.as_ref())
                                .or(settings_content.theme.buffer_font_size.as_ref())
                        },
                        write: |settings_content, value| {
                            settings_content.terminal.get_or_insert_default().font_size = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "フォントファミリー",
                    description: "ターミナルテキストのフォントファミリーです。未設定の場合はバッファーフォントファミリーを継承します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("terminal.font_family"),
                        pick: |settings_content| {
                            settings_content
                                .terminal
                                .as_ref()
                                .and_then(|terminal| terminal.font_family.as_ref())
                                .or(settings_content.theme.buffer_font_family.as_ref())
                        },
                        write: |settings_content, value| {
                            settings_content
                                .terminal
                                .get_or_insert_default()
                                .font_family = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "フォールバックフォント",
                    description: "ターミナルテキストのフォールバックフォントです。未設定の場合はバッファーフォールバックフォントを継承します。",
                    aliases: None,
                    field: Box::new(
                        SettingField {
                            json_path: Some("terminal.font_fallbacks"),
                            pick: |settings_content| {
                                settings_content
                                    .terminal
                                    .as_ref()
                                    .and_then(|terminal| terminal.font_fallbacks.as_ref())
                                    .or(settings_content.theme.buffer_font_fallbacks.as_ref())
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .terminal
                                    .get_or_insert_default()
                                    .font_fallbacks = value;
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "フォントウェイト",
                    description: "CSSウェイト単位（100-900）でのターミナルテキストのフォントの太さです。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("terminal.font_weight"),
                        pick: |settings_content| {
                            settings_content.terminal.as_ref()?.font_weight.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .terminal
                                .get_or_insert_default()
                                .font_weight = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "フォント機能",
                    description: "ターミナルテキスト用のOpenType機能です。",
                    aliases: None,
                    field: Box::new(
                        SettingField {
                            json_path: Some("terminal.font_features"),
                            pick: |settings_content| {
                                settings_content
                                    .terminal
                                    .as_ref()
                                    .and_then(|terminal| terminal.font_features.as_ref())
                                    .or(settings_content.theme.buffer_font_features.as_ref())
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .terminal
                                    .get_or_insert_default()
                                    .font_features = value;
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("表示設定"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "行の高さ",
                    description: "ターミナルテキストの行の高さです。",
                    aliases: None,
                    field: Box::new(
                        SettingField {
                            json_path: Some("terminal.line_height"),
                            pick: |settings_content| {
                                settings_content.terminal.as_ref()?.line_height.as_ref()
                            },
                            write: |settings_content, value| {
                                settings_content
                                    .terminal
                                    .get_or_insert_default()
                                    .line_height = value;
                            },
                        }
                        .unimplemented(),
                    ),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "カーソル形状",
                    description: "ターミナルのデフォルトのカーソル形状（バー、ブロック、下線、中空）です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("terminal.cursor_shape"),
                        pick: |settings_content| {
                            settings_content.terminal.as_ref()?.cursor_shape.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .terminal
                                .get_or_insert_default()
                                .cursor_shape = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "カーソルの点滅",
                    description: "ターミナル内でのカーソル点滅動作を設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("terminal.blinking"),
                        pick: |settings_content| {
                            settings_content.terminal.as_ref()?.blinking.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.terminal.get_or_insert_default().blinking = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "オルタネートスクロール",
                    description: "デフォルトでオルタネートスクロールモードを有効にするかどうか（Vim等のアプリでマウススクロールを矢印キーに変換）を設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("terminal.alternate_scroll"),
                        pick: |settings_content| {
                            settings_content
                                .terminal
                                .as_ref()?
                                .alternate_scroll
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .terminal
                                .get_or_insert_default()
                                .alternate_scroll = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "最小コントラスト",
                    description: "前景色と背景色の間のAPCA知覚コントラストの最小値（0-106）です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("terminal.minimum_contrast"),
                        pick: |settings_content| {
                            settings_content
                                .terminal
                                .as_ref()?
                                .minimum_contrast
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .terminal
                                .get_or_insert_default()
                                .minimum_contrast = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("動作設定"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "OptionをMetaとして扱う",
                    description: "OptionキーをMetaキーとして動作させるかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("terminal.option_as_meta"),
                        pick: |settings_content| {
                            settings_content.terminal.as_ref()?.option_as_meta.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .terminal
                                .get_or_insert_default()
                                .option_as_meta = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "選択時にコピー",
                    description: "ターミナル内でテキストを選択したときに自動的にクリップボードへコピーするかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("terminal.copy_on_select"),
                        pick: |settings_content| {
                            settings_content.terminal.as_ref()?.copy_on_select.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .terminal
                                .get_or_insert_default()
                                .copy_on_select = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "コピー後も選択を維持",
                    description: "クリップボードへのコピー後もテキストの選択状態を維持するかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("terminal.keep_selection_on_copy"),
                        pick: |settings_content| {
                            settings_content
                                .terminal
                                .as_ref()?
                                .keep_selection_on_copy
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .terminal
                                .get_or_insert_default()
                                .keep_selection_on_copy = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("レイアウト設定"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "デフォルト幅",
                    description: "ターミナルが左右にドックされている場合のデフォルトの幅（ピクセル単位）です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("terminal.default_width"),
                        pick: |settings_content| {
                            settings_content.terminal.as_ref()?.default_width.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .terminal
                                .get_or_insert_default()
                                .default_width = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "デフォルトの高さ",
                    description: "ターミナルが下部にドックされている場合のデフォルトの高さ（ピクセル単位）です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("terminal.default_height"),
                        pick: |settings_content| {
                            settings_content.terminal.as_ref()?.default_height.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .terminal
                                .get_or_insert_default()
                                .default_height = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("詳細設定"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "最大スクロール履歴行数",
                    description: "スクロールバック履歴として保持する最大行数です（最大: 100,000、0でスクロール無効）。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("terminal.max_scroll_history_lines"),
                        pick: |settings_content| {
                            settings_content
                                .terminal
                                .as_ref()?
                                .max_scroll_history_lines
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .terminal
                                .get_or_insert_default()
                                .max_scroll_history_lines = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "スクロール倍率",
                    description: "マウスホイールによるターミナル内スクロールの感度倍率です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("terminal.scroll_multiplier"),
                        pick: |settings_content| {
                            settings_content.terminal.as_ref()?.scroll_multiplier.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .terminal
                                .get_or_insert_default()
                                .scroll_multiplier = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("ツールバー"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "パンくずリスト",
                    description: "ターミナルペイン内のパンくずリストにターミナルのタイトルを表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("terminal.toolbar.breadcrumbs"),
                        pick: |settings_content| {
                            settings_content
                                .terminal
                                .as_ref()?
                                .toolbar
                                .as_ref()?
                                .breadcrumbs
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .terminal
                                .get_or_insert_default()
                                .toolbar
                                .get_or_insert_default()
                                .breadcrumbs = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("スクロールバー"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "スクロールバーを表示",
                    description: "ターミナルでスクロールバーを表示するタイミングを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("terminal.scrollbar.show"),
                        pick: |settings_content| {
                            show_scrollbar_or_editor(settings_content, |settings_content| {
                                settings_content
                                    .terminal
                                    .as_ref()?
                                    .scrollbar
                                    .as_ref()?
                                    .show
                                    .as_ref()
                            })
                        },
                        write: |settings_content, value| {
                            settings_content
                                .terminal
                                .get_or_insert_default()
                                .scrollbar
                                .get_or_insert_default()
                                .show = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
            ],
        },
        SettingsPage {
            title: "バージョン管理",
            items: vec![
                SettingsPageItem::SectionHeader("差分"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "左右並列の差分表示",
                    description: "リポジトリの差分、コミット、ファイル比較を2つのパネルに並べて開きます。",
                    aliases: Some(&["Split Diff"]),
                    field: Box::new(SettingField {
                        json_path: Some("git.split_diff"),
                        pick: |content| content.git.as_ref()?.split_diff.as_ref(),
                        write: |content, value| content.git.get_or_insert_default().split_diff = value,
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Gitガター"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "表示",
                    description: "エディターのガターにGitステータスを表示するかどうかを制御します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("git.git_gutter"),
                        pick: |settings_content| settings_content.git.as_ref()?.git_gutter.as_ref(),
                        write: |settings_content, value| {
                            settings_content.git.get_or_insert_default().git_gutter = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                // todo(settings_ui): Figure out the right default for this value in default.jsonc
                SettingsPageItem::SettingItem(SettingItem {
                    title: "デバウンス",
                    description: "変更がGitガターに反映されるまでのデバウンスしきい値（ミリ秒）です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("git.gutter_debounce"),
                        pick: |settings_content| {
                            settings_content.git.as_ref()?.gutter_debounce.as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content.git.get_or_insert_default().gutter_debounce = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("インラインGit Blame"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "有効",
                    description: "現在フォーカスされている行にGit blameの情報をインライン表示するかどうかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("git.inline_blame.enabled"),
                        pick: |settings_content| {
                            settings_content
                                .git
                                .as_ref()?
                                .inline_blame
                                .as_ref()?
                                .enabled
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .git
                                .get_or_insert_default()
                                .inline_blame
                                .get_or_insert_default()
                                .enabled = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "遅延時間",
                    description: "インラインblame情報が表示されるまでの遅延時間です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("git.inline_blame.delay_ms"),
                        pick: |settings_content| {
                            settings_content
                                .git
                                .as_ref()?
                                .inline_blame
                                .as_ref()?
                                .delay_ms
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .git
                                .get_or_insert_default()
                                .inline_blame
                                .get_or_insert_default()
                                .delay_ms = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "余白",
                    description: "ソースコード行末とインラインblame開始位置との間の余白列数です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("git.inline_blame.padding"),
                        pick: |settings_content| {
                            settings_content
                                .git
                                .as_ref()?
                                .inline_blame
                                .as_ref()?
                                .padding
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .git
                                .get_or_insert_default()
                                .inline_blame
                                .get_or_insert_default()
                                .padding = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "最小列",
                    description: "インラインblame情報を表示する最小列番号です。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("git.inline_blame.min_column"),
                        pick: |settings_content| {
                            settings_content
                                .git
                                .as_ref()?
                                .inline_blame
                                .as_ref()?
                                .min_column
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .git
                                .get_or_insert_default()
                                .inline_blame
                                .get_or_insert_default()
                                .min_column = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "コミットサマリーを表示",
                    description: "インラインblameの一部としてコミットの概要を表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("git.inline_blame.show_commit_summary"),
                        pick: |settings_content| {
                            settings_content
                                .git
                                .as_ref()?
                                .inline_blame
                                .as_ref()?
                                .show_commit_summary
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .git
                                .get_or_insert_default()
                                .inline_blame
                                .get_or_insert_default()
                                .show_commit_summary = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Git Blameビュー"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "アバターを表示",
                    description: "コミット作成者のアバター画像を表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("git.blame.show_avatar"),
                        pick: |settings_content| {
                            settings_content
                                .git
                                .as_ref()?
                                .blame
                                .as_ref()?
                                .show_avatar
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .git
                                .get_or_insert_default()
                                .blame
                                .get_or_insert_default()
                                .show_avatar = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("ブランチピッカー"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "作成者名を表示",
                    description: "ブランチピッカー内のコミット情報の一部として作成者名を表示します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("git.branch_picker.show_author_name"),
                        pick: |settings_content| {
                            settings_content
                                .git
                                .as_ref()?
                                .branch_picker
                                .as_ref()?
                                .show_author_name
                                .as_ref()
                        },
                        write: |settings_content, value| {
                            settings_content
                                .git
                                .get_or_insert_default()
                                .branch_picker
                                .get_or_insert_default()
                                .show_author_name = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SectionHeader("Gitハンク"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "ハンクスタイル",
                    description: "エディター内でGitハンクをどのように視覚表示するかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("git.hunk_style"),
                        pick: |settings_content| settings_content.git.as_ref()?.hunk_style.as_ref(),
                        write: |settings_content, value| {
                            settings_content.git.get_or_insert_default().hunk_style = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "パススタイル",
                    description: "Gitビューで名前とパスのどちらを先に表示するかを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("git.path_style"),
                        pick: |settings_content| settings_content.git.as_ref()?.path_style.as_ref(),
                        write: |settings_content, value| {
                            settings_content.git.get_or_insert_default().path_style = value;
                        },
                    }),
                    metadata: None,
                    files: USER,
                }),
            ],
        },
        SettingsPage {
            title: "ネットワーク",
            items: vec![
                SettingsPageItem::SectionHeader("ネットワーク"),
                SettingsPageItem::SettingItem(SettingItem {
                    title: "プロキシ",
                    description: "ネットワークリクエストに使用するプロキシを設定します。",
                    aliases: None,
                    field: Box::new(SettingField {
                        json_path: Some("proxy"),
                        pick: |settings_content| settings_content.proxy.as_ref(),
                        write: |settings_content, value| {
                            settings_content.proxy = value;
                        },
                    }),
                    metadata: Some(Box::new(SettingsFieldMetadata {
                        placeholder: Some("socks5h://localhost:10808"),
                        ..Default::default()
                    })),
                    files: USER,
                }),
            ],
        },
    ]
}

const LANGUAGES_SECTION_HEADER: &str = "言語";

fn current_language() -> Option<SharedString> {
    sub_page_stack()
        .iter()
        .find_map(|page| (page.section_header == LANGUAGES_SECTION_HEADER).then(|| page.link.title.clone()))
}

fn language_settings_field<T>(
    settings_content: &SettingsContent,
    get: fn(&LanguageSettingsContent) -> Option<&T>,
) -> Option<&T> {
    let all_languages = &settings_content.project.all_languages;
    if let Some(current_language_name) = current_language()
        && let Some(current_language) = all_languages.languages.0.get(&current_language_name)
    {
        let value = get(current_language);
        if value.is_some() {
            return value;
        }
    }

    get(&all_languages.defaults)
}

fn language_settings_field_mut<T>(
    settings_content: &mut SettingsContent,
    value: Option<T>,
    write: fn(&mut LanguageSettingsContent, Option<T>),
) {
    let all_languages = &mut settings_content.project.all_languages;
    let language_content = if let Some(current_language) = current_language() {
        all_languages.languages.0.entry(current_language).or_default()
    } else {
        &mut all_languages.defaults
    };
    write(language_content, value);
}

fn language_settings_data() -> Vec<SettingsPageItem> {
    let mut items = vec![
        SettingsPageItem::SectionHeader("インデント"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "タブ幅",
            description: "タブが占める文字幅（列数）です。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).tab_size"), // TODO(cameron): not JQ syntax because not URL-safe
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| language.tab_size.as_ref())
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.tab_size = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "ハードタブ",
            description: "スペースの代わりにタブ文字を使用して行をインデントするかどうかを設定します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).hard_tabs"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| language.hard_tabs.as_ref())
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.hard_tabs = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "自動インデント",
            description: "タイピング中のコンテキストに基づいてインデントを自動調整するかどうかを設定します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).auto_indent"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| language.auto_indent.as_ref())
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.auto_indent = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "貼り付け時の自動インデント",
            description: "貼り付けた内容のインデントをコンテキストに基づいて自動調整するかどうかを設定します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).auto_indent_on_paste"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| language.auto_indent_on_paste.as_ref())
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.auto_indent_on_paste = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SectionHeader("折り返し"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "折り返し",
            description: "長いテキスト行の折り返し方法を設定します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).soft_wrap"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| language.soft_wrap.as_ref())
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.soft_wrap = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "折り返しガイドを表示",
            description: "エディターに折り返しガイドを表示します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).show_wrap_guides"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| language.show_wrap_guides.as_ref())
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.show_wrap_guides = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "推奨行長",
            description: "折り返しが有効なバッファーで、行を折り返す列位置です。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).preferred_line_length"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| language.preferred_line_length.as_ref())
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.preferred_line_length = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "折り返しガイド",
            description: "エディターで折り返しガイドを表示する文字数列を指定します。",
            aliases: None,
            field: Box::new(
                SettingField {
                    json_path: Some("languages.$(language).wrap_guides"),
                    pick: |settings_content| {
                        language_settings_field(settings_content, |language| language.wrap_guides.as_ref())
                    },
                    write: |settings_content, value| {
                        language_settings_field_mut(settings_content, value, |language, value| {
                            language.wrap_guides = value;
                        })
                    },
                }
                .unimplemented(),
            ),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "再折り返しを許可",
            description: "この言語で`editor::rewrap`アクションを許可する箇所を制御します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).allow_rewrap"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| language.allow_rewrap.as_ref())
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.allow_rewrap = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SectionHeader("インデントガイド"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "有効",
            description: "エディターにインデントガイドを表示します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).indent_guides.enabled"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language
                            .indent_guides
                            .as_ref()
                            .and_then(|indent_guides| indent_guides.enabled.as_ref())
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.indent_guides.get_or_insert_default().enabled = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "線の太さ",
            description: "インデントガイドの太さ（1〜10ピクセル）です。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).indent_guides.line_width"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language
                            .indent_guides
                            .as_ref()
                            .and_then(|indent_guides| indent_guides.line_width.as_ref())
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.indent_guides.get_or_insert_default().line_width = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "アクティブ時の線の太さ",
            description: "アクティブなインデントガイドの太さ（1〜10ピクセル）です。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).indent_guides.active_line_width"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language
                            .indent_guides
                            .as_ref()
                            .and_then(|indent_guides| indent_guides.active_line_width.as_ref())
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.indent_guides.get_or_insert_default().active_line_width = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "配色",
            description: "インデントガイドの着色方法を設定します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).indent_guides.coloring"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language
                            .indent_guides
                            .as_ref()
                            .and_then(|indent_guides| indent_guides.coloring.as_ref())
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.indent_guides.get_or_insert_default().coloring = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "背景の配色",
            description: "インデントガイドの背景色の着色方法を設定します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).indent_guides.background_coloring"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language
                            .indent_guides
                            .as_ref()
                            .and_then(|indent_guides| indent_guides.background_coloring.as_ref())
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.indent_guides.get_or_insert_default().background_coloring = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SectionHeader("フォーマット"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "保存時にフォーマット",
            description: "保存前にバッファーのフォーマットを実行するかどうかを設定します。",
            aliases: None,
            field: Box::new(
                // TODO(settings_ui): this setting should just be a bool
                SettingField {
                    json_path: Some("languages.$(language).format_on_save"),
                    pick: |settings_content| {
                        language_settings_field(settings_content, |language| language.format_on_save.as_ref())
                    },
                    write: |settings_content, value| {
                        language_settings_field_mut(settings_content, value, |language, value| {
                            language.format_on_save = value;
                        })
                    },
                },
            ),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "保存時に末尾の空白を削除",
            description: "バッファーの保存前に行末の余分な空白文字を削除するかどうかを設定します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).remove_trailing_whitespace_on_save"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.remove_trailing_whitespace_on_save.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.remove_trailing_whitespace_on_save = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "保存時に末尾に改行を確保",
            description: "バッファーの保存時に末尾に必ず1つの改行がある状態にするかどうかを設定します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).ensure_final_newline_on_save"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.ensure_final_newline_on_save.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.ensure_final_newline_on_save = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "フォーマッター",
            description: "バッファーのフォーマットを実行する方法を設定します。",
            aliases: None,
            field: Box::new(
                SettingField {
                    json_path: Some("languages.$(language).formatter"),
                    pick: |settings_content| {
                        language_settings_field(settings_content, |language| language.formatter.as_ref())
                    },
                    write: |settings_content, value| {
                        language_settings_field_mut(settings_content, value, |language, value| {
                            language.formatter = value;
                        })
                    },
                }
                .unimplemented(),
            ),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "入力時フォーマットを使用",
            description: "LSPサーバーの機能で定義された「トリガー」記号の入力ごとに、コードのフォーマットおよび修正を行う追加LSPクエリを使用するかどうかを設定します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).use_on_type_format"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| language.use_on_type_format.as_ref())
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.use_on_type_format = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "フォーマット時のコードアクション",
            description: "フォーマット実行時に追加で実行するコードアクションです。",
            aliases: None,
            field: Box::new(
                SettingField {
                    json_path: Some("languages.$(language).code_actions_on_format"),
                    pick: |settings_content| {
                        language_settings_field(settings_content, |language| language.code_actions_on_format.as_ref())
                    },
                    write: |settings_content, value| {
                        language_settings_field_mut(settings_content, value, |language, value| {
                            language.code_actions_on_format = value;
                        })
                    },
                }
                .unimplemented(),
            ),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SectionHeader("自動クローズ"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "自動クローズを使用",
            description: "閉じ文字を自動入力するかどうかを設定します。たとえば「(」と入力した際に、正しい位置に自動で閉じ括弧「)」を追加します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).use_autoclose"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| language.use_autoclose.as_ref())
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.use_autoclose = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "自動囲みを使用",
            description: "選択したテキストを指定文字で自動的に囲むかどうかを設定します。たとえばテキストを選択して「(」と入力した際に「(...)」で囲みます。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).use_auto_surround"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| language.use_auto_surround.as_ref())
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.use_auto_surround = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "常に括弧を自動クローズとして扱う",
            description: "閉じ文字がどのように挿入されたかに関わらず、常にスキップおよび自動削除の対象として扱うかを制御します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).always_treat_brackets_as_autoclosed"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.always_treat_brackets_as_autoclosed.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.always_treat_brackets_as_autoclosed = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "JSXタグの自動クローズ",
            description: "JSXタグを自動的に閉じるかどうかを設定します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).jsx_tag_auto_close"),
                // TODO(settings_ui): this setting should just be a bool
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.jsx_tag_auto_close.as_ref()?.enabled.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.jsx_tag_auto_close.get_or_insert_default().enabled = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SectionHeader("空白文字"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "空白文字を表示",
            description: "エディター内でタブやスペースを表示するかどうかを設定します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).show_whitespaces"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| language.show_whitespaces.as_ref())
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.show_whitespaces = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "スペース空白インジケーター",
            description: "空白文字の表示が有効な場合にスペース文字の描画に使用する可視文字です（デフォルト: \"•\"）。",
            aliases: None,
            field: Box::new(
                SettingField {
                    json_path: Some("languages.$(language).whitespace_map.space"),
                    pick: |settings_content| {
                        language_settings_field(settings_content, |language| {
                            language.whitespace_map.as_ref()?.space.as_ref()
                        })
                    },
                    write: |settings_content, value| {
                        language_settings_field_mut(settings_content, value, |language, value| {
                            language.whitespace_map.get_or_insert_default().space = value;
                        })
                    },
                }
                .unimplemented(),
            ),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "タブ空白インジケーター",
            description: "空白文字の表示が有効な場合にタブ文字の描画に使用する可視文字です（デフォルト: \"→\"）。",
            aliases: None,
            field: Box::new(
                SettingField {
                    json_path: Some("languages.$(language).whitespace_map.tab"),
                    pick: |settings_content| {
                        language_settings_field(settings_content, |language| {
                            language.whitespace_map.as_ref()?.tab.as_ref()
                        })
                    },
                    write: |settings_content, value| {
                        language_settings_field_mut(settings_content, value, |language, value| {
                            language.whitespace_map.get_or_insert_default().tab = value;
                        })
                    },
                }
                .unimplemented(),
            ),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SectionHeader("補完"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "入力時に補完を表示",
            description: "明示的な要求操作を行わなくても、エディターでの入力中に補完メニューを自動表示するかどうかを設定します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).show_completions_on_input"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| language.show_completions_on_input.as_ref())
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.show_completions_on_input = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "補完ドキュメントを表示",
            description: "補完メニュー内の項目に対して、インラインおよび横並びのドキュメントを表示するかどうかを設定します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).show_completion_documentation"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.show_completion_documentation.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.show_completion_documentation = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "単語補完",
            description: "単語の補完方法を制御します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).completions.words"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.completions.as_ref()?.words.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.completions.get_or_insert_default().words = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "単語補完の最小文字数",
            description: "単語ベースの補完を自動表示するためにクエリに必要な最小文字数です。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).completions.words_min_length"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.completions.as_ref()?.words_min_length.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.completions.get_or_insert_default().words_min_length = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "補完メニューのスクロールバー",
            description: "補完メニューでスクロールバーを表示するタイミングを設定します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("editor.completion_menu_scrollbar"),
                pick: |settings_content| settings_content.editor.completion_menu_scrollbar.as_ref(),
                write: |settings_content, value| {
                    settings_content.editor.completion_menu_scrollbar = value;
                },
            }),
            metadata: None,
            files: USER,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "補完メニュー項目の種類表示",
            description: "補完メニューの各項目にLSPの種類（関数、メソッド、変数など）をどのように表示するかを設定します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("editor.completion_menu_item_kind"),
                pick: |settings_content| settings_content.editor.completion_menu_item_kind.as_ref(),
                write: |settings_content, value| {
                    settings_content.editor.completion_menu_item_kind = value;
                },
            }),
            metadata: None,
            files: USER,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "Supertabモード",
            description: "補完中でない場合にSupertabアクションが実行する動作を設定します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("editor.supertab_fallback"),
                pick: |settings_content| settings_content.editor.supertab_fallback.as_ref(),
                write: |settings_content, value| {
                    settings_content.editor.supertab_fallback = value;
                },
            }),
            metadata: None,
            files: USER,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "キルリングの同期",
            description: "キルリングをクリップボードまたはプライマリ選択と同期するかどうかを設定します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("editor.sync_kill_ring"),
                pick: |settings_content| settings_content.editor.sync_kill_ring.as_ref(),
                write: |settings_content, value| {
                    settings_content.editor.sync_kill_ring = value;
                },
            }),
            metadata: None,
            files: USER,
        }),
        SettingsPageItem::SectionHeader("インレイヒント"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "有効",
            description: "ヒント表示の有効/無効を切り替えるグローバルスイッチです。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).inlay_hints.enabled"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.inlay_hints.as_ref()?.enabled.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.inlay_hints.get_or_insert_default().enabled = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "値のヒントを表示",
            description: "デバッグ時にインライン値の表示/非表示を切り替えるグローバルスイッチです。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).inlay_hints.show_value_hints"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.inlay_hints.as_ref()?.show_value_hints.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.inlay_hints.get_or_insert_default().show_value_hints = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "型ヒントを表示",
            description: "型ヒントを表示するかどうかを設定します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).inlay_hints.show_type_hints"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.inlay_hints.as_ref()?.show_type_hints.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.inlay_hints.get_or_insert_default().show_type_hints = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "パラメーターヒントを表示",
            description: "パラメーターヒントを表示するかどうかを設定します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).inlay_hints.show_parameter_hints"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.inlay_hints.as_ref()?.show_parameter_hints.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.inlay_hints.get_or_insert_default().show_parameter_hints = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "その他のヒントを表示",
            description: "その他のヒントを表示するかどうかを設定します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).inlay_hints.show_other_hints"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.inlay_hints.as_ref()?.show_other_hints.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.inlay_hints.get_or_insert_default().show_other_hints = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "背景を表示",
            description: "インレイヒントに背景を表示します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).inlay_hints.show_background"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.inlay_hints.as_ref()?.show_background.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.inlay_hints.get_or_insert_default().show_background = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "編集デバウンス",
            description: "バッファー編集後にインレイヒントの更新をデバウンスする時間（ミリ秒）です（0でデバウンス無効）。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).inlay_hints.edit_debounce_ms"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.inlay_hints.as_ref()?.edit_debounce_ms.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.inlay_hints.get_or_insert_default().edit_debounce_ms = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "スクロールデバウンス",
            description: "バッファーのスクロール後にインレイヒントの更新をデバウンスする時間（ミリ秒）です（0でデバウンス無効）。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).inlay_hints.scroll_debounce_ms"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.inlay_hints.as_ref()?.scroll_debounce_ms.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.inlay_hints.get_or_insert_default().scroll_debounce_ms = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "修飾キー押下時に切り替え",
            description: "指定された修飾キーを押したときにインレイヒントの表示/非表示を切り替えます。",
            aliases: None,
            field: Box::new(
                SettingField {
                    json_path: Some("languages.$(language).inlay_hints.toggle_on_modifiers_press"),
                    pick: |settings_content| {
                        language_settings_field(settings_content, |language| {
                            language.inlay_hints.as_ref()?.toggle_on_modifiers_press.as_ref()
                        })
                    },
                    write: |settings_content, value| {
                        language_settings_field_mut(settings_content, value, |language, value| {
                            language.inlay_hints.get_or_insert_default().toggle_on_modifiers_press = value;
                        })
                    },
                }
                .unimplemented(),
            ),
            metadata: None,
            files: USER | PROJECT,
        }),
    ];
    if current_language().is_none() {
        items.push(SettingsPageItem::SettingItem(SettingItem {
            title: "LSPドキュメントカラー",
            description: "エディター内でLSPのカラープレビューを描画する方法を設定します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("lsp_document_colors"),
                pick: |settings_content| settings_content.editor.lsp_document_colors.as_ref(),
                write: |settings_content, value| {
                    settings_content.editor.lsp_document_colors = value;
                },
            }),
            metadata: None,
            files: USER,
        }))
    }
    items.extend([
        SettingsPageItem::SectionHeader("タスク"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "有効",
            description: "この言語でタスクを有効にするかどうかを設定します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).tasks.enabled"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.tasks.as_ref()?.enabled.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.tasks.get_or_insert_default().enabled = value;

                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "変数",
            description: "特定の言語に対して設定する追加のタスク変数です。",
            aliases: None,
            field: Box::new(
                SettingField {
                    json_path: Some("languages.$(language).tasks.variables"),
                    pick: |settings_content| {
                        language_settings_field(settings_content, |language| {
                            language.tasks.as_ref()?.variables.as_ref()
                        })
                    },
                    write: |settings_content, value| {
                        language_settings_field_mut(settings_content, value, |language, value| {
                            language.tasks.get_or_insert_default().variables = value;

                        })
                    },
                }
                .unimplemented(),
            ),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "LSPを優先",
            description: "Gramの言語拡張機能タスクよりもLSPタスクを優先して使用します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).tasks.prefer_lsp"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.tasks.as_ref()?.prefer_lsp.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.tasks.get_or_insert_default().prefer_lsp = value;

                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SectionHeader("その他"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "単語差分を有効化",
            description: "エディターで単語単位の差分ハイライトを有効にするかどうかを設定します。有効にすると、変更された行内で変更箇所が強調表示されます。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).word_diff_enabled"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.word_diff_enabled.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.word_diff_enabled = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "デバッガー",
            description: "この言語で優先して使用するデバッガーです。",
            aliases: None,
            field: Box::new(
                SettingField {
                    json_path: Some("languages.$(language).debuggers"),
                    pick: |settings_content| {
                        language_settings_field(settings_content, |language| language.debuggers.as_ref())
                    },
                    write: |settings_content, value| {
                        language_settings_field_mut(settings_content, value, |language, value| {
                            language.debuggers = value;

                        })
                    },
                }
                .unimplemented(),
            ),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "中クリックで貼り付け",
            description: "Linuxで中クリックによる貼り付けを有効にします。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).editor.middle_click_paste"),
                pick: |settings_content| settings_content.editor.middle_click_paste.as_ref(),
                write: |settings_content, value| {settings_content.editor.middle_click_paste = value;},
            }),
            metadata: None,
            files: USER,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "改行時にコメントを継続",
            description: "前の行がコメントの場合に、新しい行もコメントで開始するかどうかを設定します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).extend_comment_on_newline"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.extend_comment_on_newline.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.extend_comment_on_newline = value;

                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "括弧の色分け",
            description: "エディター内の括弧を色分けして表示するかどうかを設定します。",
            aliases: Some(&["Rainbow Parentheses", "Rainbow Brackets"]),
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).colorize_brackets"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.colorize_brackets.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.colorize_brackets = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
    ]);

    if current_language().is_none() {
        items.extend([
            SettingsPageItem::SettingItem(SettingItem {
                title: "画像ビューアー",
                description: "画像ファイルサイズの表示単位です。",
                aliases: None,
                field: Box::new(SettingField {
                    json_path: Some("image_viewer.unit"),
                    pick: |settings_content| {
                        settings_content
                            .image_viewer
                            .as_ref()
                            .and_then(|image_viewer| image_viewer.unit.as_ref())
                    },
                    write: |settings_content, value| {
                        settings_content.image_viewer.get_or_insert_default().unit = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
            SettingsPageItem::SettingItem(SettingItem {
                title: "ドロップ領域の目標サイズ",
                description: "ドロップされたファイルを分割ペインとして開くエディター内のドロップ領域の相対サイズです。",
                aliases: None,
                field: Box::new(SettingField {
                    json_path: Some("drop_target_size"),
                    pick: |settings_content| settings_content.workspace.drop_target_size.as_ref(),
                    write: |settings_content, value| {
                        settings_content.workspace.drop_target_size = value;
                    },
                }),
                metadata: None,
                files: USER,
            }),
        ]);
    }
    items
}

/// LanguageSettings items that should be included in the "Languages & Tools" page
/// not the "Editor" page
fn non_editor_language_settings_data() -> Vec<SettingsPageItem> {
    vec![
        SettingsPageItem::SectionHeader("LSP"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "言語サーバーを有効化",
            description: "コードインテリジェンスを提供するために言語サーバーを使用するかどうかを設定します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).enable_language_server"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| language.enable_language_server.as_ref())
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.enable_language_server = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "言語サーバー",
            description: "この言語で使用する（または無効化する）言語サーバーの一覧です。",
            aliases: None,
            field: Box::new(
                SettingField {
                    json_path: Some("languages.$(language).language_servers"),
                    pick: |settings_content| {
                        language_settings_field(settings_content, |language| language.language_servers.as_ref())
                    },
                    write: |settings_content, value| {
                        language_settings_field_mut(settings_content, value, |language, value| {
                            language.language_servers = value;
                        })
                    },
                }
                .unimplemented(),
            ),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "連動編集",
            description: "言語サーバーがサポートしている場合に関連範囲の連動編集を行うかを設定します。たとえば開始タグ<html>を編集すると対応する終了タグ</html>も自動編集されます。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).linked_edits"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| language.linked_edits.as_ref())
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.linked_edits = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "定義へ移動のフォールバック",
            description: "言語サーバーからの「定義へ移動」の応答が空だった場合に追加検索を行うかどうかを設定します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("go_to_definition_fallback"),
                pick: |settings_content| settings_content.editor.go_to_definition_fallback.as_ref(),
                write: |settings_content, value| {
                    settings_content.editor.go_to_definition_fallback = value;
                },
            }),
            metadata: None,
            files: USER,
        }),
        SettingsPageItem::SectionHeader("LSP補完"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "有効",
            description: "LSPによる補完を取得するかどうかを設定します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).completions.lsp"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| language.completions.as_ref()?.lsp.as_ref())
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.completions.get_or_insert_default().lsp = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "取得タイムアウト（ミリ秒）",
            description: "LSP補完を取得する際に特定のサーバーからの応答を待機する時間です（0で無期限待機）。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).completions.lsp_fetch_timeout_ms"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.completions.as_ref()?.lsp_fetch_timeout_ms.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.completions.get_or_insert_default().lsp_fetch_timeout_ms = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "挿入モード",
            description: "LSP補完の挿入方法を制御します。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).completions.lsp_insert_mode"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.completions.as_ref()?.lsp_insert_mode.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.completions.get_or_insert_default().lsp_insert_mode = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SectionHeader("デバッガー"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "デバッガー",
            description: "この言語で優先して使用するデバッガーです。",
            aliases: None,
            field: Box::new(
                SettingField {
                    json_path: Some("languages.$(language).debuggers"),
                    pick: |settings_content| {
                        language_settings_field(settings_content, |language| language.debuggers.as_ref())
                    },
                    write: |settings_content, value| {
                        language_settings_field_mut(settings_content, value, |language, value| {
                            language.debuggers = value;
                        })
                    },
                }
                .unimplemented(),
            ),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SectionHeader("Prettier"),
        SettingsPageItem::SettingItem(SettingItem {
            title: "許可",
            description: "指定された言語でPrettierによるフォーマットを有効または無効にします。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).prettier.allowed"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| {
                        language.prettier.as_ref()?.allowed.as_ref()
                    })
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.prettier.get_or_insert_default().allowed = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "パーサー",
            description: "この言語のファイルをフォーマットする際にPrettier統合で使用を強制するパーサー名です。",
            aliases: None,
            field: Box::new(SettingField {
                json_path: Some("languages.$(language).prettier.parser"),
                pick: |settings_content| {
                    language_settings_field(settings_content, |language| language.prettier.as_ref()?.parser.as_ref())
                },
                write: |settings_content, value| {
                    language_settings_field_mut(settings_content, value, |language, value| {
                        language.prettier.get_or_insert_default().parser = value;
                    })
                },
            }),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "プラグイン",
            description: "この言語のファイルをフォーマットする際にPrettier統合で使用を強制するプラグインです。",
            aliases: None,
            field: Box::new(
                SettingField {
                    json_path: Some("languages.$(language).prettier.plugins"),
                    pick: |settings_content| {
                        language_settings_field(settings_content, |language| {
                            language.prettier.as_ref()?.plugins.as_ref()
                        })
                    },
                    write: |settings_content, value| {
                        language_settings_field_mut(settings_content, value, |language, value| {
                            language.prettier.get_or_insert_default().plugins = value;
                        })
                    },
                }
                .unimplemented(),
            ),
            metadata: None,
            files: USER | PROJECT,
        }),
        SettingsPageItem::SettingItem(SettingItem {
            title: "オプション",
            description: "package.jsonのPrettier設定セクションと同じ形式で指定するデフォルトのPrettierオプションです。",
            aliases: None,
            field: Box::new(
                SettingField {
                    json_path: Some("languages.$(language).prettier.options"),
                    pick: |settings_content| {
                        language_settings_field(settings_content, |language| {
                            language.prettier.as_ref()?.options.as_ref()
                        })
                    },
                    write: |settings_content, value| {
                        language_settings_field_mut(settings_content, value, |language, value| {
                            language.prettier.get_or_insert_default().options = value;
                        })
                    },
                }
                .unimplemented(),
            ),
            metadata: None,
            files: USER | PROJECT,
        }),
    ]
}

fn show_scrollbar_or_editor(
    settings_content: &SettingsContent,
    show: fn(&SettingsContent) -> Option<&settings::ShowScrollbar>,
) -> Option<&settings::ShowScrollbar> {
    show(settings_content).or(settings_content
        .editor
        .scrollbar
        .as_ref()
        .and_then(|scrollbar| scrollbar.show.as_ref()))
}

fn dynamic_variants<T>() -> &'static [T::Discriminant]
where
    T: strum::IntoDiscriminant,
    T::Discriminant: strum::VariantArray,
{
    <<T as strum::IntoDiscriminant>::Discriminant as strum::VariantArray>::VARIANTS
}
