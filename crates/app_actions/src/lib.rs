use gpui::{Action, actions};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// If the editor binary doesn't use anything in this crate, it will be optimized away
// and the actions won't initialize. So we just provide an empty initialization function
// to be called from main.
//
// These may provide relevant context:
// https://github.com/rust-lang/rust/issues/47384
// https://github.com/mmastrac/rust-ctor/issues/280
pub fn init() {}

/// システムの既定の Web ブラウザーで URL を開きます。
#[derive(Clone, PartialEq, Deserialize, JsonSchema, Action)]
#[action(namespace = gram)]
#[serde(deny_unknown_fields)]
pub struct OpenBrowser {
    pub url: String,
}

/// アプリケーション内で gram:// URL を開きます。
#[derive(Clone, PartialEq, Deserialize, JsonSchema, Action)]
#[action(namespace = gram)]
#[serde(deny_unknown_fields)]
pub struct OpenGramUrl {
    pub url: String,
}

/// キーバインドの追加または変更を行うためにキーマップを開きます。
#[derive(PartialEq, Clone, Default, Action, JsonSchema, Serialize, Deserialize)]
#[action(namespace = gram, no_json, no_register)]
pub struct ChangeKeybinding {
    pub action: String,
}

actions!(
    gram,
    [
        /// 設定エディターを開きます。
        #[action(deprecated_aliases = ["app_actions::OpenSettingsEditor"])]
        OpenSettings,
        /// 設定 JSON ファイルを開きます。
        #[action(deprecated_aliases = ["app_actions::OpenSettings"])]
        OpenSettingsFile,
        /// プロジェクト固有の設定を開きます。
        #[action(deprecated_aliases = ["app_actions::OpenProjectSettings"])]
        OpenProjectSettings,
        /// 既定のキーマップファイルを開きます。
        OpenDefaultKeymap,
        /// ユーザーキーマップファイルを開きます。
        #[action(deprecated_aliases = ["app_actions::OpenKeymap"])]
        OpenKeymapFile,
        /// キーマップエディターを開きます。
        #[action(deprecated_aliases = ["app_actions::OpenKeymapEditor"])]
        OpenKeymap,
        /// サーバー設定を開きます。
        OpenServerSettings,
        /// アプリケーションを終了します。
        Quit,
        /// Gram についての情報を表示します。
        About,
        /// ドキュメントを開きます。
        OpenDocs,
        /// GPL ライセンスの詳細を表示します。
        ViewLicense,
        /// オンボーディング画面を開きます。
        OpenOnboarding,
        /// タイトルバーの表示を切り替えます。
        ToggleTitleBar
    ]
);

#[derive(PartialEq, Clone, Copy, Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionCategoryFilter {
    Themes,
    IconThemes,
    Languages,
    Grammars,
    LanguageServers,
    IndexedDocsProviders,
    Snippets,
    DebugAdapters,
}

/// 拡張機能管理画面を開きます。
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = gram)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
    /// 拡張機能ページを指定したカテゴリーで絞り込みます。
    #[serde(default)]
    pub category_filter: Option<ExtensionCategoryFilter>,
    /// 指定した ID の拡張機能にフォーカスします。
    #[serde(default)]
    pub id: Option<String>,
}

/// エディターバッファーのフォントサイズを縮小します。
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = gram)]
#[serde(deny_unknown_fields)]
pub struct DecreaseBufferFontSize {
    #[serde(default)]
    pub persist: bool,
}

/// エディターバッファーのフォントサイズを拡大します。
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = gram)]
#[serde(deny_unknown_fields)]
pub struct IncreaseBufferFontSize {
    #[serde(default)]
    pub persist: bool,
}

/// 指定した設定項目の位置で設定を開きます。
#[derive(PartialEq, Clone, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = gram)]
#[serde(deny_unknown_fields)]
pub struct OpenSettingsAt {
    /// 特定の設定へのパス（例: `theme.mode`）
    pub path: String,
}

/// 指定したページでドキュメントビューアを開きます。
#[derive(PartialEq, Clone, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = gram)]
#[serde(deny_unknown_fields)]
pub struct OpenDocsAt {
    /// ドキュメントページへのパス
    pub path: String,
}

/// バッファーのフォントサイズを既定値にリセットします。
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = gram)]
#[serde(deny_unknown_fields)]
pub struct ResetBufferFontSize {
    #[serde(default)]
    pub persist: bool,
}

/// ユーザーインターフェースのフォントサイズを縮小します。
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = gram)]
#[serde(deny_unknown_fields)]
pub struct DecreaseUiFontSize {
    #[serde(default)]
    pub persist: bool,
}

/// ユーザーインターフェースのフォントサイズを拡大します。
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = gram)]
#[serde(deny_unknown_fields)]
pub struct IncreaseUiFontSize {
    #[serde(default)]
    pub persist: bool,
}

/// UI のフォントサイズを既定値にリセットします。
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = gram)]
#[serde(deny_unknown_fields)]
pub struct ResetUiFontSize {
    #[serde(default)]
    pub persist: bool,
}

/// すべての拡大率（UI およびバッファーのフォントサイズ）を既定値にリセットします。
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = gram)]
#[serde(deny_unknown_fields)]
pub struct ResetAllZoom {
    #[serde(default)]
    pub persist: bool,
}

pub mod dev {
    use gpui::actions;

    actions!(
        dev,
        [
            /// UI 要素をデバッグするための開発者インスペクターを切り替えます。
            ToggleInspector,
            /// パフォーマンスプロファイラーを開きます。
            OpenProfiler,
        ]
    );
}

pub mod workspace {
    use gpui::actions;

    actions!(
        workspace,
        [
            /// パスをコピーします。
            #[action(deprecated_aliases = ["editor::CopyPath", "outline_panel::CopyPath", "project_panel::CopyPath"])]
            CopyPath,
            /// 相対パスをコピーします。
            #[action(deprecated_aliases = ["editor::CopyRelativePath", "outline_panel::CopyRelativePath", "project_panel::CopyRelativePath"])]
            CopyRelativePath,
            /// 選択したファイルをシステムの既定アプリケーションで開きます。
            #[action(deprecated_aliases = ["project_panel::OpenWithSystem"])]
            OpenWithSystem,
        ]
    );
}

pub mod git {
    use gpui::actions;

    actions!(
        git,
        [
            /// 別の Git ブランチをチェックアウトします。
            CheckoutBranch,
            /// 別の Git ブランチに切り替えます。
            Switch,
            /// 別のリポジトリを選択します。
            SelectRepo,
            /// リモートを絞り込みます。
            FilterRemotes,
            /// Git リモートを作成します。
            CreateRemote,
            /// Git ブランチ選択画面を開きます。
            #[action(deprecated_aliases = ["branches::OpenRecent"])]
            Branch,
            /// Git スタッシュ選択画面を開きます。
            ViewStash,
            /// Git ワークツリー選択画面を開きます。
            Worktree,
            /// 現在のブランチのプルリクエストを作成します。
            CreatePullRequest,
            /// Git グラフビューを開きます。
            OpenGraph,
        ]
    );
}

pub mod toast {
    use gpui::actions;

    actions!(
        toast,
        [
            /// トースト通知に関連付けられたアクションを実行します。
            RunAction
        ]
    );
}

pub mod command_palette {
    use gpui::actions;

    actions!(
        command_palette,
        [
            /// コマンドパレットを切り替えます。
            Toggle,
        ]
    );
}

pub mod docs_search {
    use gpui::actions;

    actions!(
        docs_search,
        [
            /// ドキュメント検索ポップアップを開きます。
            Toggle,
        ]
    );
}

pub mod quick_search {
    use gpui::actions;

    actions!(
        quick_search,
        [
            /// クイック検索ポップアップを開きます。
            Toggle,
        ]
    );
}

pub mod project_panel {
    use gpui::actions;

    actions!(
        project_panel,
        [
            /// プロジェクトパネルを切り替えます。
            Toggle,
            /// プロジェクトパネルへのフォーカスを切り替えます。
            ToggleFocus
        ]
    );
}

pub mod theme_selector {
    use gpui::Action;
    use schemars::JsonSchema;
    use serde::Deserialize;

    /// テーマ選択画面を切り替えます。
    #[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
    #[action(namespace = theme_selector)]
    #[serde(deny_unknown_fields)]
    pub struct Toggle {
        /// テーマ選択画面の候補を絞り込むテーマ名のリスト。
        pub themes_filter: Option<Vec<String>>,
    }
}

pub mod icon_theme_selector {
    use gpui::Action;
    use schemars::JsonSchema;
    use serde::Deserialize;

    /// アイコンテーマ選択画面を切り替えます。
    #[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
    #[action(namespace = icon_theme_selector)]
    #[serde(deny_unknown_fields)]
    pub struct Toggle {
        /// アイコンテーマ選択画面の候補を絞り込むテーマ名のリスト。
        pub themes_filter: Option<Vec<String>>,
    }
}

pub mod settings_profile_selector {
    use gpui::Action;
    use schemars::JsonSchema;
    use serde::Deserialize;

    /// 設定プロファイル選択画面を切り替えます。
    #[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
    #[action(namespace = settings_profile_selector)]
    pub struct Toggle;
}

/// 最近開いたプロジェクト画面を開きます。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = projects)]
#[serde(deny_unknown_fields)]
pub struct OpenRecent {
    #[serde(default)]
    pub create_new_window: bool,
}

/// リモートプロジェクトを開きます。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = projects)]
#[serde(deny_unknown_fields)]
pub struct OpenRemote {
    #[serde(default)]
    pub from_existing_connection: bool,
    #[serde(default)]
    pub create_new_window: bool,
}

/// UI 上でタスクを起動する場所。
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RevealTarget {
    /// 中央ペイングループ（メインのエディター領域）。
    Center,
    /// ターミナルドック（通常のターミナル配置場所）。
    #[default]
    Dock,
}

/// 名前を指定してタスクを起動するか、タスクモーダルを開きます。
#[derive(Debug, PartialEq, Clone, Deserialize, JsonSchema, Action)]
#[action(namespace = task)]
#[serde(untagged)]
pub enum Spawn {
    /// 指定した名前でタスクを起動します。
    ByName {
        task_name: String,
        #[serde(default)]
        reveal_target: Option<RevealTarget>,
    },
    /// 指定したタグでタスクを起動します。
    ByTag {
        task_tag: String,
        #[serde(default)]
        reveal_target: Option<RevealTarget>,
    },
    /// モーダルで選択してタスクを起動します。
    ViaModal {
        /// 選択したタスクの `reveal_target` プロパティを上書きします。
        #[serde(default)]
        reveal_target: Option<RevealTarget>,
    },
}

impl Spawn {
    pub fn modal() -> Self {
        Self::ViaModal { reveal_target: None }
    }
}

/// 前回のタスクを再実行します。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = task)]
#[serde(deny_unknown_fields)]
pub struct Rerun {
    /// タスク実行前にコンテキストを再評価するかどうかを制御します。
    /// false の場合、GRAM_COLUMN や GRAM_FILE などの環境変数は前回実行時の値が維持されます。
    /// true の場合、task::Rerun 実行時のエディター状態に合わせて更新されます。
    /// 既定値: false
    #[serde(default)]
    pub reevaluate_context: bool,
    /// 再実行するタスクの `allow_concurrent_runs` プロパティを上書きします。
    /// 既定値: null
    #[serde(default)]
    pub allow_concurrent_runs: Option<bool>,
    /// 再実行するタスクの `use_new_terminal` プロパティを上書きします。
    /// 既定値: null
    #[serde(default)]
    pub use_new_terminal: Option<bool>,

    /// 指定された場合はその ID のタスクを、それ以外は前回のタスクを再実行します。
    #[serde(skip)]
    pub task_id: Option<String>,
}

pub mod outline {
    use std::sync::OnceLock;

    use gpui::{AnyView, App, Window, actions};

    actions!(
        outline,
        [
            /// アウトラインを切り替えます。
            #[action(name = "Toggle")]
            ToggleOutline
        ]
    );
    /// A pointer to outline::toggle function, exposed here to sewer the breadcrumbs <-> outline dependency.
    pub static TOGGLE_OUTLINE: OnceLock<fn(AnyView, &mut Window, &mut App)> = OnceLock::new();
}

actions!(
    debug_panel,
    [
        /// デバッグパネルを切り替えます。
        Toggle,
        /// デバッグパネルへのフォーカスを切り替えます。
        ToggleFocus
    ]
);
actions!(
    debugger,
    [
        /// ブレークポイントの有効・無効を切り替えます。
        ToggleEnableBreakpoint,
        /// ブレークポイントを削除します。
        UnsetBreakpoint,
        /// プロジェクトのデバッグタスク設定を開きます。
        OpenProjectDebugTasks,
    ]
);

pub mod vim {
    use gpui::actions;

    actions!(
        vim,
        [
            /// 既定のキーマップファイルを開きます。
            OpenDefaultKeymap
        ]
    );
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WslConnectionOptions {
    pub distro_name: String,
    pub user: Option<String>,
}

#[cfg(target_os = "windows")]
pub mod wsl_actions {
    use gpui::Action;
    use schemars::JsonSchema;
    use serde::Deserialize;

    /// WSL 内のフォルダーを開きます。
    #[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
    #[action(namespace = projects)]
    #[serde(deny_unknown_fields)]
    pub struct OpenFolderInWsl {
        #[serde(default)]
        pub create_new_window: bool,
    }

    /// WSL ディストリビューションを開きます。
    #[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
    #[action(namespace = projects)]
    #[serde(deny_unknown_fields)]
    pub struct OpenWsl {
        #[serde(default)]
        pub create_new_window: bool,
    }
}

pub mod preview {
    pub mod markdown {
        use gpui::actions;

        actions!(
            markdown,
            [
                /// 現在のファイルの Markdown プレビューを開きます。
                OpenPreview,
                /// 分割ペインで Markdown プレビューを開きます。
                OpenPreviewToTheSide,
            ]
        );
    }

    pub mod svg {
        use gpui::actions;

        actions!(
            svg,
            [
                /// 現在のファイルの SVG プレビューを開きます。
                OpenPreview,
                /// 分割ペインで SVG プレビューを開きます。
                OpenPreviewToTheSide,
            ]
        );
    }
}
