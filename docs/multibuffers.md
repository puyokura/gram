# マルチバッファー

マルチバッファー（Multibuffers）を使用すると、単一のビューから複数のファイルを同時に編集できます。

マルチバッファーは、マルチカーソル機能と組み合わせて使用することで真価を発揮します。

## マルチバッファーでの編集

マルチバッファーでの編集は、通常のファイルの編集とまったく同じです。加えた変更は、エディター内の他の場所で開かれている該当ファイルにも即座に反映されます。

すべてのファイルを保存するには {#action editor::Save} を実行します（macOS では `cmd-s`、Windows/Linux では `ctrl-s`、Vim モードでは `:w` に割り当てられています）。

マルチバッファー内では、マルチカーソルを使用して各ファイルを一括編集すると便利です。複数の箇所を一度に編集するには、マウス（macOS では `option-click`、Windows/Linux では `alt-click`）またはキーボードで選択します。macOS の `cmd-d`、Windows/Linux の `ctrl-d`、または Vim モードの `gl` を使用すると、カーソル下の単語の次の一致箇所を選択できます。

すべての一致箇所を一括編集するには、{#action
editor::SelectAllMatches} コマンド（macOS では `cmd-shift-l`、Windows/Linux では `ctrl-shift-l`、Vim モードでは `g a`）を実行して選択します。

## ソースファイルへの移動

抜粋（Excerpt）間の区切り線をクリックするか、抜粋内にカーソルを置いて {#action editor::OpenExcerpts} コマンドを実行します。注意: 複数のカーソルを使用している場合、マルチバッファー内の各カーソルが位置しているソースファイルがそれぞれ開かれます。

マウスのダブルクリックで抜粋元のソースファイルを開く動作はデフォルトで無効になっていますが、次の設定を行うことで有効化できます:
`"double_click_in_multibuffer": "open"`

## プロジェクト内検索

{#action pane::ToggleSearch}（macOS では `cmd-shift-f`、Windows/Linux では `ctrl-shift-f`、Vim モードでは `g/`）を実行すると、プロジェクト全体を検索できます。検索が完了すると、結果が新しいマルチバッファーに表示されます。プロジェクト全体の一致行ごとに抜粋が表示されます。

## 診断

適切な言語サーバーがインストールされている場合、診断（Diagnostics）ペインにプロジェクト全体のすべてのエラーを表示できます。ステータスバーのアイコンをクリックするか、{#action diagnostics::Deploy} コマンド（macOS では `cmd-shift-m`、Windows/Linux では `ctrl-shift-m`、Vim モードでは `:clist`）を実行して開きます。

## 参照箇所の検索

言語サーバーがインストールされている場合、{#action editor::FindReferences} コマンド（macOS では `cmd-click`、Windows/Linux では `ctrl-click`、Vim モードでは `g A`）を使用して、カーソル下のシンボルのすべての参照箇所を検索できます。

言語サーバーの動作によっては、複数の定義候補が存在する場合に {#action editor::GoToDefinition} や {#action editor::GoToTypeDefinition} などのコマンドもマルチバッファーを開きます。
