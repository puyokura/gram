# VS Code から Gram への移行ガイド

このガイドは、VS Code を長く使っていて、ゼロからやり直すことなく Gram を試してみたい開発者向けです。

Electron ベースの重い実装や、VS Code や Zed のようなエディターに入り込みつつある不安定な AI ツール・サブスクリプション前提のモデルに不満を感じている方にとって、Gram は有力な選択肢になります。

Gram は AI・テレメトリ・サブスクリプションを一切含まない完全なオープンソースエディターであり、VS Code がサポートする多くの言語に対応しています。

## Gram のインストール

Gram は macOS、Windows、Linux で利用できます。

ソースからビルドしてインストールする場合でも、Rust プロジェクトのビルドは簡単です。https://rustup.rs を導入すれば、Gram のコンパイルに必要なコンパイラとツール一式を用意できます。

インストール後は、お使いのプラットフォームの通常のアプリケーションと同じ方法で Gram を起動できます。

## VS Code から設定をインポートする

初期セットアップ時に、VS Code から主要な設定をインポートできます。Gram がインポートする設定は次のとおりです。

### VS Code からインポートされる設定一覧

**VS Code から設定をインポート** を実行すると、以下の VS Code 設定が自動的にインポートされます。

**エディター**

| VS Code 設定                                | Gram 設定                                      |
| ------------------------------------------- | ---------------------------------------------- |
| `editor.fontFamily`                         | `buffer_font_family`                           |
| `editor.fontSize`                           | `buffer_font_size`                             |
| `editor.fontWeight`                         | `buffer_font_weight`                           |
| `editor.tabSize`                            | `tab_size`                                     |
| `editor.insertSpaces`                       | `hard_tabs`（反転）                            |
| `editor.wordWrap`                           | `soft_wrap`                                    |
| `editor.wordWrapColumn`                     | `preferred_line_length`                        |
| `editor.cursorStyle`                        | `cursor_shape`                                 |
| `editor.cursorBlinking`                     | `cursor_blink`                                 |
| `editor.renderLineHighlight`                | `current_line_highlight`                       |
| `editor.lineNumbers`                        | `gutter.line_numbers`, `relative_line_numbers` |
| `editor.showFoldingControls`                | `gutter.folds`                                 |
| `editor.minimap.enabled`                    | `minimap.show`                                 |
| `editor.minimap.autohide`                   | `minimap.show`                                 |
| `editor.minimap.showSlider`                 | `minimap.thumb`                                |
| `editor.minimap.maxColumn`                  | `minimap.max_width_columns`                    |
| `editor.stickyScroll.enabled`               | `sticky_scroll.enabled`                        |
| `editor.scrollbar.horizontal`               | `scrollbar.axes.horizontal`                    |
| `editor.scrollbar.vertical`                 | `scrollbar.axes.vertical`                      |
| `editor.mouseWheelScrollSensitivity`        | `scroll_sensitivity`                           |
| `editor.fastScrollSensitivity`              | `fast_scroll_sensitivity`                      |
| `editor.cursorSurroundingLines`             | `vertical_scroll_margin`                       |
| `editor.hover.enabled`                      | `hover_popover_enabled`                        |
| `editor.hover.delay`                        | `hover_popover_delay`                          |
| `editor.parameterHints.enabled`             | `auto_signature_help`                          |
| `editor.multiCursorModifier`                | `multi_cursor_modifier`                        |
| `editor.selectionHighlight`                 | `selection_highlight`                          |
| `editor.roundedSelection`                   | `rounded_selection`                            |
| `editor.find.seedSearchStringFromSelection` | `seed_search_query_from_cursor`                |
| `editor.rulers`                             | `wrap_guides`                                  |
| `editor.renderWhitespace`                   | `show_whitespaces`                             |
| `editor.guides.indentation`                 | `indent_guides.enabled`                        |
| `editor.linkedEditing`                      | `linked_edits`                                 |
| `editor.autoSurround`                       | `use_auto_surround`                            |
| `editor.formatOnSave`                       | `format_on_save`                               |
| `editor.formatOnPaste`                      | `auto_indent_on_paste`                         |
| `editor.formatOnType`                       | `use_on_type_format`                           |
| `editor.trimAutoWhitespace`                 | `remove_trailing_whitespace_on_save`           |
| `editor.suggestOnTriggerCharacters`         | `show_completions_on_input`                    |
| `editor.suggest.showWords`                  | `completions.words`                            |
| `editor.inlineSuggest.enabled`              | `show_edit_predictions`                        |

**ファイルとワークスペース**

| VS Code 設定                | Gram 設定                      |
| --------------------------- | ------------------------------ |
| `files.autoSave`            | `autosave`                     |
| `files.autoSaveDelay`       | `autosave.milliseconds`        |
| `files.insertFinalNewline`  | `ensure_final_newline_on_save` |
| `files.associations`        | `file_types`                   |
| `files.watcherExclude`      | `file_scan_exclusions`         |
| `files.watcherInclude`      | `file_scan_inclusions`         |
| `files.simpleDialog.enable` | `use_system_path_prompts`      |
| `search.smartCase`          | `use_smartcase_search`         |
| `search.useIgnoreFiles`     | `search.include_ignored`       |

**ターミナル**

| VS Code 設定                          | Gram 設定                           |
| ------------------------------------- | ----------------------------------- |
| `terminal.integrated.fontFamily`      | `terminal.font_family`              |
| `terminal.integrated.fontSize`        | `terminal.font_size`                |
| `terminal.integrated.lineHeight`      | `terminal.line_height`              |
| `terminal.integrated.cursorStyle`     | `terminal.cursor_shape`             |
| `terminal.integrated.cursorBlinking`  | `terminal.blinking`                 |
| `terminal.integrated.copyOnSelection` | `terminal.copy_on_select`           |
| `terminal.integrated.scrollback`      | `terminal.max_scroll_history_lines` |
| `terminal.integrated.macOptionIsMeta` | `terminal.option_as_meta`           |
| `terminal.integrated.{platform}Exec`  | `terminal.shell`                    |
| `terminal.integrated.env.{platform}`  | `terminal.env`                      |

**タブとパネル**

| VS Code 設定                                       | Gram 設定                                          |
| -------------------------------------------------- | -------------------------------------------------- |
| `workbench.editor.showTabs`                        | `tab_bar.show`                                     |
| `workbench.editor.showIcons`                       | `tabs.file_icons`                                  |
| `workbench.editor.tabActionLocation`               | `tabs.close_position`                              |
| `workbench.editor.tabActionCloseVisibility`        | `tabs.show_close_button`                           |
| `workbench.editor.focusRecentEditorAfterClose`     | `tabs.activate_on_close`                           |
| `workbench.editor.enablePreview`                   | `preview_tabs.enabled`                             |
| `workbench.editor.enablePreviewFromQuickOpen`      | `preview_tabs.enable_preview_from_file_finder`     |
| `workbench.editor.enablePreviewFromCodeNavigation` | `preview_tabs.enable_preview_from_code_navigation` |
| `workbench.editor.editorActionsLocation`           | `tab_bar.show_tab_bar_buttons`                     |
| `workbench.editor.limit.enabled` / `value`         | `max_tabs`                                         |
| `workbench.editor.restoreViewState`                | `restore_on_file_reopen`                           |
| `workbench.statusBar.visible`                      | `status_bar.show`                                  |

**プロジェクトパネル（ファイルエクスプローラー）**

| VS Code 設定                   | Gram 設定                           |
| ------------------------------ | ----------------------------------- |
| `explorer.compactFolders`      | `project_panel.auto_fold_dirs`      |
| `explorer.autoReveal`          | `project_panel.auto_reveal_entries` |
| `explorer.excludeGitIgnore`    | `project_panel.hide_gitignore`      |
| `problems.decorations.enabled` | `project_panel.show_diagnostics`    |
| `explorer.decorations.badges`  | `project_panel.git_status`          |

**Git**

| VS Code 設定                         | Gram 設定                                      |
| ------------------------------------ | ---------------------------------------------- |
| `git.enabled`                        | `git_panel.button`                             |
| `git.defaultBranchName`              | `git_panel.fallback_branch_name`               |
| `git.decorations.enabled`            | `git.inline_blame`, `project_panel.git_status` |
| `git.blame.editorDecoration.enabled` | `git.inline_blame.enabled`                     |

**ウィンドウと動作**

| VS Code 設定                                     | Gram 設定                                |
| ------------------------------------------------ | ---------------------------------------- |
| `window.confirmBeforeClose`                      | `confirm_quit`                           |
| `window.nativeTabs`                              | `use_system_window_tabs`                 |
| `window.closeWhenEmpty`                          | `when_closing_with_no_tabs`              |
| `accessibility.dimUnfocused.enabled` / `opacity` | `active_pane_modifiers.inactive_opacity` |

**その他**

| VS Code 設定         | Gram 設定                                                |
| -------------------- | -------------------------------------------------------- |
| `http.proxy`         | `proxy`                                                  |
| `npm.packageManager` | `node.npm_path`                                          |
| `outline.icons`      | `outline_panel.file_icons`, `outline_panel.folder_icons` |

Gram は拡張機能やキーバインドまではインポートしませんが、慣れた操作感で新しい環境を試す最短の方法です。初期セットアップ時にスキップした場合でも、後からコマンドパレットで手動インポートできます。

`Cmd+Shift+P → Gram: Import VS Code Settings`

## エディター設定を構成する

設定エディターから手動で設定を変更することもできます。

設定を編集するには、次のいずれかを実行します。

1. `Cmd+,` で設定エディターを開きます。
2. コマンドパレットで `gram: open settings` を実行します。

よく使われる VS Code 設定との対応は次のとおりです。

| VS Code             | Gram               | 備考                                   |
| ------------------- | ------------------ | -------------------------------------- |
| editor.fontFamily   | buffer_font_family | Gram は既定で Fira Code を使用         |
| editor.fontSize     | buffer_font_size   | ピクセル単位で指定                     |
| editor.tabSize      | tab_size           | 言語ごとに上書き可能                   |
| editor.insertSpaces | insert_spaces      | 真偽値                                 |
| editor.formatOnSave | format_on_save     | フォーマッター有効時に動作             |
| editor.wordWrap     | soft_wrap          | 任意の折り返し桁数を指定可能           |

Gram はプロジェクトごとの設定もサポートしており、設定エディターから編集できます。

## プロジェクトを開く・作成する

セットアップ後、`Cmd+O`（Linux では `Ctrl+O`）を押してフォルダーを開くと、そのフォルダーが Gram のワークスペースになります。VS Code のようなマルチルートワークスペースや `.code-workspace` ファイルには対応していません。Gram はシンプルに「1 フォルダー＝1 ワークスペース」を採用しています。

新しいプロジェクトを始めるには、ターミナルやファイルマネージャーでディレクトリを作成してから Gram で開きます。エディターはそのフォルダーをプロジェクトルートとして扱います。

任意のフォルダー内でターミナルから `gram .` を実行して Gram を起動することもできます。

プロジェクトを開いたら、`Cmd+P` でファイル間を素早く移動できます。`Cmd+Shift+P`（Linux では `Ctrl+Shift+P`）でコマンドパレットを開き、アクションやタスクの実行、設定の切り替えなどを行えます。

開いているバッファーは上部にタブとして表示されます。サイドバーにはファイルツリーと Git ステータスが表示され、`Cmd+B` で折りたたんでコードに集中できます。

## キーバインドの違い

オンボーディングで VS Code キーマップを選択していれば、ほとんどのショートカットはそのまま慣れた操作で使えます。VS Code と Gram のキーバインドの比較は以下のとおりです。

### 共通の主要キーバインド（Gram <> VS Code）

| 操作                             | ショートカット         |
| -------------------------------- | ---------------------- |
| ファイルを検索                   | `Cmd + P`              |
| コマンドを実行                   | `Cmd + Shift + P`      |
| プロジェクト全体をテキスト検索   | `Cmd + Shift + F`      |
| シンボルを検索（プロジェクト全体） | `Cmd + T`            |
| シンボルを検索（ファイル内）     | `Cmd + Shift + O`      |
| 左ドックを切り替え               | `Cmd + B`              |
| 下ドックを切り替え               | `Cmd + J`              |
| ターミナルを開く                 | `Ctrl + ~`             |
| ファイルツリーを開く             | `Cmd + Shift + E`      |
| 現在のバッファーを閉じる         | `Cmd + W`              |
| プロジェクト全体を閉じる         | `Cmd + Shift + W`      |
| リファクタリング: シンボル名変更 | `F2`                   |
| テーマを変更                     | `Cmd + K, Cmd + T`     |
| テキストの折り返し               | `Opt + Z`              |
| 開いているタブを移動             | `Cmd + Opt + Arrow`    |
| 構文ベースの折りたたみ / 展開    | `Cmd + Opt + {` or `}` |

### 異なるキーバインド（Gram <> VS Code）

| 操作                       | VS Code               | Gram                   |
| -------------------------- | --------------------- | ---------------------- |
| 最近使ったプロジェクトを開く | `Ctrl + R`          | `Cmd + Opt + O`        |
| 行を上下に移動             | `Opt + Up/Down`       | `Cmd + Ctrl + Up/Down` |
| ペインを分割               | `Cmd + \`             | `Cmd + K, Arrow Keys`  |
| 選択範囲を拡張             | `Shift + Alt + Right` | `Opt + Up`             |

### Gram 固有のキーバインド

| 操作             | ショートカット               | 備考                                         |
| ---------------- | ---------------------------- | -------------------------------------------- |
| 右ドックを切り替え | `Cmd + R` or `Cmd + Alt + B` |                                              |
| 構文選択         | `Opt + Up/Down`              | 構造単位（波括弧の内側など）でコードを選択します。 |

### キーバインドをカスタマイズする方法

キーバインドを編集するには次の手順を実行します。

- コマンドパレット（`Cmd+Shift+P`）を開く
- `Gram: Open Keymap Editor` を実行する

利用可能なキーバインドの一覧が開き、個別のショートカットの上書きや競合の解消、自分の環境に合わせた構成を行えます。

Gram も VS Code と同様に `Cmd+K Cmd+C` のようなコード（複数キーの連続入力）をサポートしています。

## ユーザーインターフェースの違い

### ワークスペースファイルなし

VS Code にはマルチルートフォルダーや `.code-workspace` ファイル、「ウィンドウ」と「ワークスペース」の明確な区別といった専用のワークスペース概念があります。Gram はこのモデルを簡素化しています。

Gram では次のようになります。

- ワークスペース専用のファイル形式はありません。フォルダーを開くこと自体がプロジェクトコンテキストになります。

- Gram はマルチルートワークスペースをサポートしていません。1 つのウィンドウで開けるフォルダーは 1 つです。

- プロジェクト単位の動作の大半は開いたフォルダーに紐づきます。検索、Git 統合、タスク、環境検出はいずれも開いたディレクトリをプロジェクトルートとして扱います。

- プロジェクトごとの設定は任意です。プロジェクト内に `.gram/settings.json` を置けばグローバル設定を上書きできますが、`.code-workspace` ファイルは使用せずインポートもしません。

- 単一ファイルや空のウィンドウから始めることもできます。編集を始めるためにフォルダーを開く必要はありません。

結果として、「フォルダーを開く → その中で作業する」という余計な層のないシンプルな構成になっています。

### プロジェクト内のナビゲーション

VS Code ではフォルダーを開いて左側のサイドバーを中心に操作するのが一般的ですが、Gram は異なるアプローチを取ります。

- フォルダーを開くこともできますが、必須ではありません。単一ファイルや空のワークスペースからでも始められます。
- コマンドパレット（`Cmd+Shift+P`）とファイル検索（`Cmd+P`）が中心的なナビゲーションツールです。ファイル検索はワークスペース全体からファイル・シンボル・コマンドを即座に検索します。
- サイドバーを常時開いておく代わりに、Gram では次のような操作を推奨しています。
  - ファイル名でのファジー検索（`Cmd+P`）
  - シンボルへの直接ジャンプ（`Cmd+Shift+O`）
  - 大きなファイルツリーを常時開く代わりに、分割ペインとタブでコンテキストを管理する（好みに応じてプロジェクトパネルを使うこともできます）

UI は意図的に最小限に保たれています。パネルは必要なときだけ表示され、邪魔をしません。ペインの管理ではなくコード間の移動に集中できる設計です。

### 拡張機能とマーケットプレイス

Gram は VS Code ほど多くの拡張機能を提供していません。提供される拡張機能は、言語サポート、テーマ、シンタックスハイライト、その他の基本的な編集機能の強化に重点を置いています。

一方で、VS Code では通常拡張機能が必要になる多くの機能が Gram に標準で組み込まれています。

- Vim モーダル編集モード（および Helix モード）
- 組み込みターミナルパネル
- プロジェクト全体のファジー検索
- JSON 設定によるタスクランナー
- LSP 経由のインライン診断とコードアクション

特に DevOps、コンテナ、テストランナー系のツールに依存している場合、すべての VS Code 拡張機能に 1 対 1 の代替が見つかるわけではありません。Gram の拡張機能エコシステムはまだ成長途中であり、カタログも意図的にコンパクトに保たれています。

### 高度な設定と生産性向上のヒント

Gram には環境を細かく調整したいパワーユーザー向けの高度な設定が用意されています。

便利な設定例をいくつか紹介します。

**保存時の自動フォーマット:**

```json
"format_on_save": "on"
```

**direnv サポートを有効化:**

```json
"load_direnv": "shell_hook"
```

**カスタムタスク**: `tasks.jsonc`（コマンドパレットの `gram: open tasks` から開けます）でビルドや実行コマンドを定義できます。

```json
[
  {
    "label": "build",
    "command": "cargo build"
  }
]
```

**カスタムスニペットの移行**: VS Code のスニペット JSON をそのまま Gram のスニペットフォルダー（`gram: configure snippets`）にコピーできます。
