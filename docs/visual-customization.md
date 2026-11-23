# 見た目のカスタマイズ

Gram の見た目のレイアウトのさまざまな要素は、設定ウィンドウまたは `settings.json` ファイルから構成できます。

設定 UI または設定ファイルを開くには、次のコマンドを使います:

- {#action gram::OpenSettings} ({#kb gram::OpenSettings})
- {#action gram::OpenSettingsFile} ({#kb gram::OpenSettingsFile})

追加情報と見た目以外の設定については、[Gram の設定](./configuring-gram.md) を参照してください。

## テーマ

[テーマ](./themes.md)と[アイコンテーマ](./icon-themes.md)は拡張機能としてインストールできます。Zed の拡張機能のテーマとアイコンテーマのほとんどは互換性があるはずです。

コマンド: {#action gram::Extensions}。

現在のテーマはテーマセレクターでプレビューして選択できます: {#action theme_selector::Toggle}（{#kb theme_selector::Toggle}）と {#action icon_theme_selector::Toggle}。

セレクターは次の設定を変更します:

```json
{
  "theme": "One Dark",
  "icon_theme": "Gram (Default)",
}
```

ライトモード/ダークモードで別々のテーマを設定するには、次のようにします:

```json
{
  "theme": {
    "dark": "One Dark",
    "light": "One Light",
    // 使用するモード（dark、light）、または OS のライト/ダークモードに追従する "system"（既定）
    "mode": "system",
  },
  "icon_theme": {
    "dark": "Gram (Default)",
    "light": "Gram (Default)",
    // 使用するモード（dark、light）、または OS のライト/ダークモードに追従する "system"（既定）
    "mode": "system",
  },
}
```

## フォント

```json
  // UI フォント。既定のシステムフォント（macOS では SF Pro）を使うには ".SystemUIFont"、
  // 同梱の既定フォントを使うには ".GramSans" を指定します。
  "ui_font_family": ".SystemUIFont",
  "ui_font_weight": 400, // 標準 CSS 単位のフォントウェイト（100〜900）。
  "ui_font_size": 16,

  // バッファーフォント - エディターバッファーで使います
  // 同梱の既定等幅フォントを使うには ".GramMono" を指定します
  "buffer_font_family": "Berkeley Mono", // エディターバッファーのフォント名
  "buffer_font_size": 15,                 // エディターバッファーのフォントサイズ
  "buffer_font_weight": 400,              // CSS 単位のフォントウェイト [100-900]
  // 行の高さ。"comfortable"（1.618）、"standard"（1.3）、またはカスタム: `{ "custom": 2 }`
  "buffer_line_height": "comfortable",

  // ターミナルのフォント設定
  "terminal": {
    "font_family": "",
    "font_size": 15,
    // ターミナルの行の高さ: comfortable（1.618）、standard（1.3）、または `{ "custom": 2 }`
    "line_height": "standard",
  },
```

### フォントの合字

フォントの合字は、特定の隣接する文字を視覚的に組み合わせます。たとえば `=>` は `→`、`!=` は `≠` として表示されます。

これは純粋に見た目上のもので、個々の文字自体は変わりません。

この動作を無効にするには、次を使います:

```json
{
  "buffer_font_features": {
    "calt": false, // 合字を無効化
  },
}
```

### ステータスバー

```json
{
  // 行インジケーターに完全なラベルを表示するか、短いラベルを表示するか
  //   - `short`: "2 s, 15 l, 32 c"
  //   - `long`: "2 selections, 15 lines, 32 characters"
  "line_indicator_format": "long",

  // ステータスバーのアイコンは個別に非表示にできます:
  // "project_panel": {"button": false },
  // "outline_panel": {"button": false },
  // "git_panel": {"button": false },
  // "debugger": {"button": false },
  // "diagnostics": {"button": false },
  // "search": {"button": false },
}
```

### タイトルバー

```json
// タイトルバーに表示/非表示する項目を制御します
"title_bar": {
  "show": true,                   // タイトルバー自体を表示/非表示
  "show_branch_icon": false,      // ブランチ切り替え横のブランチアイコンを表示/非表示
  "show_branch_name": true,       // ブランチ名を表示/非表示
  "show_project_items": true,     // プロジェクトのホストと名前を表示/非表示
  "show_onboarding_banner": true, // オンボーディングバナーを表示/非表示
  "show_user_picture": true,      // ユーザーのアバターを表示/非表示
  "show_menus": false             // メニューを表示/非表示
},
```

## ワークスペース

```json
{
  // OS 標準のピッカー（false）の代わりに、Gram 内蔵のパスプロンプト
  // （ファイル・ディレクトリのピッカー）を強制的に使います。
  "use_system_path_prompts": true,
  // OS 標準のプロンプト（false）の代わりに、Gram 内蔵の確認プロンプト
  //（「保存しますか？」）を強制的に使います。Linux では無視されます（常に false）。
  "use_system_prompts": true,

  // アクティブなペインのスタイル設定。
  "active_pane_modifiers": {
    // アクティブなペインの内側の境界線のサイズ（ピクセル）。
    "border_size": 0.0,
    // 非アクティブなペインの不透明度。0 は透明、1 は不透明。
    "inactive_opacity": 1.0,
  },

  // 下部ドックのレイアウトモード: contained、full、left_aligned、right_aligned
  "bottom_dock_layout": "contained",

  // ドックのサイズを変更したときに、ドック内のすべてのパネルのサイズも変更するか。
  // "left"、"right"、"bottom" の組み合わせを指定できます。
  "resize_all_panels_in_dock": ["left"],
}
```

<!--
TBD: 中央揃えレイアウトに関連する設定
```json
    "centered_layout": {
    // 中央揃えレイアウトを使うときの、ワークスペースからの
    // 中央ペイン左側パディングの相対幅。
    "left_padding": 0.2,
    // 中央揃えレイアウトを使うときの、ワークスペースからの
    // 中央ペイン右側パディングの相対幅。
    "right_padding": 0.2
    },
```
-->

## エディター

```json
  // エディターでカーソルを点滅させるか。
  "cursor_blink": true,

  // 既定のエディターのカーソル形状: bar、block、underline、hollow
  "cursor_shape": null,

  // エディターで現在の行をハイライトする範囲: none、gutter、line、all
  "current_line_highlight": "all",

  // マウスカーソルをいつ隠すか: never、on_typing、on_typing_and_movement
  "hide_mouse": "on_typing_and_movement",

  // エディター内で選択中テキストのすべての出現箇所をハイライトするか。
  "selection_highlight": true,

  // タブとスペースを視覚的に表示する（none、all、selection、boundary、trailing）
  "show_whitespaces": "selection",
  "whitespace_map": { // `show_whitespaces` 有効時に表示する文字
    "space": "•",
    "tab": "⟶"       // より短い矢印にするには "→" を使います
  },

  "unnecessary_code_fade": 0.3, // 未使用のコードをどれだけフェードさせるか。

  // クライアントサイドデコレーションを使うプラットフォームでのウィンドウの角丸を定義します。
  "client_side_decoration_rounding": 10.0,
  // クライアントサイドデコレーションを使うプラットフォームでのウィンドウの影のサイズを定義します。
  "client_side_decoration_shadow": 10.0,

  // プライベートファイルで変数の値を視覚表示から隠す
  "redact_private_values": false,

  // ソフト折り返しとルーラー
  "soft_wrap": "none",          // none、editor_width、preferred_line_length、bounded
  "preferred_line_length": 80,  // ソフト折り返しする列
  "show_wrap_guides": true,     // 折り返しガイド（縦ルーラー）を表示/非表示
  "wrap_guides": [],            // 折り返しガイドを配置する位置（文字数）

  // ガターの設定
  "gutter": {
    "line_numbers": true,         // ガターの行番号を表示/非表示。
    "runnables": true,            // ガターの実行可能ボタンを表示/非表示。
    "breakpoints": true,          // ガターのブレークポイントを表示/非表示。
    "folds": true,                // ガターの折りたたみボタンを表示/非表示。
    "min_line_number_digits": 4   // N 桁の行番号用の領域を確保
  },
  "relative_line_numbers": "enabled", // ガターに相対行番号を表示

  // インデントガイド
  "indent_guides": {
    "enabled": true,
    "line_width": 1,                  // ガイドの幅（ピクセル）[1-10]
    "active_line_width": 1,           // アクティブなガイドの幅（ピクセル）[1-10]
    "coloring": "fixed",              // disabled、fixed、indent_aware
    "background_coloring": "disabled" // disabled、indent_aware
  },

  // スムーズスクロール
  "smooth_scroll": {
    "enabled": false // スムーズなイージング効果でスクロールをアニメーションするか。既定では無効。
    "duration": 160 // スクロールアニメーションの長さ（ミリ秒）
  },

  "sticky_scroll": {
    "enabled": false // スコープをエディターの先頭に固定するか。既定では無効。
  }
```

### Git Blame {#editor-blame}

```json
"git": {
  "split_diff": false,           // Diff を左右に並べて開く
  "inline_blame": {
    "enabled": true,             // インライン blame を表示/非表示
    "delay_ms": 0,                  // 遅延後に表示（ミリ秒）
    "min_column": 0,             // インラインで blame を表示する最小カラム
    "padding": 7,                // コードとインライン blame の間のパディング（em）
    "show_commit_summary": false // コミットの概要を表示/非表示
  },
  "hunk_style": "staged_hollow"  // staged_hollow、unstaged_hollow
}
```

### エディターツールバー

```json
// エディターツールバーに関連する設定
"toolbar": {
  "breadcrumbs": true, // ブレッドクラムを表示するか。
  "quick_actions": true, // クイックアクションボタンを表示するか。
  "selections_menu": true, // 選択範囲メニューを表示するか
  "code_actions": false // コードアクションボタンを表示するか
}
```

### エディターのスクロールバーとミニマップ {#editor-scrollbar}

```json
  // スクロールバーに関連する設定
  "scrollbar": {
    // エディターでスクロールバーをいつ表示するか（auto、system、always、never）
    "show": "auto",
    "cursors": true,          // スクロールバーにカーソル位置を表示。
    "git_diff": true,         // スクロールバーに git diff インジケーターを表示。
    "search_results": true,   // スクロールバーにバッファー検索の結果を表示。
    "selected_text": true,    // スクロールバーに選択中テキストの出現箇所を表示。
    "selected_symbol": true,  // スクロールバーに選択中シンボルの出現箇所を表示。
    "diagnostics": "all",     // 診断を表示（none、error、warning、information、all）
    "axes": {
      "horizontal": true,     // 水平スクロールバーを表示/非表示
      "vertical": true        // 垂直スクロールバーを表示/非表示
    }
  },

  // ミニマップに関連する設定
  "minimap": {
    "show": "never",                // いつ表示するか（auto、always、never）
    "display_in": "active_editor",  // どこに表示するか（active_editor、all_editor）
    "thumb": "always",              // サムネイルをいつ表示するか（always、hover）
    "thumb_border": "left_open",    // サムネイルの境界線（left_open、right_open、full、none）
    "max_width_columns": 80,        // ミニマップの最大幅
    "current_line_highlight": null  // 現在の行をハイライト（null、line、gutter）
  },

  // 最終行を超えるエディターのスクロールを制御: off、one_page、vertical_scroll_margin
  "scroll_beyond_last_line": "one_page",
  // キーボードでスクロールするときにカーソルの上下に確保する行数
  "vertical_scroll_margin": 3,
  // マウスでスクロールするときに左右に確保する文字数
  "horizontal_scroll_margin": 5,
  // スクロール感度の倍率
  "scroll_sensitivity": 1.0,
  // 高速スクロール（スクロール中に alt を押す）のスクロール感度の倍率
  "fast_scroll_sensitivity": 4.0,
```

### エディターのタブ

```json
  // ペインごとのタブの最大数。未設定なら無制限。
  "max_tabs": null,

  // タブバーの外観をカスタマイズ
  "tab_bar": {
    "show": true,                           // タブバーを表示/非表示
    "show_nav_history_buttons": true,       // タブバーの履歴ボタンを表示/非表示
    "show_tab_bar_buttons": "on_pane_focus" // タブバーのボタンを表示するタイミング（new、split、zoom）
  },
  "tabs": {
    "git_status": false,              // git ステータスを示す色
    "close_position": "right",        // 閉じるボタンの位置（left、right、hidden）
    "show_close_button": "hover",     // 閉じるボタンの表示（hover、always、hidden）
    "file_icons": false,              // ファイル種別を示すアイコン
    // ファイルアイコンに診断を表示（off、errors、all）。file_icons=true が必要
    "show_diagnostics": "off"
  }
```

### ステータスバー

```json
"status_bar": {
  // アクティブなバッファーの言語を表示するボタンを表示/非表示。
  // ボタンをクリックすると言語セレクターが開きます。
  // 既定値は true。
  "active_language_button": true,
  // カーソル位置を表示するボタンを表示/非表示。
  // ボタンをクリックすると、行とカラムへジャンプする入力欄が開きます。
  // 既定値は true。
  "cursor_position_button": true,
  // バッファーの改行モードを表示するボタンを表示/非表示。
  // ボタンをクリックすると改行コードセレクターが開きます。
  // 既定値は false。
  "line_endings_button": false,
  // バッファーの文字エンコーディングを表示するボタンを表示/非表示。
  // "non_utf8" に設定すると、BOM なしの UTF-8 の場合のみボタンが非表示になります。
  // 既定値は "non_utf8"。
  "active_encoding_button": "non_utf8"
},
"global_lsp_settings": {
  // ステータスバーの LSP ボタンを表示/非表示。
  // LSP のアクティビティは引き続き表示されます。
  // "enable_language_server" が false の場合はボタンが表示されません。
  "button": true
},
```

### マルチバッファー

```json
{
  // マルチバッファーで抜粋を展開する既定の行数。
  "expand_excerpt_lines": 5,
  // マルチバッファーの抜粋に提供する既定のコンテキスト行数。
  "excerpt_context_lines": 2,
}
```

### エディターの補完、スニペット、アクション、診断 {#editor-lsp}

```json
  "snippet_sort_order": "inline",        // スニペット補完: top、inline、bottom、none
  "show_completions_on_input": true,     // 入力中に補完を表示
  "show_completion_documentation": true, // 補完にドキュメントを表示
  "auto_signature_help": false,          // 括弧内にメソッドのシグネチャを表示

  // 補完後、または括弧の組の挿入後にシグネチャヘルプを表示するか。
  // `auto_signature_help` が有効な場合、この設定も有効として扱われます。
  "show_signature_help_after_edits": false,

  // バッファー行の先頭にコードアクションボタンを表示するか。
  "inline_code_actions": true,

  // エディターに表示する診断をフィルタリングするレベル:
  "diagnostics_max_severity": null,      // off、error、warning、info、hint、null（すべて）

  // エディターで LSP の `textDocument/documentColor` の色をレンダリングする方法。
  "lsp_document_colors": "inlay",        // none、inlay、border、background
  // 補完メニューでスクロールバーをいつ表示するか。
  "completion_menu_scrollbar": "never", // auto、system、always、never
  // 補完メニューの各項目で LSP の項目種別（関数、メソッド、変数など）を
  // どのように表示するか。"symbol" は構文に応じて色付けされた
  // 1 文字のバッジを表示します。
  "completion_menu_item_kind": "off", // off、symbol
  // 補完中でないとき、Supertab アクションは何をすべきか？
  "supertab_fallback": "tab", // tab、indent
  // kill ring を同期（emacs モード）
  "sync_kill_ring": "clipboard", // clipboard、primary、または none
  // 行番号のスケール
  "line_number_scale": "default", // default、small、xsmall
  // エディターで括弧の色付けを有効にする（言語ごとに構成可能）
  "colorize_brackets": true,
```

### エディターのインレイヒント

```json
{
  "inlay_hints": {
    "enabled": false,
    // 特定の種類のヒントを個別にオン/オフする。既定ではすべてオン。
    "show_type_hints": true,
    "show_parameter_hints": true,
    "show_other_hints": true,

    // インレイヒントに背景を表示するか（テーマの `hint.background`）
    "show_background": false, //

    // ヒントを要求する前に、編集後どれだけ待つか（0 でデバウンス無効）
    "edit_debounce_ms": 700,
    // ヒントを要求する前に、スクロール後どれだけ待つか（0 でデバウンス無効）
    "scroll_debounce_ms": 50,

    // 押すとインレイヒントの表示を切り替える修飾キーの組。
    "toggle_on_modifiers_press": {
      "control": false,
      "shift": false,
      "alt": false,
      "platform": false,
      "function": false,
    },
  },
}
```

## ファイルファインダー

```json
// ファイルファインダーの設定
"file_finder": {
  "file_icons": true,         // ファイルアイコンを表示/非表示
  "modal_max_width": "small", // 水平方向のサイズ: small、medium、large、xlarge、full
  "git_status": true,         // 各項目の git ステータスを表示
  "include_ignored": null     // 結果に gitignore 対象を含める: true、false、null
},
```

## プロジェクトパネル

プロジェクトパネルは {#action project_panel::ToggleFocus}（{#kb project_panel::ToggleFocus}）または {#action pane::RevealInProjectPanel}（{#kb pane::RevealInProjectPanel}）で表示/非表示を切り替えられます。

```json
  // プロジェクトパネルの設定
  "project_panel": {
    "button": true,                 // ステータスバーのボタンを表示/非表示
    "default_width": 240,           // 既定のパネル幅
    "dock": "left",                 // ドックの位置（left、right）
    "entry_spacing": "comfortable", // 縦方向の間隔（comfortable、standard）
    "file_icons": true,             // ファイルアイコンを表示/非表示
    "folder_icons": true,           // フォルダーアイコンを表示/非表示
    "git_status": true,             // 新規/更新されたファイルを示す
    "indent_size": 20,              // インデント 1 段階ごとのピクセル数
    "auto_reveal_entries": true,    // バッファーをアクティブにしたときパネルにファイルを表示
    "auto_fold_dirs": true,         // サブディレクトリが 1 つだけのディレクトリを折りたたむ
    "sticky_scroll": true,          // 親ディレクトリをプロジェクトパネルの先頭に固定。
    "drag_and_drop": true,          // ドラッグ&ドロップを有効にするか
    "scrollbar": {                  // プロジェクトパネルのスクロールバー設定
      "show": null                  // 表示/非表示: (auto、system、always、never)
    },
    "show_diagnostics": "all",      //
    // プロジェクトパネルのインデントガイドに関連する設定。
    "indent_guides": {
      // プロジェクトパネルでインデントガイドをいつ表示するか。(always、never)
      "show": "always"
    },
    // 項目の並び順（directories_first、smart_sort、mixed、files_first）
    "sort_mode": "directories_first",
    // ウィンドウで 1 つのフォルダーしか開いていないとき、ルート項目を隠すか。
    "hide_root": false,
    // プロジェクトパネルで隠しファイルを隠すか。
    "hide_hidden": false
  }
```

## ターミナルパネル

```json
  // ターミナルパネルの設定
  "terminal": {
    "dock": "bottom",                   // ドックの位置: left、right、bottom
    "button": true,                     // ステータスバーのアイコンを表示/非表示
    "default_width": 640,               // 既定の幅（左右ドック時）
    "default_height": 320,              // 既定の高さ（下部ドック時）

    // ターミナルのカーソル点滅動作を設定（on、off、terminal_controlled）
    "blinking": "terminal_controlled",
    // ターミナルカーソルの既定の形状（block、bar、underline、hollow）
    "cursor_shape": "block",

    // ターミナルのプロセス環境に追加する環境変数
    "env": {
      // "KEY": "value"
    },

    // ターミナルのスクロールバー
    "scrollbar": {
      "show": null                       // 表示/非表示: (auto、system、always、never)
    },
    // ターミナルのフォント設定
    "font_family": "Fira Code",
    "font_size": 15,
    "font_weight": 400,
    // ターミナルの行の高さ: comfortable（1.618）、standard（1.3）、または `{ "custom": 2 }`
    "line_height": "comfortable",

    "max_scroll_history_lines": 10000,   // スクロールバックの履歴（0=無効、最大=100000）
  }
```

見た目以外のカスタマイズオプションについては、[ターミナルの設定](./configuring-gram.md#terminal) を参照してください。

### その他のパネル

```json
  // Git パネル
  "git_panel": {
    "button": true,               // ステータスバーのアイコンを表示/非表示
    "dock": "left",               // ドックの位置: left、right
    "default_width": 360,         // git パネルの既定の幅。
    "status_style": "icon",       // label_color、icon
    "sort_by_path": false,        // パス順（false）かステータス順（true）で並べ替え
    "tree_view": false,           // ツリー表示（true）かフラットな一覧（false）
    "diff_stats": true,           // 追加/削除された行数を表示
    "scrollbar": {
      "show": null                // 表示/非表示: (auto、system、always、never)
    }
  },

  // デバッガーパネル
  "debugger": {
    "dock": "bottom",             // ドックの位置: left、right、bottom
    "button": true                // ステータスバーのアイコンを表示/非表示
  },

  // アウトラインパネル
  "outline_panel": {
    "button": true,               // ステータスバーのアイコンを表示/非表示
    "default_width": 300,         // git パネルの既定の幅
    "dock": "left",               // ドックの位置: left、right
    "file_icons": true,           // file_icons を表示/非表示
    "folder_icons": true,         // ディレクトリにファイルアイコン（true）かシェブロン（false）を表示
    "git_status": true,           // git ステータスを表示
    "indent_size": 20,            // ネストした項目のインデント（ピクセル）
    "indent_guides": {
      "show": "always"            // インデントガイドを表示（always、never）
    },
    "auto_reveal_entries": true,  // バッファーをアクティブにしたときパネルにファイルを表示
    "auto_fold_dirs": true,       // サブディレクトリが 1 つだけのディレクトリを折りたたむ
    "scrollbar": {                // プロジェクトパネルのスクロールバー設定
      "show": null                // 表示/非表示: (auto、system、always、never)
    }
  }
```
