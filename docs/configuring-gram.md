# 設定

エディターの各機能の設定方法については、以下の個別ページで説明しています。

- [テーマ](./themes.md)
- [キーバインド](./key-bindings.md)
- [タスク](./tasks.md)
- [拡張機能](./extensions.md)

## 設定エディター

設定エディターは、{#kb gram::OpenSettings} キーバインド、またはコマンドパレットの `gram: open settings` アクションから開くことができます。ローカル設定、ユーザー設定、プロジェクト設定をカスタマイズできます。

> Gram がサポートするすべての設定が現在設定エディターから利用できるわけではありません。言語フォーマッターなどのより複雑な設定は、JSON 設定ファイル {#kb
> gram::OpenSettingsFile} からのみ変更できます。

## ユーザー設定ファイル

`settings.jsonc` ファイルは {#kb gram::OpenSettingsFile} で開くことができます。デフォルトでは `~/.config/gram/settings.jsonc` に配置されますが、Linux で環境変数 `XDG_CONFIG_HOME` が設定されている場合は `$XDG_CONFIG_HOME/gram/settings.json` に配置されます。

ユーザー設定ファイルに追加した内容は、プロジェクト内のローカル設定とマージされます。

### デフォルト設定

設定エディターに表示されている値はデフォルト値です。コマンドパレットから {#action gram::OpenDefaultSettings} を実行して、JSON 形式でデフォルト値を確認することもできます。

言語サーバーを提供する拡張機能は、それらの言語サーバー用のデフォルト設定を提供する場合もあります。

## プロジェクト設定ファイル

ユーザーファイルと同様に、コマンドパレットから {#action gram::OpenProjectSettings} を実行してプロジェクト設定ファイルを開くことができます。これにより、`.gram/settings.jsonc` を含む `.gram` ディレクトリが作成されます。

大半のプロジェクトではルートに 1 つの設定ファイルがあれば十分ですが、必要に応じてサブディレクトリにローカル設定ファイルを追加することも可能です。ローカルファイルにはすべての設定を指定できるわけではなく、エディターの挙動や言語ツールに影響を与える設定のみを指定できます。たとえば `tab_size` や `formatter` などは設定できますが、`theme` や `vim_mode` などは設定できません。

設定ファイルの構文は、`//` コメントを許可する JSON のスーパーセット（JSONC）です。

## リリースチャンネル別の上書き

最上位に `stable` または `dev` オブジェクトを追加することで、特定のリリースチャンネルを対象とした上書きを行えます。

これらは基本設定にマージされ、指定されたビルドの起動時にこれらのキーの設定が優先されます。例:

```json
{
  "theme": "sunset",
  "vim_mode": false,
  "stable": {
    "theme": "gram-dark",
  },
}
```

この設定では、Dev チャンネルではすべての基本設定が維持され、Stable チャンネルでは `gram-dark` に切り替わります。

設定エディターで設定を変更すると、常にすべてのチャンネルにその変更が適用されます。

# 設定一覧

以下に、Gram でサポートされている多数の設定の詳細を網羅して説明します。

## Active Pane Modifiers

- 説明: アクティブなペインに適用されるスタイル設定。
- 設定: `active_pane_modifiers`
- デフォルト:

```json
{
  "active_pane_modifiers": {
    "border_size": 0.0,
    "inactive_opacity": 1.0,
  },
}
```

### Border size

- 説明: アクティブなペインを囲むボーダーのサイズ。0 に設定すると、アクティブなペインにボーダーは表示されません。ボーダーは内側に描画されます。
- 設定: `border_size`
- デフォルト: `0.0`

**選択肢**

0 以上の `float` 値

### Inactive Opacity

- 説明: 非アクティブなパネルの不透明度。1.0 に設定すると、非アクティブなペインはアクティブなペインと同じ不透明度になります。0 に設定すると、非アクティブなペインのコンテンツはまったく表示されなくなります。値は [0.0, 1.0] の範囲に制限されます。
- 設定: `inactive_opacity`
- デフォルト: `1.0`

**選択肢**

`float` 値

## Bottom Dock Layout

- 説明: 左右のドックに対するボトムドックのレイアウトを制御します。
- 設定: `bottom_dock_layout`
- デフォルト: `"contained"`

**選択肢**

1. ボトムドックを包含し、左右のドックにウィンドウ全体の高さを割り当てます。

```json
{
  "bottom_dock_layout": "contained",
}
```

2. ボトムドックにウィンドウ全体の幅を割り当て、左右のドックを切り詰めます。

```json
{
  "bottom_dock_layout": "full",
}
```

3. ボトムドックを左寄せにし、左ドックを切り詰めて右ドックにウィンドウ全体の高さを割り当てます。

```json
{
  "bottom_dock_layout": "left_aligned",
}
```

4. ボトムドックを右寄せにし、左ドックにウィンドウ全体の高さを割り当てて右ドックを切り詰めます。

```json
{
  "bottom_dock_layout": "right_aligned",
}
```

## Allow Rewrap

- 説明: 現在の言語スコープで {#action editor::Rewrap} アクションを許可する場所を制御します。
- 設定: `allow_rewrap`
- デフォルト: `"in_comments"`

**選択肢**

1. コメント内でのみ再折り返しを許可:

```json
{
  "allow_rewrap": "in_comments",
}
```

2. 選択範囲内でのみ再折り返しを許可:

```json
{
  "allow_rewrap": "in_selections",
}
```

3. 任意の場所で再折り返しを許可:

```json
{
  "allow_rewrap": "anywhere",
}
```

注意: Vim モードではすでにすべての場所で再折り返しが許可されているため、この設定は効果がありません。

## Auto Indent

- 説明: 入力中にコンテキストに基づいてインデントを調整するかどうか。言語ごとに個別に指定できます。
- 設定: `auto_indent`
- デフォルト: `true`

**選択肢**

`boolean` 値

## Auto Indent On Paste

- 説明: 貼り付けたコンテンツのインデントをコンテキストに基づいて調整するかどうか。
- 設定: `auto_indent_on_paste`
- デフォルト: `true`

**選択肢**

`boolean` 値

## Auto Install extensions

Gram では拡張機能を手動でインストールする必要があります。

現在インストールされている拡張機能の名前は、[拡張機能のインストール先](./extensions/installing-extensions.md#installation-location) 配下のサブフォルダーを一覧表示することで確認できます。

macOS の場合:

```sh
ls ~/Library/Application\ Support/Gram/extensions/installed/
```

Linux の場合:

```sh
ls ~/.local/share/gram/extensions/installed
```

## Autosave

- 説明: 編集されたバッファーを自動保存するタイミング。
- 設定: `autosave`
- デフォルト: `off`

**選択肢**

1. 自動保存を無効化するには、`off` に設定します:

```json
{
  "autosave": "off",
}
```

2. フォーカスが切り替わったときに自動保存するには、`on_focus_change` を使用します:

```json
{
  "autosave": "on_focus_change",
}
```

3. アクティブウィンドウが切り替わったときに自動保存するには、`on_window_change` を使用します:

```json
{
  "autosave": "on_window_change",
}
```

4. 一定時間操作がなかった後に自動保存するには、`after_delay` を使用します:

```json
{
  "autosave": {
    "after_delay": {
      "milliseconds": 1000,
    },
  },
}
```

未保存のタブを閉じる際は、設定された待機時間よりも前であっても保存がトリガーされる点に注意してください。

## Autoscroll on Clicks

- 説明: 表示可能なテキスト領域の端付近をクリックしたときにスクロールするかどうか。
- 設定: `autoscroll_on_clicks`
- デフォルト: `false`

**選択肢**

`boolean` 値

## Auto Signature Help

- 説明: 括弧の内部にいるときに、エディター内にメソッドシグネチャを表示します。
- 設定: `auto_signature_help`
- デフォルト: `false`

**選択肢**

`boolean` 値

### Show Signature Help After Edits

- 説明: 補完の確定後や括弧ペアの挿入後にシグネチャヘルプを表示するかどうか。`auto_signature_help` が有効な場合、この設定も有効として扱われます。
- 設定: `show_signature_help_after_edits`
- デフォルト: `false`

**選択肢**

`boolean` 値

## Base Keymap

- 説明: ベースとなるキーバインドスキーム。ベースキーマップはユーザーキーマップで上書きできます。
- 設定: `base_keymap`
- デフォルト: `Emacs`

**選択肢**

1. VS Code

```json
{
  "base_keymap": "VSCode",
}
```

2. Atom

```json
{
  "base_keymap": "Atom",
}
```

3. JetBrains

```json
{
  "base_keymap": "JetBrains",
}
```

4. None

```json
{
  "base_keymap": "None",
}
```

5. Sublime Text

```json
{
  "base_keymap": "SublimeText",
}
```

6. TextMate

```json
{
  "base_keymap": "TextMate",
}
```

7. Emacs

```json
{
  "base_keymap": "Emacs",
}
```

8. Minimal

```json
{
  "base_keymap": "Minimal",
}
```

## Buffer Font Family

- 説明: エディター内のテキスト描画に使用するフォント名。
- 設定: `buffer_font_family`
- デフォルト: `.GramMono`

**選択肢**

ユーザーのシステムにインストールされている任意のフォントファミリー名、または `".GramMono"`。

## Buffer Font Features

- 説明: エディター内のテキストに対して有効にする OpenType 機能。
- 設定: `buffer_font_features`
- デフォルト: `null`
- プラットフォーム: macOS および Windows。

**選択肢**

Gram は、指定されたバッファーまたはターミナルのフォントに対して有効化または無効化できるすべての OpenType 機能、およびフォント機能の値の設定をサポートしています。

たとえば、フォントの合字（リガチャ）を無効化するには、設定に以下を追加します:

```json
{
  "buffer_font_features": {
    "calt": false,
  },
}
```

また、`cv01` を `7` に設定するなど、その他の OpenType 機能を設定することもできます:

```json
{
  "buffer_font_features": {
    "cv01": 7,
  },
}
```

## Buffer Font Fallbacks

- 説明: バッファーテキストのフォントフォールバックを設定します。プラットフォームのデフォルトのフォールバックとマージされます。
- 設定: `buffer_font_fallbacks`
- デフォルト: `null`
- プラットフォーム: macOS および Windows。

**選択肢**

たとえば、フォールバックとして `Nerd Font` を使用するには、設定に以下を追加します:

```json
{
  "buffer_font_fallbacks": ["Nerd Font"],
}
```

## Buffer Font Size

- 説明: エディター内のテキストのデフォルトフォントサイズ。
- 設定: `buffer_font_size`
- デフォルト: `15`

**選択肢**

`6` から `100` ピクセル（両端を含む）のフォントサイズ

## Buffer Font Weight

- 説明: エディター内のテキストのデフォルトフォント太さ（ウェイト）。
- 設定: `buffer_font_weight`
- デフォルト: `400`

**選択肢**

`100` から `900` までの `integer` 値

## Buffer Line Height

- 説明: エディター内のテキストのデフォルトの行の高さ。
- 設定: `buffer_line_height`
- デフォルト: `"comfortable"`

**選択肢**

`"standard"`、`"comfortable"`、または `{ "custom": float }`（`1` はコンパクト、`2` はゆったり）

## Centered Layout

- 説明: センタリングレイアウトモードの設定。
- 設定: `centered_layout`
- デフォルト:

```json
"centered_layout": {
  "left_padding": 0.2,
  "right_padding": 0.2,
}
```

**選択肢**

`left_padding` および `right_padding` オプションは、センタリングレイアウトモードがアクティブになった際の、ワークスペースに対する中央ペインの左右のパディングの相対的な幅を定義します。有効な値の範囲は `0` から `0.4` です。

## Close on File Delete

- 説明: 対応するファイルがディスクから削除されたときに、エディタータブを自動的に閉じるかどうか。
- 設定: `close_on_file_delete`
- デフォルト: `false`

**選択肢**

`boolean` 値

有効にすると、ファイルシステムから削除されたファイルのタブが自動的に閉じられます。これは、頻繁に作成・削除される一時ファイルやスクラッチファイルを扱うワークフローで特に便利です。無効（デフォルト）の場合、削除されたファイルはタブタイトルに取り消し線が引かれた状態で開いたまま残ります。

注意: ダーティファイル（未保存の変更があるファイル）は、この設定が有効であっても自動的に閉じられることはないため、未保存の作業が失われる心配はありません。

## Confirm Quit

- 説明: アプリケーションを終了する前にユーザーに確認を促すかどうか。
- 設定: `confirm_quit`
- デフォルト: `false`

**選択肢**

`boolean` 値

## Diagnostics Max Severity

- 説明: エディターに表示される診断情報をフィルタリングするためのレベル。
- 設定: `diagnostics_max_severity`
- デフォルト: `null`

**選択肢**

1. すべての診断を許可（デフォルト）:

```json
{
  "diagnostics_max_severity": "all",
}
```

2. エラーのみ表示:

```json
{
  "diagnostics_max_severity": "error",
}
```

3. エラーと警告を表示:

```json
{
  "diagnostics_max_severity": "warning",
}
```

4. エラー、警告、情報を表示:

```json
{
  "diagnostics_max_severity": "info",
}
```

5. ヒントを含むすべてを表示:

```json
{
  "diagnostics_max_severity": "hint",
}
```

## Direnv Integration

- 説明: [direnv](https://direnv.net/) 連携の設定。`direnv` がインストールされている必要があります。`direnv` 連携により、`direnv` 構成によって設定された環境変数を使用して、言語サーバーをインストールする代わりに `$PATH` から検出できるようになります。また、これらの環境変数をタスク内で使用することも可能になります。
- 設定: `load_direnv`
- デフォルト: `"direct"`

**選択肢**

選択可能なオプションは 3 つあります:

1. `shell_hook`: シェルフックを使用して direnv を読み込みます。ディレクトリに入ったときに direnv が有効化されることに依存します。POSIX シェルおよび fish をサポートします。
2. `direct`: `direnv export json` を使用して direnv を読み込みます。シェルフックに依存せずに direnv を直接読み込むため、若干の不整合が生じる可能性があります。任意のシェルで direnv を動作させることができます。
3. `disabled`: シェル環境は自動的に読み込まれません。direnv を使用するには手動で呼び出す必要があります（例: `direnv exec`）。

## Double Click In Multibuffer

- 説明: マルチバッファー内の抜粋部分（単一バッファーの一部）をダブルクリックしたときの動作。
- 設定: `double_click_in_multibuffer`
- デフォルト: `"select"`

**選択肢**

1. 通常のバッファーと同様に動作し、単語全体を選択（デフォルト）:

```json
{
  "double_click_in_multibuffer": "select",
}
```

2. クリックした抜粋部分を新しいタブの新規バッファーとして開く:

```json
{
  "double_click_in_multibuffer": "open",
}
```

「open」の場合でも、ダブルクリック時に `Alt` を押したままにすることで通常の選択動作を行えます。

## Drop Target Size

- 説明: ドロップされたファイルを分割ペインとして開くエディター内のドロップターゲットの相対サイズ（0〜0.5）。たとえば 0.25 の場合、ペインの上部/下部 4 分の 1 にドロップすると垂直分割が使用され、ペインの左側/右側 4 分の 1 にドロップすると水平分割が使用されます。
- 設定: `drop_target_size`
- デフォルト: `0.2`

**選択肢**

`0` から `0.5` までの `float` 値

## Current Line Highlight

- 説明: エディター内で現在の行をハイライトする方法。
- 設定: `current_line_highlight`
- デフォルト: `all`

**選択肢**

1. 現在の行をハイライトしない:

```json
"current_line_highlight": "none"
```

2. ガター領域をハイライト:

```json
"current_line_highlight": "gutter"
```

3. エディター領域をハイライト:

```json
"current_line_highlight": "line"
```

4. 行全体をハイライト:

```json
"current_line_highlight": "all"
```

## Selection Highlight

- 説明: エディター内で選択されたテキストのすべての出現箇所をハイライトするかどうか。
- 設定: `selection_highlight`
- デフォルト: `true`

## Rounded Selection

- 説明: テキストの選択範囲の角を丸くするかどうか。
- 設定: `rounded_selection`
- デフォルト: `true`

## Cursor Blink

- 説明: カーソルを点滅させるかどうか。
- 設定: `cursor_blink`
- デフォルト: `true`

**選択肢**

`boolean` 値

## Cursor Shape

- 説明: デフォルトエディターのカーソル形状。
- 設定: `cursor_shape`
- デフォルト: `bar`

**選択肢**

1. 垂直バー:

```json
"cursor_shape": "bar"
```

2. 後続の文字を囲むブロック:

```json
"cursor_shape": "block"
```

3. 後続の文字に沿って引かれる下線 / アンダースコア:

```json
"cursor_shape": "underline"
```

4. 後続の文字の周囲に描画される中空ボックス:

```json
"cursor_shape": "hollow"
```

## Gutter

- 説明: エディターガターの設定。
- 設定: `gutter`
- デフォルト:

```json
{
  "gutter": {
    "line_numbers": true,
    "runnables": true,
    "breakpoints": true,
    "folds": true,
    "min_line_number_digits": 4,
  },
}
```

**選択肢**

- `line_numbers`: ガターに行番号を表示するかどうか
- `runnables`: ガターに実行可能ボタンを表示するかどうか
- `breakpoints`: ガターにブレークポイントを表示するかどうか
- `folds`: ガターに折りたたみボタンを表示するかどうか
- `min_line_number_digits`: ガター内でスペースを確保する最小文字数

## Hide Mouse

- 説明: エディターまたは入力ボックス内でマウスカーソルを非表示にするタイミングを決定します。
- 設定: `hide_mouse`
- デフォルト: `on_typing_and_movement`

**選択肢**

1. マウスカーソルを非表示にしない:

```json
"hide_mouse": "never"
```

2. 入力時のみ非表示:

```json
"hide_mouse": "on_typing"
```

3. 入力時およびカーソル移動時の両方で非表示:

```json
"hide_mouse": "on_typing_and_movement"
```

## Snippet Sort Order

- 説明: 他の補完項目と比較してスニペットをどのように並べ替えるかを決定します。
- 設定: `snippet_sort_order`
- デフォルト: `inline`

**選択肢**

1. 補完リストの先頭にスニペットを配置:

```json
"snippet_sort_order": "top"
```

2. 優先度を付けずにスニペットを通常どおり配置:

```json
"snippet_sort_order": "inline"
```

3. 補完リストの末尾にスニペットを配置:

```json
"snippet_sort_order": "bottom"
```

4. 補完リストにスニペットを一切表示しない:

```json
"snippet_sort_order": "none"
```

## Editor Scrollbar

- 説明: エディターのスクロールバーおよびその中の各種要素を表示するかどうか。
- 設定: `scrollbar`
- デフォルト:

```json
"scrollbar": {
  "show": "auto",
  "cursors": true,
  "git_diff": true,
  "search_results": true,
  "selected_text": true,
  "selected_symbol": true,
  "diagnostics": "all",
  "axes": {
    "horizontal": true,
    "vertical": true,
  },
},
```

### Show Mode

- 説明: エディターのスクロールバーを表示するタイミング。
- 設定: `show`
- デフォルト: `auto`

**選択肢**

1. 重要な情報がある場合にスクロールバーを表示するか、システムの設定された動作に従う:

```json
"scrollbar": {
  "show": "auto"
}
```

2. システムの設定された動作に合わせる:

```json
"scrollbar": {
  "show": "system"
}
```

3. 常にスクロールバーを表示:

```json
"scrollbar": {
  "show": "always"
}
```

4. スクロールバーを表示しない:

```json
"scrollbar": {
  "show": "never"
}
```

### Git Diff Indicators

- 説明: スクロールバーに Git diff インジケーターを表示するかどうか。
- 設定: `git_diff`
- デフォルト: `true`

Git diff インジケーターは、Git の HEAD と比較して追加、変更、または削除された行を示す色付きマークとして表示されます。

**選択肢**

`boolean` 値

### Search Results Indicators

- 説明: スクロールバーにバッファー検索結果を表示するかどうか。
- 設定: `search_results`
- デフォルト: `true`

検索結果インジケーターは、現在の検索クエリに一致するファイル内のすべての位置を示すマークとして表示されます。

**選択肢**

`boolean` 値

### Selected Text Indicators

- 説明: スクロールバーに選択テキストの出現箇所を表示するかどうか。
- 設定: `selected_text`
- デフォルト: `true`

選択テキストインジケーターは、現在選択されているテキストのファイル全体でのすべての出現箇所を示すマークとして表示されます。

**選択肢**

`boolean` 値

### Selected Symbols Indicators

- 説明: スクロールバーに選択シンボルの出現箇所を表示するかどうか。
- 設定: `selected_symbol`
- デフォルト: `true`

選択シンボルインジケーターは、現在選択されているシンボル（関数名や変数名など）のファイル全体でのすべての出現箇所を示すマークとして表示されます。

**選択肢**

`boolean` 値

### Diagnostics

- 説明: スクロールバーに表示する診断インジケーター。
- 設定: `diagnostics`
- デフォルト: `all`

診断インジケーターは、ファイル内の対応する行位置にエラー、警告、およびその他の言語サーバー診断情報を示す色付きマークとして表示されます。

**選択肢**

1. すべての診断を表示:

```json
{
  "show_diagnostics": "all",
}
```

2. 診断を一切表示しない:

```json
{
  "show_diagnostics": "off",
}
```

3. エラーのみ表示:

```json
{
  "show_diagnostics": "error",
}
```

4. エラーと警告のみ表示:

```json
{
  "show_diagnostics": "warning",
}
```

5. エラー、警告、情報のみ表示:

```json
{
  "show_diagnostics": "info",
}
```

### Axes

- 説明: 各軸のスクロールバーを強制的に有効化または無効化します。
- 設定: `axes`
- デフォルト:

```json
"scrollbar": {
  "axes": {
    "horizontal": true,
    "vertical": true,
  },
}
```

#### Horizontal

- 説明: false の場合、水平スクロールバーを強制的に無効化します。それ以外の場合は他の設定に従います。
- 設定: `horizontal`
- デフォルト: `true`

**選択肢**

`boolean` 値

#### Vertical

- 説明: false の場合、垂直スクロールバーを強制的に無効化します。それ以外の場合は他の設定に従います。
- 設定: `vertical`
- デフォルト: `true`

**選択肢**

`boolean` 値

## Minimap

- 説明: ドキュメントの全体像を提供するエディターのミニマップに関する設定。
- 設定: `minimap`
- デフォルト:

```json
{
  "minimap": {
    "show": "never",
    "thumb": "always",
    "thumb_border": "left_open",
    "current_line_highlight": null,
  },
}
```

### Show Mode

- 説明: エディターにミニマップを表示するタイミング。
- 設定: `show`
- デフォルト: `never`

**選択肢**

1. 常にミニマップを表示:

```json
{
  "show": "always",
}
```

2. エディターのスクロールバーが表示されている場合にミニマップを表示:

```json
{
  "show": "auto",
}
```

3. ミニマップを表示しない:

```json
{
  "show": "never",
}
```

### Thumb Display

- 説明: ミニマップ内でサム（エディターの表示領域）を表示するタイミング。
- 設定: `thumb`
- デフォルト: `always`

**選択肢**

1. ミニマップ上にホバーしたときにミニマップサムを表示:

```json
{
  "thumb": "hover",
}
```

2. 常にミニマップサムを表示:

```json
{
  "thumb": "always",
}
```

### Thumb Border

- 説明: ミニマップサムのボーダーの外観。
- 設定: `thumb_border`
- デフォルト: `left_open`

**選択肢**

1. サムの四辺すべてにボーダーを表示:

```json
{
  "thumb_border": "full",
}
```

2. 左側を除くすべての辺にボーダーを表示:

```json
{
  "thumb_border": "left_open",
}
```

3. 右側を除くすべての辺にボーダーを表示:

```json
{
  "thumb_border": "right_open",
}
```

4. 左側のみにボーダーを表示:

```json
{
  "thumb_border": "left_only",
}
```

5. ボーダーなしでサムを表示:

```json
{
  "thumb_border": "none",
}
```

### Current Line Highlight

- 説明: ミニマップ内で現在の行をハイライトする方法。
- 設定: `current_line_highlight`
- デフォルト: `null`

**選択肢**

1. エディターの現在の行ハイライト設定を継承:

```json
{
  "minimap": {
    "current_line_highlight": null,
  },
}
```

2. ミニマップ内で現在の行をハイライト:

```json
{
  "minimap": {
    "current_line_highlight": "line",
  },
}
```

または

```json
{
  "minimap": {
    "current_line_highlight": "all",
  },
}
```

3. ミニマップ内で現在の行をハイライトしない:

```json
{
  "minimap": {
    "current_line_highlight": "gutter",
  },
}
```

または

```json
{
  "minimap": {
    "current_line_highlight": "none",
  },
}
```

## Editor Tab Bar

- 説明: エディターのタブバーに関する設定。
- 設定: `tab_bar`
- デフォルト:

```json
"tab_bar": {
  "show": true,
  "show_nav_history_buttons": true,
  "show_tab_bar_buttons": "on_pane_focus",
  "show_pinned_tabs_in_separate_row": false,
  "wrap_tabs": false
}
```

### Show

- 説明: エディター内にタブバーを表示するかどうか。
- 設定: `show`
- デフォルト: `true`

**選択肢**

`boolean` 値

### Navigation History Buttons

- 説明: ナビゲーション履歴ボタンを表示するかどうか。
- 設定: `show_nav_history_buttons`
- デフォルト: `true`

**選択肢**

`boolean` 値

### Tab Bar Buttons

- 説明: タブバーボタンを表示するタイミング。
- 設定: `show_tab_bar_buttons`
- デフォルト: `on_pane_focus`

**選択肢**

`boolean` 値

### Pinned Tabs in Separate Row

- 説明: ピン留めされたタブを上の行、ピン留めされていないタブを下の行に分けて表示するかどうか。無効にすると、すべてのタブが 1 行に表示されます。
- 設定: `show_pinned_tabs_in_separate_row`
- デフォルト: `false`

**選択肢**

`boolean` 値

### Wrap Tabs

- 説明: タブバーがあふれたときに、水平スクロールする代わりにタブを複数行に折り返して表示するかどうか。
- 設定: `wrap_tabs`
- デフォルト: `false`

**選択肢**

`boolean` 値

## Editor Tabs

- 説明: エディタータブの設定。
- 設定: `tabs`
- デフォルト:

```json
"tabs": {
  "close_position": "right",
  "file_icons": false,
  "git_status": false,
  "activate_on_close": "history",
  "show_close_button": "hover",
  "show_diagnostics": "off"
},
```

### Close Position

- 説明: タブ内の閉じるボタンを表示する位置。
- 設定: `close_position`
- デフォルト: `right`

**選択肢**

1. 閉じるボタンを右側に表示:

```json
{
  "close_position": "right",
}
```

2. 閉じるボタンを左側に表示:

```json
{
  "close_position": "left",
}
```

### File Icons

- 説明: タブにファイルアイコンを表示するかどうか。
- 設定: `file_icons`
- デフォルト: `false`

### Git Status

- 説明: タブに Git のファイルステータスを表示するかどうか。
- 設定: `git_status`
- デフォルト: `false`

### Activate on close

- 説明: 現在のタブを閉じた後の動作。
- 設定: `activate_on_close`
- デフォルト: `history`

**選択肢**

1. 直前に開いていたタブをアクティブ化:

```json
{
  "activate_on_close": "history",
}
```

2. 右隣のタブが存在する場合はアクティブ化:

```json
{
  "activate_on_close": "neighbour",
}
```

3. 左隣のタブが存在する場合はアクティブ化:

```json
{
  "activate_on_close": "left_neighbour",
}
```

### Show close button

- 説明: タブの閉じるボタンの表示動作を制御します。
- 設定: `show_close_button`
- デフォルト: `hover`

**選択肢**

1. タブにホバーしたときのみ表示:

```json
{
  "show_close_button": "hover",
}
```

2. 常に表示:

```json
{
  "show_close_button": "always",
}
```

3. ホバー時も含め、一切表示しない:

```json
{
  "show_close_button": "hidden",
}
```

### Show Diagnostics

- 説明: タブに診断インジケーターを表示するかどうか。この設定はファイルアイコンが有効な場合にのみ機能し、診断上の問題があるどのファイルにマークを付けるかを制御します。
- 設定: `show_diagnostics`
- デフォルト: `off`

**選択肢**

1. どのファイルにもマークを付けない:

```json
{
  "show_diagnostics": "off",
}
```

2. エラーのあるファイルのみマークを付ける:

```json
{
  "show_diagnostics": "errors",
}
```

3. エラーおよび警告のあるファイルにマークを付ける:

```json
{
  "show_diagnostics": "all",
}
```

### Show Inline Code Actions

- 説明: バッファー行の先頭にコードアクションボタンを表示するかどうか。
- 設定: `inline_code_actions`
- デフォルト: `true`

**選択肢**

`boolean` 値

### Drag And Drop Selection

- 説明: バッファー内でのドラッグ＆ドロップによるテキスト選択を許可するかどうか。`delay` はドラッグ＆ドロップが許可されるまでに経過する必要があるミリ秒単位の時間です。それ以外の場合は新しいテキスト選択が作成されます。
- 設定: `drag_and_drop_selection`
- デフォルト:

```json
"drag_and_drop_selection": {
  "enabled": true,
  "delay": 300
}
```

## Editor Toolbar

- 説明: エディターツールバーに各種要素を表示するかどうか。
- 設定: `toolbar`
- デフォルト:

```json
"toolbar": {
  "breadcrumbs": true,
  "quick_actions": true,
  "selections_menu": true,
  "code_actions": false
},
```

**選択肢**

各オプションは、特定のツールバー要素の表示を制御します。すべての要素が非表示の場合、エディターツールバーは表示されません。

## Use System Tabs

- 説明: ユーザーのタブ設定に基づいてウィンドウ同士をタブとしてまとめることを許可するかどうか（macOS のみ）。
- 設定: `use_system_window_tabs`
- デフォルト: `false`

**選択肢**

この設定は、macOS のネイティブウィンドウタブ機能との統合を有効化します。`true` に設定すると、Gram ウィンドウはユーザーが設定したシステム全体のタブ設定（「常に」「フルスクリーン時のみ」「しない」など）に従って、単一の macOS ウィンドウ内のタブとしてグループ化されます。この設定は macOS でのみ利用可能です。

## Enable Language Server

- 説明: コードインテリジェンスを提供するために言語サーバーを使用するかどうか。
- 設定: `enable_language_server`
- デフォルト: `true`

**選択肢**

`boolean` 値

## Ensure Final Newline On Save

- 説明: ファイル末尾の空白のみを含む行を削除し、末尾に改行が 1 つだけ存在するようにします。
- 設定: `ensure_final_newline_on_save`
- デフォルト: `true`

**選択肢**

`boolean` 値

## Expand Excerpt Lines

- 説明: マルチバッファー内で抜粋を展開する際のデフォルト行数。
- 設定: `expand_excerpt_lines`
- デフォルト: `5`

**選択肢**

正の `integer` 値

## Excerpt Context Lines

- 説明: マルチバッファー内で抜粋を表示する際に含めるコンテキスト（前後の行）の行数。
- 設定: `excerpt_context_lines`
- デフォルト: `2`

**選択肢**

1 から 32 までの正の `integer` 値。この範囲外の値は範囲内にクランプされます。

## Extend Comment On Newline

- 説明: 前の行がコメントの場合、改行時に次の行もコメントとして開始するかどうか。
- 設定: `extend_comment_on_newline`
- デフォルト: `true`

**選択肢**

`boolean` 値

## Status Bar

- 説明: ステータスバー内の各種要素を制御します。ステータスバー内の一部の項目には、別の場所で定義されている独自の設定がある点に注意してください。
- 設定: `status_bar`
- デフォルト:

```json
"status_bar": {
  "show": true,
  "position": "bottom",
  "active_language_button": true,
  "cursor_position_button": true,
  "line_endings_button": false,
  "active_encoding_button": "non_utf8"
},
```

## LSP

- 説明: 言語サーバーの設定。
- 設定: `lsp`
- デフォルト: `null`

**選択肢**

特定の言語サーバーに対して、以下の設定を上書きできます:

- `initialization_options`
- `settings`

言語サーバーの設定を上書きするには、その言語サーバーの名前のエントリを `lsp` の値に追加します。

一部のオプションは `initialization_options` を介して言語サーバーに渡されます。これらは言語サーバーの起動時に指定する必要があるオプションであり、変更した場合は言語サーバーの再起動が必要です。

たとえば `rust-analyzer` に `check` オプションを渡すには、以下の設定を使用します:

```json
"lsp": {
  "rust-analyzer": {
    "initialization_options": {
      "check": {
        "command": "clippy" // rust-analyzer.check.command (default: "check")
      }
    }
  }
}
```

その他のオプションは実行時に変更可能であり、`settings` 配下に配置する必要があります:

```json
"lsp": {
  "yaml-language-server": {
    "settings": {
      "yaml": {
        "keyOrdering": true // Enforces alphabetical ordering of keys in maps
      }
    }
  }
}
```

## Global LSP Settings

- 説明: すべての言語サーバーに適用されるグローバル LSP 設定。
- 設定: `global_lsp_settings`
- デフォルト:

```json
{
  "global_lsp_settings": {
    "button": true,
  },
}
```

**選択肢**

- `button`: ステータスバーに LSP ステータスボタンを表示するかどうか

## LSP Highlight Debounce

- 説明: 現在のカーソル位置に基づいて言語サーバーからハイライトを取得する前のデバウンス待機時間（ミリ秒）。
- 設定: `lsp_highlight_debounce`
- デフォルト: `75`

**選択肢**

ミリ秒を表す `integer` 値

## Features

- 説明: グローバルに有効化または無効化できる機能。
- 設定: `features`
- デフォルト:

```json
{
  "features": {},
}
```

## Format On Save

- 説明: 保存前にバッファーのフォーマットを実行するかどうか。
- 設定: `format_on_save`
- デフォルト: `on`

**選択肢**

1. `on`: `formatter` 設定に従って保存時フォーマットを有効化します:

```json
{
  "format_on_save": "on",
}
```

2. `off`: 保存時フォーマットを無効化します:

```json
{
  "format_on_save": "off",
}
```

## Formatter

- 説明: バッファーのフォーマットを実行する方法。
- 設定: `formatter`
- デフォルト: `auto`

**選択肢**

1. 現在の言語サーバーを使用するには、`"language_server"` を使用します:

```json
{
  "formatter": "language_server",
}
```

2. または外部コマンドを使用するには、`"external"` を使用します。実行するフォーマットプログラムの名前と、プログラムに渡す引数の配列を指定します。バッファーのテキストは標準入力経由でプログラムに渡され、フォーマットされた出力が標準出力に書き込まれる必要があります。たとえば、以下のコマンドは [`sed(1)`](https://linux.die.net/man/1/sed) を使用して末尾のスペースを削除します:

```json
{
  "formatter": {
    "external": {
      "command": "sed",
      "arguments": ["-e", "s/ *$//"],
    },
  },
}
```

3. 外部フォーマッターには、実行時にフォーマット対象バッファーのパスが含まれる `{buffer_path}` プレースホルダーを任意で含めることができます。フォーマッターは標準入力を介してファイルコンテンツを受け取り、再フォーマットして標準出力に出力するため、通常はフォーマット対象のファイル名を知りません。Prettier のようなツールは、コマンドライン引数経由でファイルパスを受け取り、フォーマット動作に反映させることができます。

警告: フォーマッターにファイル名から直接読み取らせるために `{buffer_path}` を使用しないでください。フォーマッターは標準入力からのみ読み取り、ファイルを直接読み書きしてはなりません。

```json
"formatter": {
  "external": {
    "command": "prettier",
    "arguments": ["--stdin-filepath", "{buffer_path}"]
  }
}
```

4. または接続されている言語サーバーから提供されるコードアクションを使用するには、`"code_actions"` を使用します:

```json
{
  "formatter": [
    // Use ESLint's --fix:
    { "code_action": "source.fixAll.eslint" },
    // Organize imports on save:
    { "code_action": "source.organizeImports" },
  ],
}
```

5. または複数のフォーマッターを連続して使用するには、フォーマッターの配列を使用します:

```json
{
  "formatter": [
    { "language_server": { "name": "rust-analyzer" } },
    {
      "external": {
        "command": "sed",
        "arguments": ["-e", "s/ *$//"],
      },
    },
  ],
}
```

ここでは、コードのフォーマットにまず `rust-analyzer` が使用され、続いて sed の呼び出しが行われます。いずれかのフォーマッターが失敗した場合でも、後続のフォーマッターは実行されます。

## Auto close

- 説明: 丸括弧、角括弧、波括弧、単一引用符、二重引用符の開き文字を入力したときに、対応する閉じ文字を自動的に追加するかどうか。
- 設定: `use_autoclose`
- デフォルト: `true`

**選択肢**

`boolean` 値

## Always Treat Brackets As Autoclosed

- 説明: 自動挿入された閉じ括弧文字をエディターがどのように処理するかを制御します。
- 設定: `always_treat_brackets_as_autoclosed`
- デフォルト: `false`

**選択肢**

`boolean` 値

**例**

設定が `true` の場合:

1. エディターに入力: `)))`
2. カーソルを先頭に移動: `^)))`
3. 再び入力: `)))`

結果はデフォルトの動作である `))))))` ではなく、`)))` のままになります。

## File Scan Exclusions

- 設定: `file_scan_exclusions`
- 説明: Gram から完全に除外されるファイルまたはファイルの Glob。ファイルスキャンやファイル検索時にスキップされ、プロジェクトファイルツリーにも表示されません。`file_scan_inclusions` を上書きします。
- デフォルト:

```json
"file_scan_exclusions": [
  "**/.git",
  "**/.svn",
  "**/.hg",
  "**/.jj",
  "**/CVS",
  "**/.DS_Store",
  "**/Thumbs.db",
  "**/.classpath",
  "**/.settings"
],
```

注意: settings.jsonc で `file_scan_exclusions` を指定すると、デフォルト値（上記）が上書きされます。追加の項目を除外したい場合は、デフォルト値のすべてを設定に含める必要があります。

## File Scan Inclusions

- 設定: `file_scan_inclusions`
- 説明: Git で無視されている場合でも、Gram によって含められるファイルまたはファイルの Glob。Git で追跡されていないものの、プロジェクトにとって重要なファイルに便利です。広すぎる Glob は Gram のファイルスキャンを遅くする可能性がある点に注意してください。`file_scan_exclusions` はこれらの包含設定よりも優先されます。
- デフォルト:

```json
"file_scan_inclusions": [".env*"],
```

## File Types

- 設定: `file_types`
- 説明: ファイル名または拡張子に基づいて、Gram がファイルに対応する言語を選択する方法を設定します。Glob エントリーをサポートしています。
- デフォルト:

```json
"file_types": {
  "Shell Script": [".env.*"]
}
```

**例**

すべての `.c` ファイルを C++ として、`MyLockFile` というファイルを TOML として、`Dockerfile` で始まるファイルを Dockerfile として解釈する場合:

```json
{
  "file_types": {
    "C++": ["c"],
    "TOML": ["MyLockFile"],
    "Dockerfile": ["Dockerfile*"],
  },
}
```

## Diagnostics

- 説明: 診断関連機能の設定。
- 設定: `diagnostics`
- デフォルト:

```json
{
  "diagnostics": {
    "include_warnings": true,
    "inline": {
      "enabled": false,
    },
    "update_with_cursor": false,
    "primary_only": false,
    "use_rendered": false,
  },
}
```

### Inline Diagnostics

- 説明: 診断情報をインラインで表示するかどうか。
- 設定: `inline`
- デフォルト:

```json
{
  "diagnostics": {
    "inline": {
      "enabled": false,
      "update_debounce_ms": 150,
      "padding": 4,
      "min_column": 0,
      "max_severity": null,
    },
  },
}
```

**選択肢**

1. インライン診断を有効化。

```json
{
  "diagnostics": {
    "inline": {
      "enabled": true,
    },
  },
}
```

2. 最後の診断更新から一定時間経過するまで診断更新を遅延。

```json
{
  "diagnostics": {
    "inline": {
      "enabled": true,
      "update_debounce_ms": 150,
    },
  },
}
```

3. ソース行の末尾と診断表示の開始位置との間のパディングを設定。

```json
{
  "diagnostics": {
    "inline": {
      "enabled": true,
      "padding": 4,
    },
  },
}
```

4. 指定した列位置でインライン診断を水平方向に揃える。

```json
{
  "diagnostics": {
    "inline": {
      "enabled": true,
      "min_column": 80,
    },
  },
}
```

5. 警告およびエラーの診断のみ表示。

```json
{
  "diagnostics": {
    "inline": {
      "enabled": true,
      "max_severity": "warning",
    },
  },
}
```

## Git

- 説明: Git 関連機能の設定。
- 設定: `git`
- デフォルト:

```json
{
  "git": {
    "git_gutter": "tracked_files",
    "inline_blame": {
      "enabled": true,
    },
    "branch_picker": {
      "show_author_name": true,
    },
    "hunk_style": "staged_hollow",
  },
}
```

### Git Gutter

- 説明: Git ガターを表示するかどうか。
- 設定: `git_gutter`
- デフォルト: `tracked_files`

**選択肢**

1. 追跡されているファイルで Git ガターを表示する

```json
{
  "git": {
    "git_gutter": "tracked_files",
  },
}
```

2. Git ガターを非表示にする

```json
{
  "git": {
    "git_gutter": "hide",
  },
}
```

### Gutter Debounce

- 説明: 変更が Git ガターに反映されるまでのデバウンスしきい値（ミリ秒）。
- 設定: `gutter_debounce`
- デフォルト: `null`

**選択肢**

ミリ秒を表す `integer` 値

例:

```json
{
  "git": {
    "gutter_debounce": 100,
  },
}
```

### Inline Git Blame

- 説明: フォーカス中の行に Git blame の情報をインラインで表示するかどうか。
- 設定: `inline_blame`
- デフォルト:

```json
{
  "git": {
    "inline_blame": {
      "enabled": true,
    },
  },
}
```

**選択肢**

1. インラインの Git blame を無効にする:

```json
{
  "git": {
    "inline_blame": {
      "enabled": false,
    },
  },
}
```

2. カーソルが止まってから一定時間後にのみインラインの Git blame を表示する:

```json
{
  "git": {
    "inline_blame": {
      "delay_ms": 500,
    },
  },
}
```

3. コミットの日付と作成者の隣にコミットの要約を表示する:

```json
{
  "git": {
    "inline_blame": {
      "show_commit_summary": true,
    },
  },
}
```

4. インラインの blame 情報を表示する最小の列位置を指定する:

```json
{
  "git": {
    "inline_blame": {
      "min_column": 80,
    },
  },
}
```

5. 行の末尾とインラインの blame ヒントとの間のパディングを em 単位で設定する:

```json
{
  "git": {
    "inline_blame": {
      "padding": 10,
    },
  },
}
```

### Branch Picker

- 説明: ブランチピッカーに関する設定。
- 設定: `branch_picker`
- デフォルト:

```json
{
  "git": {
    "branch_picker": {
      "show_author_name": false,
    },
  },
}
```

**選択肢**

1. ブランチピッカーに作成者名を表示する:

```json
{
  "git": {
    "branch_picker": {
      "show_author_name": true,
    },
  },
}
```

### Hunk Style

- 説明: 差分ハンクに使うスタイル。
- 設定: `hunk_style`
- デフォルト:

```json
{
  "git": {
    "hunk_style": "staged_hollow",
  },
}
```

**選択肢**

1. ステージングされたハンクを薄く表示し、枠線を付ける:

```json
{
  "git": {
    "hunk_style": "staged_hollow",
  },
}
```

2. ステージングされていないハンクを薄く表示し、枠線を付ける:

```json
{
  "git": {
    "hunk_style": "unstaged_hollow",
  },
}
```

## Go to Definition Fallback

- 説明: {#action editor::GoToDefinition} アクションが定義を見つけられなかったときの動作。
- 設定: `go_to_definition_fallback`
- デフォルト: `"find_all_references"`

**選択肢**

1. 何もしない:

```json
{
  "go_to_definition_fallback": "none",
}
```

2. 同じシンボルの参照を検索する（既定）:

```json
{
  "go_to_definition_fallback": "find_all_references",
}
```

## Hard Tabs

- 説明: タブ文字と複数のスペースのどちらで行をインデントするか。
- 設定: `hard_tabs`
- デフォルト: `false`

**選択肢**

`boolean` 値

## Helix Mode

- 説明: Helix モードを有効にするかどうか。`helix_mode` を有効にすると `vim_mode` も有効になります。詳しくは [Helix のドキュメント](./helix.md)を参照してください。
- 設定: `helix_mode`
- デフォルト: `false`

**選択肢**

`boolean` 値

## Indent Guides

- 説明: インデントガイドに関する設定。インデントガイドは言語ごとに個別に設定できます。
- 設定: `indent_guides`
- デフォルト:

```json
{
  "indent_guides": {
    "enabled": true,
    "line_width": 1,
    "active_line_width": 1,
    "coloring": "fixed",
    "background_coloring": "disabled",
  },
}
```

**選択肢**

1. インデントガイドを無効にする

```json
{
  "indent_guides": {
    "enabled": false,
  },
}
```

2. 特定の言語でインデントガイドを有効にする。

```json
{
  "languages": {
    "Python": {
      "indent_guides": {
        "enabled": true,
      },
    },
  },
}
```

3. インデントの深さに応じた色付け（「レインボーインデント」）を有効にする。インデントの深さごとに使う色はテーマ（テーマキー: `accents`）で定義されています。テーマの上書きでカスタマイズできます。

```json
{
  "indent_guides": {
    "enabled": true,
    "coloring": "indent_aware",
  },
}
```

4. インデントの深さに応じた背景色付け（「レインボーインデント」）を有効にする。インデントの深さごとに使う色はテーマ（テーマキー: `accents`）で定義されています。テーマの上書きでカスタマイズできます。

```json
{
  "indent_guides": {
    "enabled": true,
    "coloring": "indent_aware",
    "background_coloring": "indent_aware",
  },
}
```

## Hover Popover Enabled

- 説明: エディター内のシンボルにマウスを重ねたときに、情報を表示するホバーボックスを表示するかどうか。
- 設定: `hover_popover_enabled`
- デフォルト: `true`

**選択肢**

`boolean` 値

## Hover Popover Delay

- 説明: 情報を表示するホバーボックスを表示するまでの待ち時間（ミリ秒）。
- 設定: `hover_popover_delay`
- デフォルト: `300`

**選択肢**

ミリ秒を表す `integer` 値

## Icon Theme

- 説明: アイコンテーマの設定は2つの形式で指定できます。アイコンテーマ名として指定するか、Gram 内のファイル/フォルダー用の `mode`、`dark`、`light` のアイコンテーマを含むオブジェクトとして指定します。
- 設定: `icon_theme`
- デフォルト: `Gram (Default)`

### Icon Theme Object

- 説明: `mode`、`dark`、`light` を含むオブジェクトでアイコンテーマを指定します。
- 設定: `icon_theme`
- デフォルト:

```json
"icon_theme": {
  "mode": "system",
  "dark": "Gram (Default)",
  "light": "Gram (Default)"
},
```

### Mode

- 説明: アイコンテーマのモードを指定します。
- 設定: `mode`
- デフォルト: `system`

**選択肢**

1. アイコンテーマをダークモードにする

```json
{
  "mode": "dark",
}
```

2. アイコンテーマをライトモードにする

```json
{
  "mode": "light",
}
```

3. アイコンテーマをシステムモードにする

```json
{
  "mode": "system",
}
```

### Dark

- 説明: ダーク用アイコンテーマの名前。
- 設定: `dark`
- デフォルト: `Gram (Default)`

**選択肢**

コマンドパレットで {#action icon_theme_selector::Toggle} アクションを実行すると、現在有効なアイコンテーマ名の一覧を確認できます。

### Light

- 説明: ライト用アイコンテーマの名前。
- 設定: `light`
- デフォルト: `Gram (Default)`

**選択肢**

コマンドパレットで {#action icon_theme_selector::Toggle} アクションを実行すると、現在有効なアイコンテーマ名の一覧を確認できます。

## Image Viewer

- 説明: 画像ビューアー機能の設定。
- 設定: `image_viewer`
- デフォルト:

```json
{
  "image_viewer": {
    "unit": "binary",
  },
}
```

**選択肢**

### Unit

- 説明: 画像ファイルサイズの単位。
- 設定: `unit`
- デフォルト: `"binary"`

**選択肢**

1. 2進単位（KiB、MiB）を使う:

```json
{
  "image_viewer": {
    "unit": "binary",
  },
}
```

2. 10進単位（KB、MB）を使う:

```json
{
  "image_viewer": {
    "unit": "decimal",
  },
}
```

## Inlay hints

- 説明: エディター内にヒント付きの追加テキストを表示するための設定。
- 設定: `inlay_hints`
- デフォルト:

```json
"inlay_hints": {
  "enabled": false,
  "show_type_hints": true,
  "show_parameter_hints": true,
  "show_other_hints": true,
  "show_background": false,
  "edit_debounce_ms": 700,
  "scroll_debounce_ms": 50,
  "toggle_on_modifiers_press": null
}
```

**選択肢**

インレイヒントの問い合わせは、エディター（クライアント）と LSP サーバーの2つの部分で構成されます。上記のインレイ設定を変更してヒントを有効にすると、エディターは特定の種類のヒントを問い合わせ、サーバーからの LSP ヒント更新要求に反応します。この時点で、サーバーはその実装に応じてヒントを返す場合と返さない場合があり、追加の設定が必要になることもあります。該当する LSP サーバーのドキュメントを参照してください。

Gram でインレイヒントが既定で設定されている言語:

- Go
- Rust
- Svelte
- TypeScript

サーバーの設定には `lsp` セクションを使用します。例は各言語のドキュメントに記載されています。

Gram ではヒントは即座に問い合わせされず、2種類のデバウンスが使われます。いずれも 0 に設定すると無効になります。設定に関連するヒントの更新はデバウンスされません。

`toggle_on_modifiers_press` に指定できるすべての値は次のとおりです:

```json
"inlay_hints": {
  "toggle_on_modifiers_press": {
    "control": true,
    "shift": true,
    "alt": true,
    "platform": true,
    "function": true
  }
}
```

指定のない値は `false` になります。すべての修飾キーが `false` の場合、またはすべての修飾キーが押されていない場合、ヒントは切り替わりません。

## Journal

- 説明: ジャーナルの設定。
- 設定: `journal`
- デフォルト:

```json
"journal": {
  "path": "~",
  "hour_format": "hour12"
}
```

### Path

- 説明: ジャーナルの項目を保存するフォルダーのパス。無効なパスを指定した場合、ジャーナルは `~`（ホームフォルダー）の使用に戻ります。
- 設定: `path`
- デフォルト: `~`

**選択肢**

`string` 値

### Hour Format

- 説明: ジャーナルで時刻を表示する形式。
- 設定: `hour_format`
- デフォルト: `hour12`

**選択肢**

1. 12時間形式:

```json
{
  "hour_format": "hour12",
}
```

2. 24時間形式:

```json
{
  "hour_format": "hour24",
}
```

## JSX Tag Auto Close

- 説明: JSX タグを自動的に閉じるかどうか。
- 設定: `jsx_tag_auto_close`
- デフォルト:

```json
{
  "jsx_tag_auto_close": {
    "enabled": true,
  },
}
```

**選択肢**

- `enabled`: JSX タグの自動閉じを有効にするかどうか

## Languages

- 説明: 言語ごとの設定。
- 設定: `languages`
- デフォルト: `null`

**選択肢**

言語の設定を上書きするには、`languages` の値にその言語名のエントリーを追加します。例:

```json
"languages": {
  "C": {
    "format_on_save": "off",
    "preferred_line_length": 64,
    "soft_wrap": "preferred_line_length"
  },
  "JSON": {
    "tab_size": 4
  }
}
```

次の設定は言語ごとに上書きできます:

- [`enable_language_server`](#enable-language-server)
- [`ensure_final_newline_on_save`](#ensure-final-newline-on-save)
- [`format_on_save`](#format-on-save)
- [`formatter`](#formatter)
- [`hard_tabs`](#hard-tabs)
- [`preferred_line_length`](#preferred-line-length)
- [`remove_trailing_whitespace_on_save`](#remove-trailing-whitespace-on-save)
- [`show_whitespaces`](#show-whitespaces)
- [`whitespace_map`](#whitespace-map)
- [`soft_wrap`](#soft-wrap)
- [`tab_size`](#tab-size)
- [`use_autoclose`](#use-autoclose)
- [`always_treat_brackets_as_autoclosed`](#always-treat-brackets-as-autoclosed)

これらの値は、同じ名前のルートレベルの設定と同じオプションを受け取ります。

## Line Indicator Format

- 説明: ステータスバーの行表示の形式。
- 設定: `line_indicator_format`
- デフォルト: `"short"`

**選択肢**

1. 短い形式:

```json
{
  "line_indicator_format": "short",
}
```

2. 長い形式:

```json
{
  "line_indicator_format": "long",
}
```

## Linked Edits

- 説明: 言語サーバーが対応している場合に、関連する範囲を連動して編集するかどうか。たとえば `<html>` の開始タグを編集すると、`</html>` の終了タグの内容も同様に編集されます。
- 設定: `linked_edits`
- デフォルト: `true`

**選択肢**

`boolean` 値

## LSP Document Colors

- 説明: 言語サーバーからのドキュメントの色情報を表示するかどうか。
- 設定: `lsp_document_colors`
- デフォルト: `true`

**選択肢**

`boolean` 値

## Max Tabs

- 説明: タブバーに表示するタブの最大数。
- 設定: `max_tabs`
- デフォルト: `null`

**選択肢**

正の `integer` 値、またはタブ数を無制限にする場合は `null`

## Middle Click Paste (Linux only)

- 説明: Linux で中クリックによる貼り付けを有効にします。
- 設定: `middle_click_paste`
- デフォルト: `true`

**選択肢**

`boolean` 値

## Multi Cursor Modifier

- 説明: マウスで複数のカーソルを追加するときに使う修飾キーを指定します。ホバーリンクのマウス操作は、マルチカーソル用の修飾キーと競合しないように調整されます。
- 設定: `multi_cursor_modifier`
- デフォルト: `alt`

**選択肢**

1. Linux と Windows では `Alt`、macOS では `Option` に対応します:

```json
{
  "multi_cursor_modifier": "alt",
}
```

2. Linux と Windows では `Control`、macOS では `Command` に対応します:

```json
{
  "multi_cursor_modifier": "cmd_or_ctrl", // alias: "cmd", "ctrl"
}
```

## Node

- 説明: Node.js 連携の設定。
- 設定: `node`
- デフォルト:

```json
{
  "node": {
    "ignore_system_version": false,
    "path": null,
    "npm_path": null,
  },
}
```

**選択肢**

- `ignore_system_version`: システムの Node.js バージョンを無視するかどうか
- `path`: Node.js バイナリへのカスタムパス
- `npm_path`: npm バイナリへのカスタムパス

## Network Proxy

- 説明: Gram のネットワークプロキシを設定します。
- 設定: `proxy`
- デフォルト: `null`

**選択肢**

プロキシの設定には、プロキシの URL を含める必要があります。

次の URI スキームをサポートしています:

- `http`
- `https`
- `socks4` - ローカル DNS を使う SOCKS4 プロキシ
- `socks4a` - リモート DNS を使う SOCKS4 プロキシ
- `socks5` - ローカル DNS を使う SOCKS5 プロキシ
- `socks5h` - リモート DNS を使う SOCKS5 プロキシ

スキームを指定しない場合は `http` が使われます。

既定ではプロキシは使用されません。または、Gram が `http_proxy`、`HTTP_PROXY`、`https_proxy`、`HTTPS_PROXY`、`all_proxy`、`ALL_PROXY`、`no_proxy`、`NO_PROXY` などの環境変数からプロキシ設定を取得しようとします。

たとえば `http` プロキシを設定するには、設定に次を追加します:

```json
{
  "proxy": "http://127.0.0.1:10809",
}
```

`socks5` プロキシを設定する場合は次のとおりです:

```json
{
  "proxy": "socks5h://localhost:10808",
}
```

特定のホストにプロキシを使わせたくない場合は、環境変数 `NO_PROXY` を設定します。プロキシを使わないホスト名、ホストサフィックス、IPv4/IPv6 アドレスまたはブロックをカンマ区切りで指定できます。たとえば環境に `NO_PROXY="google.com, 192.168.1.0/24"` が含まれている場合、`192.168.1.*` 内のすべてのホスト、`google.com`、`*.google.com` はプロキシを経由しません。詳しくは [reqwest NoProxy のドキュメント](https://docs.rs/reqwest/latest/reqwest/struct.NoProxy.html#method.from_string)を参照してください。

## On Last Window Closed

- 説明: 最後のウィンドウを閉じたときの動作。
- 設定: `on_last_window_closed`
- デフォルト: `"platform_default"`

**選択肢**

1. プラットフォームの既定の動作を使う:

```json
{
  "on_last_window_closed": "platform_default",
}
```

2. 常にアプリケーションを終了する:

```json
{
  "on_last_window_closed": "quit_app",
}
```

## Profiles

- 説明: 既存の設定の上に適用できる設定プロファイル。
- 設定: `profiles`
- デフォルト: `{}`

**選択肢**

設定プロファイルを定義するための設定オブジェクト。例:

```json
{
  "profiles": {
    "presentation": {
      "buffer_font_size": 20,
      "ui_font_size": 18,
      "theme": "One Light",
    },
  },
}
```

## Preview tabs

- 説明: プレビュータブを使うと、ファイルをプレビューモードで開けます。明示的にピン留めしない限り、別のファイルに切り替えると自動的に閉じます。ワークスペースを散らかさずにファイルを素早く確認したいときに便利です。プレビュータブはファイル名を斜体で表示します。\ プレビュータブを通常のタブに変換する方法はいくつかあります:

- ファイルをダブルクリックする
- タブのヘッダーをダブルクリックする
- {#action project_panel::OpenPermanent} アクションを使う
- ファイルを編集する
- ファイルを別のペインへドラッグする

- 設定: `preview_tabs`
- デフォルト:

```json
"preview_tabs": {
  "enabled": true,
  "enable_preview_from_project_panel": true,
  "enable_preview_from_file_finder": false,
  "enable_preview_from_multibuffer": true,
  "enable_preview_multibuffer_from_code_navigation": false,
  "enable_preview_file_from_code_navigation": true,
  "enable_keep_preview_on_code_navigation": false,
}
```

### Enable preview from project panel

- 説明: プロジェクトパネルから1回のクリックで開いたファイルをプレビューモードで開くかどうかを指定します。
- 設定: `enable_preview_from_project_panel`
- デフォルト: `true`

**選択肢**

`boolean` 値

### Enable preview from file finder

- 説明: ファイル検索から選択したファイルをプレビューモードで開くかどうかを指定します。
- 設定: `enable_preview_from_file_finder`
- デフォルト: `false`

**選択肢**

`boolean` 値

### Enable preview from multibuffer

- 説明: マルチバッファーから開いたファイルをプレビューモードで開くかどうかを指定します。
- 設定: `enable_preview_from_multibuffer`
- デフォルト: `true`

**選択肢**

`boolean` 値

### Enable preview multibuffer from code navigation

- 説明: コードナビゲーションでマルチバッファーを開くとき、プレビューモードでタブを開くかどうかを指定します。
- 設定: `enable_preview_multibuffer_from_code_navigation`
- デフォルト: `false`

**選択肢**

`boolean` 値

### Enable preview file from code navigation

- 説明: コードナビゲーションで単一ファイルを開くとき、プレビューモードでタブを開くかどうかを指定します。
- 設定: `enable_preview_file_from_code_navigation`
- デフォルト: `true`

**選択肢**

`boolean` 値

### Enable keep preview on code navigation

- 説明: コードナビゲーションでタブから離れたとき、そのタブをプレビューモードのまま維持するかどうかを指定します。`enable_preview_file_from_code_navigation` または `enable_preview_multibuffer_from_code_navigation` も true の場合、新しいタブが既存のタブを置き換えることがあります。
- 設定: `enable_keep_preview_on_code_navigation`
- デフォルト: `false`

**選択肢**

`boolean` 値

## File Finder

### File Icons

- 説明: ファイル検索にファイルのアイコンを表示するかどうか。
- 設定: `file_icons`
- デフォルト: `true`

### Modal Max Width

- 説明: ファイル検索モーダルの最大幅。`small`、`medium`、`large`、`xlarge`、`full` のいずれかを指定できます。
- 設定: `modal_max_width`
- デフォルト: `small`

### Skip Focus For Active In Search

- 説明: 検索結果の中でアクティブなファイルにフォーカスを移すかどうかを、ファイル検索側で判定します。
- 設定: `skip_focus_for_active_in_search`
- デフォルト: `true`

## Pane Split Direction Horizontal

- 説明: ペインを水平方向に分割する向き。
- 設定: `pane_split_direction_horizontal`
- デフォルト: `"up"`

**選択肢**

1. 上方向に分割する:

```json
{
  "pane_split_direction_horizontal": "up",
}
```

2. 下方向に分割する:

```json
{
  "pane_split_direction_horizontal": "down",
}
```

## Pane Split Direction Vertical

- 説明: ペインを垂直方向に分割する向き。
- 設定: `pane_split_direction_vertical`
- デフォルト: `"left"`

**選択肢**

1. 左方向に分割する:

```json
{
  "pane_split_direction_vertical": "left",
}
```

2. 右方向に分割する:

```json
{
  "pane_split_direction_vertical": "right",
}
```

## Preferred Line Length

- 説明: ソフト折り返しが有効なバッファーで、行を折り返す列位置。
- 設定: `preferred_line_length`
- デフォルト: `80`

**選択肢**

`integer` 値

## Private Files

- 説明: ファイルがプライベートかどうかを判定するために、ファイルパスと照合する Glob。
- 設定: `private_files`
- デフォルト: `["**/.env*", "**/*.pem", "**/*.key", "**/*.cert", "**/*.crt", "**/secrets.yml"]`

**選択肢**

`string` の Glob パターンのリスト

## Projects Online By Default

- 説明: オンラインプロジェクトのビューを既定で表示するかどうか。
- 設定: `projects_online_by_default`
- デフォルト: `true`

**選択肢**

`boolean` 値

## Read SSH Config

- 説明: SSH の設定ファイルを読み取るかどうか。
- 設定: `read_ssh_config`
- デフォルト: `true`

**選択肢**

`boolean` 値

## Redact Private Values

- 説明: プライベートファイル内で変数の値を表示から隠します。
- 設定: `redact_private_values`
- デフォルト: `false`

**選択肢**

`boolean` 値

## Relative Line Numbers

- 説明: ガターに相対行番号を表示するかどうか。
- 設定: `relative_line_numbers`
- デフォルト: `"disabled"`

**選択肢**

1. 折り返した行を1行として数え、ガターに相対行番号を表示する:

```json
{
  "relative_line_numbers": "enabled",
}
```

2. 折り返した行も数に含めて、ガターに相対行番号を表示する:

```json
{
  "relative_line_numbers": "wrapped",
}
```

2. 相対行番号を使わない:

```json
{
  "relative_line_numbers": "disabled",
}
```

## Remove Trailing Whitespace On Save

- 説明: バッファーを保存する前に、各行の末尾の空白を削除するかどうか。
- 設定: `remove_trailing_whitespace_on_save`
- デフォルト: `true`

**選択肢**

`boolean` 値

## Resize All Panels In Dock

- 説明: ドックのサイズを変更するときに、そのドック内のすべてのパネルのサイズも変更するかどうか。"left"、"right"、"bottom" の組み合わせを指定できます。
- 設定: `resize_all_panels_in_dock`
- デフォルト: `["left"]`

**選択肢**

次の任意の組み合わせを含む文字列のリスト:

- `"left"`: 左ドックのパネルをまとめてサイズ変更する
- `"right"`: 右ドックのパネルをまとめてサイズ変更する
- `"bottom"`: ボトムドックのパネルをまとめてサイズ変更する

## Restore on File Reopen

- 説明: ファイルを再度開いたときに、以前の状態を復元しようとするかどうか。状態はペインごとに保存されます。
- 設定: `restore_on_file_reopen`
- デフォルト: `true`

**選択肢**

`boolean` 値

## Restore on Startup

- 説明: 起動時のセッション復元を制御します。
- 設定: `restore_on_startup`
- デフォルト: `last_session`

**選択肢**

1. Gram の終了時に開いていたすべてのワークスペースを復元する:

```json
{
  "restore_on_startup": "last_session",
}
```

2. 最後に閉じたワークスペースを復元する:

```json
{
  "restore_on_startup": "last_workspace",
}
```

3. 常に空のエディターで起動する:

```json
{
  "restore_on_startup": "empty_tab",
}
```

4. 常にウェルカムランチパッドで起動する:

```json
{
  "restore_on_startup": "launchpad",
}
```

## Scroll Beyond Last Line

- 説明: エディターが最終行より先までスクロールできるかどうか。
- 設定: `scroll_beyond_last_line`
- デフォルト: `"one_page"`

**選択肢**

1. 最終行の先まで1ページ分スクロールできるようにする:

```json
{
  "scroll_beyond_last_line": "one_page",
}
```

2. `vertical_scroll_margin` と同じ行数だけ最終行の先までスクロールする:

```json
{
  "scroll_beyond_last_line": "vertical_scroll_margin",
}
```

3. 最終行の先へはスクロールしない:

```json
{
  "scroll_beyond_last_line": "off",
}
```

**選択肢**

`boolean` 値

## Scroll Sensitivity

- 説明: スクロール感度の倍率。この倍率はスクロール中の水平方向と垂直方向の移動量の両方に適用されます。
- 設定: `scroll_sensitivity`
- デフォルト: `1.0`

**選択肢**

正の `float` 値

### Fast Scroll Sensitivity

- 説明: 高速スクロール時のスクロール感度の倍率。この倍率はスクロール中の水平方向と垂直方向の移動量の両方に適用されます。高速スクロールは、スクロール中に alt キーまたは option キーを押しているときに発生します。
- 設定: `fast_scroll_sensitivity`
- デフォルト: `4.0`

**選択肢**

正の `float` 値

### Horizontal Scroll Margin

- 説明: マウスでスクロールするときに、左右に残しておく文字数。
- 設定: `horizontal_scroll_margin`
- デフォルト: `5`

**選択肢**

0 以上の `integer` 値

### Vertical Scroll Margin

- 説明: キーボードでスクロールするときに、カーソルの上下に残しておく行数。
- 設定: `vertical_scroll_margin`
- デフォルト: `3`

**選択肢**

0 以上の `integer` 値

## Search

- 説明: 新しいプロジェクト検索およびバッファー検索を開いたときに既定で有効にする検索オプション。
- 設定: `search`
- デフォルト:

```json
"search": {
  "button": true,
  "whole_word": false,
  "case_sensitive": false,
  "include_ignored": false,
  "regex": false,
  "center_on_match": false
},
```

### Button

- 説明: ステータスバーにプロジェクト検索のボタンを表示するかどうか。
- 設定: `button`
- デフォルト: `true`

### Whole Word

- 説明: 単語単位でのみ一致させるかどうか。
- 設定: `whole_word`
- デフォルト: `false`

### Case Sensitive

- 説明: 大文字小文字を区別して一致させるかどうか。この設定は検索と、"Select Next Occurrence"、"Select Previous Occurrence"、"Select All Occurrences" などのエディターアクションの両方に影響します。
- 設定: `case_sensitive`
- デフォルト: `false`

### Include Ignore

- 説明: 検索結果に Git で無視されているファイルを含めるかどうか。
- 設定: `include_ignored`
- デフォルト: `false`

### Regex

- 説明: 検索クエリを正規表現として解釈するかどうか。
- 設定: `regex`
- デフォルト: `false`

### Center On Match

- 説明: 移動中に、各検索の一致箇所へカーソルを中央揃えするかどうか。
- 設定: `center_on_match`
- デフォルト: `false`

## Search Wrap

- 説明: `search_wrap` が無効な場合、検索結果はファイル末尾から先頭へ折り返されません。
- 設定: `search_wrap`
- デフォルト: `true`

## Center on Match

- 説明: `center_on_match` が有効な場合、検索時にカーソルを現在の一致箇所の中央に配置します。
- 設定: `center_on_match`
- デフォルト: `false`

## Seed Search Query From Cursor

- 説明: カーソル下のテキストに基づいて、新しい検索のクエリをいつ設定するか。
- 設定: `seed_search_query_from_cursor`
- デフォルト: `always`

**選択肢**

1. `always` カーソル下の単語で常に検索クエリを設定する
2. `selection` テキストを選択している場合にのみ検索クエリを設定する
3. `never` 検索クエリを設定しない

## Use Smartcase Search

- 説明: 有効にすると、検索クエリに応じて検索の大文字小文字の区別を自動的に調整します。検索クエリに大文字が含まれる場合は大文字小文字を区別し、小文字のみの場合は区別しません。\ これはファイル内検索とプロジェクト全体の検索の両方に適用されます。
- 設定: `use_smartcase_search`
- デフォルト: `false`

**選択肢**

`boolean` 値

例:

- "function" を検索すると "function"、"Function"、"FUNCTION" などに一致します
- "Function" を検索すると "Function" にのみ一致し、"function" や "FUNCTION" には一致しません

## Completions

- 説明: この言語で補完をどのように処理するかを制御します。
- 設定: `completions`
- デフォルト:

```json
{
  "completions": {
    "words": "fallback",
    "words_min_length": 3,
    "lsp": true,
    "lsp_fetch_timeout_ms": 0,
    "lsp_insert_mode": "replace_suffix",
  },
}
```

### Words

- 説明: 単語補完の処理方法を制御します。大きなドキュメントでは、すべての単語が補完用に取得されるとは限りません。
- 設定: `words`
- デフォルト: `fallback`

**選択肢**

1. `enabled` - LSP の補完とあわせて、常にドキュメント内の単語も補完用に取得する
2. `fallback` - LSP の応答がエラーまたはタイムアウトした場合にのみ、ドキュメント内の単語を使って補完を表示する
3. `disabled` - ドキュメント内の単語を補完用に取得・補完しない（単語ベースの補完は別のアクションから問い合わせできます）

### Min Words Query Length

- 説明: 単語ベースの補完を自動的に起動するために必要な最小文字数。この値に満たない場合でも、対応するエディターコマンドで手動で単語ベースの補完を起動できます。
- 設定: `words_min_length`
- デフォルト: `3`

**選択肢**

正の整数値

### LSP

- 説明: LSP の補完を取得するかどうか。
- 設定: `lsp`
- デフォルト: `true`

**選択肢**

`boolean` 値

### LSP Fetch Timeout (ms)

- 説明: LSP の補完を取得するときに、特定のサーバーの応答を待つ時間を指定します。0 に設定すると無制限に待機します。
- 設定: `lsp_fetch_timeout_ms`
- デフォルト: `0`

**選択肢**

ミリ秒を表す `integer` 値

### LSP Insert Mode

- 説明: LSP の補完を確定したときに置き換える範囲を制御します。
- 設定: `lsp_insert_mode`
- デフォルト: `replace_suffix`

**選択肢**

1. `insert` - LSP 仕様の `insert` 範囲を使い、カーソルより前のテキストを置き換える
2. `replace` - LSP 仕様の `replace` 範囲を使い、カーソルの前後のテキストを置き換える
3. `replace_subsequence` - 置き換え対象のテキストが補完テキストの部分列であれば `"replace"` と同じ動作、そうでなければ `"insert"` と同じ動作
4. `replace_suffix` - カーソルより後のテキストが補完テキストの末尾と一致すれば `"replace"` と同じ動作、そうでなければ `"insert"` と同じ動作

## Show Completions On Input

- 説明: 入力中に補完を表示するかどうか。
- 設定: `show_completions_on_input`
- デフォルト: `true`

**選択肢**

`boolean` 値

## Show Completion Documentation

- 説明: 補完メニューの項目について、インラインおよび並べてドキュメントを表示するかどうか。
- 設定: `show_completion_documentation`
- デフォルト: `true`

**選択肢**

`boolean` 値

## Show Whitespaces

- 説明: エディター内で空白文字を描画するかどうか。
- 設定: `show_whitespaces`
- デフォルト: `selection`

**選択肢**

1. `all`
2. `selection`
3. `none`
4. `boundary`

## Whitespace Map

- 説明: `show_whitespaces` が有効なときに、空白文字の描画に使う文字を指定します。
- 設定: `whitespace_map`
- デフォルト:

```json
{
  "whitespace_map": {
    "space": "•",
    "tab": "→",
  },
}
```

## Soft Wrap

- 説明: テキストの行をエディターの幅または優先幅に合わせて自動的に折り返すかどうか。
- 設定: `soft_wrap`
- デフォルト: `none`

**選択肢**

1. `none` 折り返しを完全に行わない
2. `prefer_line`（非推奨。`none` と同じ）
3. `editor_width` エディターの幅を超える行を折り返す
4. `preferred_line_length` `preferred_line_length` の設定値を超える行を折り返す
5. `bounded` `editor_width` と `preferred_line_length` の小さいほうで行を折り返す

## Show Wrap Guides

- 説明: エディターに折り返しガイド（縦のルーラー）を表示するかどうか。true に設定すると、'soft_wrap' が 'preferred_line_length' の場合は 'preferred_line_length' の値の位置にガイドが表示され、'wrap_guides' の設定で指定した追加のガイドも表示されます。
- 設定: `show_wrap_guides`
- デフォルト: `true`

**選択肢**

`boolean` 値

## Use On Type Format

- 説明: "trigger" シンボルの入力のたびに（LSP サーバーの機能で定義されたもの）、追加の LSP クエリでコードを整形（修正）するかどうか。
- 設定: `use_on_type_format`
- デフォルト: `true`

**選択肢**

`boolean` 値

## Use Auto Surround

- 説明: 開き丸括弧、角括弧、波括弧、単一引用符、二重引用符を入力したときに、選択中のテキストを自動的に囲むかどうか。たとえばテキストを選択して '(' と入力すると、Gram はそのテキストを () で囲みます。
- 設定: `use_auto_surround`
- デフォルト: `true`

**選択肢**

`boolean` 値

## Use System Path Prompts

- 説明: 開くときと名前を付けて保存するときに、システム提供のダイアログを使うかどうか。false に設定すると、Gram は組み込みのキーボード中心のピッカーを使用します。
- 設定: `use_system_path_prompts`
- デフォルト: `true`

**選択肢**

`boolean` 値

## Use System Prompts

- 説明: 確認プロンプトなどのプロンプトに、システム提供のダイアログを使うかどうか。false に設定すると、Gram は組み込みのプロンプトを使用します。Linux ではこのオプションは無視され、Gram は常に組み込みのプロンプトを使用します。
- 設定: `use_system_prompts`
- デフォルト: `true`

**選択肢**

`boolean` 値

## Wrap Guides (Vertical Rulers)

- 説明: 縦のルーラーを折り返しガイドとして表示する位置。`show_wrap_guides` を `false` に設定すると無効になります。
- 設定: `wrap_guides`
- デフォルト: []

**選択肢**

`integer` の列番号のリスト

## Tab Size

- 説明: タブ文字1つに使うスペースの数。
- 設定: `tab_size`
- デフォルト: `4`

**選択肢**

`integer` 値

## Tasks

- 説明: Gram 内で実行できるタスクの設定。
- 設定: `tasks`
- デフォルト:

```json
{
  "tasks": {
    "variables": {},
    "enabled": true,
    "prefer_lsp": false,
  },
}
```

**選択肢**

- `variables`: タスク設定用のカスタム変数
- `enabled`: タスクを有効にするかどうか
- `prefer_lsp`: Gram の言語拡張機能のタスクより LSP 提供のタスクを優先するかどうか

### Diagnostics

- 説明: クラッシュレポートなど、デバッグ関連のデータを送信するための設定。
- 設定: `diagnostics`
- デフォルト: `true`

**選択肢**

`boolean` 値

## Terminal

- 説明: ターミナルの設定。
- 設定: `terminal`
- デフォルト:

```json
{
  "terminal": {
    "alternate_scroll": "off",
    "blinking": "terminal_controlled",
    "copy_on_select": false,
    "keep_selection_on_copy": true,
    "dock": "bottom",
    "default_width": 640,
    "default_height": 320,
    "detect_venv": {
      "on": {
        "directories": [".env", "env", ".venv", "venv"],
        "activate_script": "default",
      },
    },
    "env": {},
    "font_family": null,
    "font_features": null,
    "font_size": null,
    "line_height": "comfortable",
    "minimum_contrast": 45,
    "option_as_meta": false,
    "button": true,
    "shell": "system",
    "scroll_multiplier": 3.0,
    "toolbar": {
      "breadcrumbs": false,
    },
    "working_directory": "current_project_directory",
    "scrollbar": {
      "show": null,
    },
  },
}
```

### Terminal: Dock

- 説明: ドックの位置を制御します。
- 設定: `dock`
- デフォルト: `bottom`

**選択肢**

`"bottom"`、`"left"`、`"right"` のいずれか

### Terminal: Alternate Scroll

- 説明: Alternate Scroll モード（DECSET コード: `?1007`）を既定で有効にするかどうかを設定します。Alternate Scroll モードでは、代替画面（vim や less などのアプリケーションの実行中など）でマウスのスクロールイベントを上下キーの押下に変換します。ターミナル側では ANSI エスケープシーケンスでこのモードを設定・解除できます。
- 設定: `alternate_scroll`
- デフォルト: `off`

**選択肢**

1. Alternate Scroll モードを既定でオフにする

```json
{
  "terminal": {
    "alternate_scroll": "off",
  },
}
```

2. Alternate Scroll モードを既定でオンにする

```json
{
  "terminal": {
    "alternate_scroll": "on",
  },
}
```

### Terminal: Blinking

- 説明: ターミナルにおけるカーソルの点滅動作を設定します。
- 設定: `blinking`
- デフォルト: `terminal_controlled`

**選択肢**

1. カーソルを点滅させない（ターミナルのモードを無視する）

```json
{
  "terminal": {
    "blinking": "off",
  },
}
```

2. カーソルの点滅を既定でオフにし、ターミナル側で点滅をオンにできるようにする

```json
{
  "terminal": {
    "blinking": "terminal_controlled",
  },
}
```

3. 常にカーソルを点滅させる（ターミナルのモードを無視する）

```json
{
  "terminal": {
    "blinking": "on",
  },
}
```

### Terminal: Copy On Select

- 説明: ターミナル内でテキストを選択したときに、自動的にシステムのクリップボードへコピーするかどうか。
- 設定: `copy_on_select`
- デフォルト: `false`

**選択肢**

`boolean` 値

**例**

```json
{
  "terminal": {
    "copy_on_select": true,
  },
}
```

### Terminal: Cursor Shape

- 説明: ターミナルにおけるカーソルの形状を制御します。明示的に設定しない場合はブロック形状になります。
- 設定: `cursor_shape`
- デフォルト: `null` (defaults to block)

**選択肢**

1. 後続の文字を囲むブロック

```json
{
  "terminal": {
    "cursor_shape": "block",
  },
}
```

2. 縦棒

```json
{
  "terminal": {
    "cursor_shape": "bar",
  },
}
```

3. 後続の文字の下に沿って描かれる下線

```json
{
  "terminal": {
    "cursor_shape": "underline",
  },
}
```

4. 後続の文字の周りに描かれる枠

```json
{
  "terminal": {
    "cursor_shape": "hollow",
  },
}
```

### Terminal: Keep Selection On Copy

- 説明: テキストをコピーした後も、ターミナル内の選択範囲を維持するかどうか。
- 設定: `keep_selection_on_copy`
- デフォルト: `true`

**選択肢**

`boolean` 値

**例**

```json
{
  "terminal": {
    "keep_selection_on_copy": false,
  },
}
```

### Terminal: Env

- 説明: このオブジェクトに追加したキーと値のペアは、ターミナルの環境変数に追加されます。キーは一意である必要があり、1つの変数に複数の値を設定する場合は `:` で区切ります。
- 設定: `env`
- デフォルト: `{}`

**例**

```json
{
  "terminal": {
    "env": {
      "GRAM": "1",
      "KEY": "value1:value2",
    },
  },
}
```

### Terminal: Font Size

- 説明: ターミナルに使うフォントサイズ。設定しない場合は、エディターのフォントサイズに合わせて既定値が決まります。
- 設定: `font_size`
- デフォルト: `null`

**選択肢**

`integer` 値

```json
{
  "terminal": {
    "font_size": 15,
  },
}
```

### Terminal: Font Family

- 説明: ターミナルに使うフォント。設定しない場合は、エディターのフォントに合わせて既定値が決まります。
- 設定: `font_family`
- デフォルト: `null`

**選択肢**

ユーザーのシステムにインストールされている任意のフォントファミリー名

```json
{
  "terminal": {
    "font_family": "Berkeley Mono",
  },
}
```

### Terminal: Font Features

- 説明: ターミナルに使うフォントの機能。設定しない場合は、エディターのフォント機能に合わせて既定値が決まります。
- 設定: `font_features`
- デフォルト: `null`
- プラットフォーム: macOS and Windows.

**選択肢**

Buffer Font Features を参照してください

```json
{
  "terminal": {
    "font_features": {
      "calt": false,
      // See Buffer Font Features for more features
    },
  },
}
```

### Terminal: Line Height

- 説明: ターミナルの行の高さを設定します。
- 設定: `line_height`
- デフォルト: `standard`

**選択肢**

1. 読みやすさを重視した `comfortable` な行の高さ（1.618）を使う。

```json
{
  "terminal": {
    "line_height": "comfortable",
  },
}
```

2. `standard` な行の高さ（1.3）を使う。このオプションは TUI、とくに罫線文字を使う TUI に適しています（既定）。

```json
{
  "terminal": {
    "line_height": "standard",
  },
}
```

3. 任意の行の高さを使う。

```json
{
  "terminal": {
    "line_height": {
      "custom": 2,
    },
  },
}
```

### Terminal: Minimum Contrast

- 説明: ターミナルにおける前景色と背景色の最小コントラストを制御します。色の調整には APCA（Accessible Perceptual Contrast Algorithm）を使用します。0 に設定するとこの機能は無効になります。
- 設定: `minimum_contrast`
- デフォルト: `45`

**選択肢**

0 から 106 までの `integer` 値。よく推奨される値:

- `0`: コントラスト調整なし
- `45`: 大きな流暢なテキストの最小値（既定）
- `60`: その他の本文テキストの最小値
- `75`: 本文テキストの最小値
- `90`: 本文テキストの推奨値

```json
{
  "terminal": {
    "minimum_contrast": 45,
  },
}
```

### Terminal: Option As Meta

- 説明: option キーを Emacs の 'meta' キーのように再解釈します。
- 設定: `option_as_meta`
- デフォルト: `false`

**選択肢**

`boolean` 値

```json
{
  "terminal": {
    "option_as_meta": true,
  },
}
```

### Terminal: Shell

- 説明: ターミナルを起動するときに使うシェル。
- 設定: `shell`
- デフォルト: `system`

**選択肢**

1. システムの既定のターミナル設定（通常は `/etc/passwd` ファイル）を使う。

```json
{
  "terminal": {
    "shell": "system",
  },
}
```

2. 起動するプログラム:

```json
{
  "terminal": {
    "shell": {
      "program": "sh",
    },
  },
}
```

3. 引数付きで起動するプログラム:

```json
{
  "terminal": {
    "shell": {
      "with_arguments": {
        "program": "/bin/bash",
        "args": ["--login"],
      },
    },
  },
}
```

## Terminal: Detect Virtual Environments {#terminal-detect_venv}

- 説明: [Python 仮想環境](https://docs.python.org/3/library/venv.html)がターミナルの作業フォルダー（`working_directory` で解決）に見つかった場合に有効化し、自動的に仮想環境をアクティブ化します。
- 設定: `detect_venv`
- デフォルト:

```json
{
  "terminal": {
    "detect_venv": {
      "on": {
        // Default directories to search for virtual environments, relative
        // to the current working directory. We recommend overriding this
        // in your project's settings, rather than globally.
        "directories": [".env", "env", ".venv", "venv"],
        // Can also be `csh`, `fish`, and `nushell`
        "activate_script": "default",
      },
    },
  },
}
```

無効にするには次のようにします:

```json
{
  "terminal": {
    "detect_venv": "off",
  },
}
```

### Terminal: Scroll Multiplier

- 説明: マウスホイールやトラックパッドを使ったときの、ターミナルでのスクロール速度の倍率。
- 設定: `scroll_multiplier`
- デフォルト: `1.0`

**選択肢**

正の浮動小数点値。0 以下の値は最小値 0.01 に丸められます。

**例**

```json
{
  "terminal": {
    "scroll_multiplier": 5.0
  }
}
```

## Terminal: Toolbar

- 説明: ターミナルのツールバーに各種の要素を表示するかどうか。
- 設定: `toolbar`
- デフォルト:

```json
{
  "terminal": {
    "toolbar": {
      "breadcrumbs": false,
    },
  },
}
```

**選択肢**

現時点では `breadcrumbs` オプションのみが利用できます。これは `PROMPT_COMMAND` で変更できるターミナルのタイトルの表示を制御します。

ターミナルのタイトルが空の場合、ブレッドクラムは表示されません。

ターミナルで動作するシェルがタイトルを出力するよう設定されている必要があります。

タイトルを設定するコマンドの例: `echo -e "\e]2;New Title\007";`

### Terminal: Button

- 説明: ステータスバーにターミナルボタンを表示するかどうかを制御します。
- 設定: `button`
- デフォルト: `true`

**選択肢**

`boolean` 値

```json
{
  "terminal": {
    "button": false,
  },
}
```

### Terminal: Working Directory

- 説明: ターミナルを起動するときに使う作業フォルダー。
- 設定: `working_directory`
- デフォルト: `"current_project_directory"`

**選択肢**

1. 現在のファイルのプロジェクトフォルダーを使う。失敗した場合は「最初のプロジェクトのフォルダー」の方法に戻ります。

```json
{
  "terminal": {
    "working_directory": "current_project_directory",
  },
}
```

2. このワークスペースの最初のプロジェクトのフォルダーを使う。失敗した場合はこのプラットフォームのホームフォルダーを使います。

```json
{
  "terminal": {
    "working_directory": "first_project_directory",
  },
}
```

3. 見つかれば常にこのプラットフォームのホームフォルダーを使う。

```json
{
  "terminal": {
    "working_directory": "always_home",
  },
}
```

4. 常に特定のフォルダーを使う。この値はシェルで展開されます。このパスが有効なフォルダーでない場合、ターミナルはこのプラットフォームのホームフォルダーを使います。

```json
{
  "terminal": {
    "working_directory": {
      "always": {
        "directory": "~/gram/projects/",
      },
    },
  },
}
```

### Terminal: Path Hyperlink Regexes

- 説明: パスへのハイパーリンクを識別するための正規表現。正規表現は、単一の正規表現文字列と、文字列の配列（1つの複数行正規表現にまとめられます）の2つの形式で指定できます。
- 設定: `path_hyperlink_regexes`
- デフォルト:

```json
{
  "terminal": {
    "path_hyperlink_regexes": [
      // Python-style diagnostics
      "File \"(?<path>[^\"]+)\", line (?<line>[0-9]+)",
      // Common path syntax with optional line, column, description, trailing punctuation, or
      // surrounding symbols or quotes
      [
        "(?x)",
        "# optionally starts with 0-2 opening prefix symbols",
        "[({\\[<]{0,2}",
        "# which may be followed by an opening quote",
        "(?<quote>[\"'`])?",
        "# `path` is the shortest sequence of any non-space character",
        "(?<link>(?<path>[^ ]+?",
        "    # which may end with a line and optionally a column,",
        "    (?<line_column>:+[0-9]+(:[0-9]+)?|:?\\([0-9]+([,:][0-9]+)?\\))?",
        "))",
        "# which must be followed by a matching quote",
        "(?(<quote>)\\k<quote>)",
        "# and optionally a single closing symbol",
        "[)}\\]>]?",
        "# if line/column matched, may be followed by a description",
        "(?(<line_column>):[^ 0-9][^ ]*)?",
        "# which may be followed by trailing punctuation",
        "[.,:)}\\]>]*",
        "# and always includes trailing whitespace or end of line",
        "([ ]+|$)",
      ],
    ],
  },
}
```

### Terminal: Path Hyperlink Timeout (ms)

- 説明: パスへのハイパーリンクを探す最大時間。0 に設定するとパスのハイパーリンクは無効になります。
- 設定: `path_hyperlink_timeout_ms`
- デフォルト: `1`

## REPL

- 説明: REPL の設定。
- 設定: `repl`
- デフォルト:

```json
"repl": {
  // Maximum number of columns to keep in REPL's scrollback buffer.
  // Clamped with [20, 512] range.
  "max_columns": 128,
  // Maximum number of lines to keep in REPL's scrollback buffer.
  // Clamped with [4, 256] range.
  "max_lines": 32
},
```

## Theme

- 説明: テーマの設定は2つの形式で指定できます。テーマ名として指定するか、Gram UI 用の `mode`、`dark`、`light` のテーマを含むオブジェクトとして指定します。
- 設定: `theme`
- デフォルト: `One Dark`

### Theme Object

- 説明: `mode`、`dark`、`light` のテーマを含むオブジェクトでテーマを指定します。
- 設定: `theme`
- デフォルト:

```json
"theme": {
  "mode": "system",
  "dark": "One Dark",
  "light": "One Light"
},
```

### Mode

- 説明: テーマのモードを指定します。
- 設定: `mode`
- デフォルト: `system`

**選択肢**

1. テーマをダークモードにする

```json
{
  "mode": "dark",
}
```

2. テーマをライトモードにする

```json
{
  "mode": "light",
}
```

3. テーマをシステムモードにする

```json
{
  "mode": "system",
}
```

### Dark

- 説明: UI に使うダークテーマの名前。
- 設定: `dark`
- デフォルト: `One Dark`

**選択肢**

コマンドパレットで {#action theme_selector::Toggle} アクションを実行すると、現在有効なテーマ名の一覧を確認できます。

### Light

- 説明: UI に使うライトテーマの名前。
- 設定: `light`
- デフォルト: `One Light`

**選択肢**

コマンドパレットで {#action theme_selector::Toggle} アクションを実行すると、現在有効なテーマ名の一覧を確認できます。

## Title Bar

- 説明: タイトルバーに各種の要素を表示するかどうか。
- 設定: `title_bar`
- デフォルト:

```json
"title_bar": {
  "show": true,
  "show_branch_icon": false,
  "show_branch_name": true,
  "show_project_items": true,
  "show_onboarding_banner": true,
  "show_user_picture": true,
  "show_menus": false
}
```

**選択肢**

- `show_branch_icon`: タイトルバーのブランチ切り替えの横にブランチアイコンを表示するかどうか
- `show_branch_name`: タイトルバーにブランチ名のボタンを表示するかどうか
- `show_project_items`: タイトルバーにプロジェクトのホストと名前を表示するかどうか
- `show_onboarding_banner`: タイトルバーにオンボーディングのバナーを表示するかどうか
- `show_user_picture`: タイトルバーにユーザーの画像を表示するかどうか
- `show_menus`: タイトルバーにメニューを表示するかどうか

## Vim

- 説明: Vim モードを有効にするかどうか。
- 設定: `vim_mode`
- デフォルト: `false`

## When Closing With No Tabs

- 説明: タブのないウィンドウで 'close active item' を使用したときに、そのウィンドウを閉じるかどうか。
- 設定: `when_closing_with_no_tabs`
- デフォルト: `"platform_default"`

**選択肢**

1. プラットフォームの既定の動作を使う:

```json
{
  "when_closing_with_no_tabs": "platform_default",
}
```

2. 常にウィンドウを閉じる:

```json
{
  "when_closing_with_no_tabs": "close_window",
}
```

3. ウィンドウを閉じない:

```json
{
  "when_closing_with_no_tabs": "keep_window_open",
}
```

## Project Panel

- 説明: プロジェクトパネルをカスタマイズします。
- 設定: `project_panel`
- デフォルト:

```json
{
  "project_panel": {
    "button": true,
    "default_width": 240,
    "dock": "left",
    "entry_spacing": "comfortable",
    "file_icons": true,
    "folder_icons": true,
    "git_status": true,
    "indent_size": 20,
    "auto_reveal_entries": true,
    "auto_fold_dirs": true,
    "drag_and_drop": true,
    "scrollbar": {
      "show": null,
    },
    "sticky_scroll": true,
    "show_diagnostics": "all",
    "indent_guides": {
      "show": "always",
    },
    "sort_mode": "directories_first",
    "hide_root": false,
    "hide_hidden": false,
    "starts_open": true,
    "auto_open": {
      "on_create": true,
      "on_paste": true,
      "on_drop": true,
    },
  },
}
```

### Dock

- 説明: ドックの位置を制御します。
- 設定: `dock`
- デフォルト: `left`

**選択肢**

1. ドックの既定位置を左にする

```json
{
  "dock": "left",
}
```

2. ドックの既定位置を右にする

```json
{
  "dock": "right",
}
```

### Entry Spacing

- 説明: ワークツリーの項目間の間隔。
- 設定: `entry_spacing`
- デフォルト: `comfortable`

**選択肢**

1. ゆとりのある項目間隔

```json
{
  "entry_spacing": "comfortable",
}
```

2. 標準の項目間隔

```json
{
  "entry_spacing": "standard",
}
```

### Git Status

- 説明: 新規作成されたファイルと更新されたファイルを表示します。
- 設定: `git_status`
- デフォルト: `true`

**選択肢**

1. Git ステータスを既定で有効にする

```json
{
  "git_status": true,
}
```

2. Git ステータスを既定で無効にする

```json
{
  "git_status": false,
}
```

### Default Width

- 説明: プロジェクトパネルが使用する既定の幅をカスタマイズします。
- 設定: `default_width`
- デフォルト: `240`

**選択肢**

`float` 値

### Auto Reveal Entries

- 説明: 対応するプロジェクト項目がアクティブになったときに、プロジェクトパネル内で自動的に表示するかどうか。Git で無視されている項目は自動表示されません。
- 設定: `auto_reveal_entries`
- デフォルト: `true`

**選択肢**

1. 項目の自動表示を有効にする

```json
{
  "auto_reveal_entries": true,
}
```

2. 項目の自動表示を無効にする

```json
{
  "auto_reveal_entries": false,
}
```

### Auto Fold Dirs

- 説明: フォルダー内にフォルダーが1つしかない場合に、そのフォルダーを自動的に折りたたむかどうか。
- 設定: `auto_fold_dirs`
- デフォルト: `true`

**選択肢**

1. フォルダーの自動折りたたみを有効にする

```json
{
  "auto_fold_dirs": true,
}
```

2. フォルダーの自動折りたたみを無効にする

```json
{
  "auto_fold_dirs": false,
}
```

### Indent Size

- 説明: ネストした項目のインデント量（ピクセル単位）。
- 設定: `indent_size`
- デフォルト: `20`

### Indent Guides: Show

- 説明: プロジェクトパネルにインデントガイドを表示するかどうか。
- 設定: `indent_guides`
- デフォルト:

```json
"indent_guides": {
  "show": "always"
}
```

**選択肢**

1. プロジェクトパネルにインデントガイドを表示する

```json
{
  "indent_guides": {
    "show": "always",
  },
}
```

2. プロジェクトパネルのインデントガイドを非表示にする

```json
{
  "indent_guides": {
    "show": "never",
  },
}
```

### Scrollbar: Show

- 説明: プロジェクトパネルにスクロールバーを表示するかどうか。指定できる値は null、"auto"、"system"、"always"、"never" です。省略した場合はエディターの設定を継承します。詳しくはその説明を参照してください。
- 設定: `scrollbar`
- デフォルト:

```json
"scrollbar": {
  "show": null
}
```

**選択肢**

1. プロジェクトパネルにスクロールバーを表示する

```json
{
  "scrollbar": {
    "show": "always",
  },
}
```

2. プロジェクトパネルのスクロールバーを非表示にする

```json
{
  "scrollbar": {
    "show": "never",
  },
}
```

### Sort Mode

- 説明: プロジェクトパネル内の項目の並び順。
- 設定: `sort_mode`
- デフォルト: `directories_first`

**選択肢**

1. フォルダーを先に、次にファイルを表示する

```json
{
  "sort_mode": "directories_first",
}
```

2. フォルダー、次に mod.rs ファイル、次にその他すべてを表示する

```json
{
  "sort_mode": "smart_sort",
}

3. Mix directories and files together

```json
{
  "sort_mode": "mixed",
}
```

4. ファイルを先に、次にフォルダーを表示する

```json
{
  "sort_mode": "files_first",
}
```

### Auto Open

- 説明: プロジェクトパネルでのさまざまな作成操作の後、ファイルを自動的に開くかどうかを制御します。
- 設定: `auto_open`
- デフォルト:

```json
"auto_open": {
  "on_create": true,
  "on_paste": true,
  "on_drop": true
}
```

**選択肢**

- `on_create`: 新しく作成したファイルをエディターで自動的に開くかどうか。
- `on_paste`: 貼り付けまたは複製した後にファイルを自動的に開くかどうか。
- `on_drop`: 外部からドロップしたファイルを自動的に開くかどうか。
- `should_focus`: 自動的に開いたファイルにフォーカスするかどうか。

## Debugger

- 説明: デバッガーパネルとその設定の構成。
- 設定: `debugger`
- デフォルト:

```json
{
  "debugger": {
    "stepping_granularity": "line",
    "save_breakpoints": true,
    "dock": "bottom",
    "button": true,
  },
}
```

Gram のデバッグサポートについて詳しくは、[デバッガーのページ](./debugger.md)を参照してください。

## Git Panel

- 説明: Git パネルの動作をカスタマイズするための設定。
- 設定: `git_panel`
- デフォルト:

```json
{
  "git_panel": {
    "button": true,
    "dock": "left",
    "default_width": 360,
    "status_style": "icon",
    "fallback_branch_name": "main",
    "sort_by_path": false,
    "collapse_untracked_diff": false,
    "tree_view": false,
    "diff_stats": true,
    "scrollbar": {
      "show": null,
    },
  },
}
```

**選択肢**

- `button`: ステータスバーに Git パネルのボタンを表示するかどうか
- `dock`: Git パネルをドックする位置。`left` または `right` を指定できます
- `default_width`: Git パネルの既定の幅
- `status_style`: Git ステータスの表示方法。`label_color` または `icon` を指定できます
- `fallback_branch_name`: `init.defaultBranch` が設定されていないときに使うブランチ名
- `sort_by_path`: パネル内の項目をパス順で並べるか、ステータス順（既定）で並べるか
- `collapse_untracked_diff`: 差分パネルで未追跡ファイルを折りたたむかどうか
- `tree_view`: 項目をフラットなリストではなくツリーで表示するかどうか
- `diff_stats`: 項目ごとの追加行数と削除行数、およびリポジトリ全体の合計を表示するかどうか
- `scrollbar`: Git パネルでスクロールバーを表示する条件

## Git Hosting Providers

- 説明: セルフホストの GitHub、GitLab、Bitbucket インスタンスを登録し、コミットハッシュ、issue 参照、パーマリンクが正しいホストに解決されるようにします。
- 設定: `git_hosting_providers`
- デフォルト: `[]`

**選択肢**

各エントリーには次を指定できます:

- `provider`: `github`、`gitlab`、`bitbucket` のいずれか
- `name`: インスタンスの表示名
- `base_url`: ベース URL（例: `https://git.example.corp`）

これらはユーザー設定またはプロジェクト設定で定義できます。プロジェクト設定はユーザー設定の上にマージされます。

```json
{
  "git_hosting_providers": [
    {
      "provider": "github",
      "name": "BigCorp GitHub",
      "base_url": "https://git.example.corp",
    },
  ],
}
```

## Outline Panel

- 説明: アウトラインパネルをカスタマイズします。
- 設定: `outline_panel`
- デフォルト:

```json
"outline_panel": {
  "button": true,
  "default_width": 300,
  "dock": "left",
  "file_icons": true,
  "folder_icons": true,
  "git_status": true,
  "indent_size": 20,
  "auto_reveal_entries": true,
  "auto_fold_dirs": true,
  "indent_guides": {
    "show": "always"
  },
  "scrollbar": {
    "show": null
  }
}
```

## Calls

- 説明: 通話に参加したときの動作をカスタマイズします。
- 設定: `calls`
- デフォルト:

```json
"calls": {
  // Join calls with the microphone live by default
  "mute_on_join": false,
  // Share your project when you are the first to join a channel
  "share_on_join": false
},
```

## Colorize Brackets

- 説明: tree-sitter の括弧クエリを使ってエディター内の括弧を検出し、色分けするかどうか（「レインボーブラケット」とも呼ばれます）。
- 設定: `colorize_brackets`
- デフォルト: `false`

**選択肢**

`boolean` 値

インデントの深さごとに使う色はテーマ（テーマキー: `accents`）で定義されています。テーマの上書きでカスタマイズできます。

## Unnecessary Code Fade

- 説明: 未使用のコードをどの程度薄く表示するか。
- 設定: `unnecessary_code_fade`
- デフォルト: `0.3`

**選択肢**

`0.0` から `0.9` までの浮動小数点値。意味は次のとおりです:

- `0.0` はフェードなし（未使用のコードが使用中のコードと同じ見た目）
- `0.9` は最大のフェード（未使用のコードが非常に薄く表示されますが、判読は可能）

**例**

```json
{
  "unnecessary_code_fade": 0.5,
}
```

## UI Font Family

- 説明: UI のテキストに使うフォントの名前。
- 設定: `ui_font_family`
- デフォルト: `.GramSans`. This currently aliases to [IBM Plex](https://www.ibm.com/plex/).

**選択肢**

システムにインストールされている任意のフォントファミリー名、Gram が提供する既定値を使う場合は `".GramSans"`、システムの既定 UI フォント（macOS と Windows）を使う場合は `".SystemUIFont"` を指定します。

## UI Font Features

- 説明: UI のテキストで有効にする OpenType の機能。
- 設定: `ui_font_features`
- デフォルト:

```json
"ui_font_features": {
  "calt": false
}
```

- プラットフォーム: macOS and Windows.

**選択肢**

Gram は、特定の UI フォントで有効化・無効化できるすべての OpenType 機能をサポートし、フォント機能に値を設定することもできます。

たとえばフォントの合字を無効にするには、設定に次を追加します:

```json
{
  "ui_font_features": {
    "calt": false,
  },
}
```

`cv01` を `7` に設定するなど、他の OpenType 機能も設定できます:

```json
{
  "ui_font_features": {
    "cv01": 7,
  },
}
```

## UI Font Fallbacks

- 説明: UI のテキストに使うフォールバックフォント。
- 設定: `ui_font_fallbacks`
- デフォルト: `null`
- プラットフォーム: macOS and Windows.

**選択肢**

たとえば `Nerd Font` をフォールバックとして使うには、設定に次を追加します:

```json
{
  "ui_font_fallbacks": ["Nerd Font"],
}
```

## UI Font Size

- 説明: UI のテキストの既定のフォントサイズ。
- 設定: `ui_font_size`
- デフォルト: `16`

**選択肢**

`6` から `100` ピクセルまで（両端を含む）の `integer` 値

## UI Font Weight

- 説明: UI のテキストの既定のフォントの太さ。
- 設定: `ui_font_weight`
- デフォルト: `400`

**選択肢**

`100` から `900` までの `integer` 値

## Client Side Decoration Rounding

- 説明: クライアント側の装飾を使用するプラットフォームでのウィンドウの枠線の角丸半径を定義します。
- 設定: `client_side_decoration_rounding`
- デフォルト: `10.0`

## Client Side Decoration Shadow

- 説明: クライアント側の装飾を使用するプラットフォームでのウィンドウの影のサイズを定義します。
- 設定: `client_side_decoration_shadow`
- デフォルト: `10.0`

## Settings Profiles

- 説明: 設定プロファイルを任意の数だけ定義できます。`settings profile selector: toggle` から選択すると、既存のユーザー設定の上に一時的に適用されます。
- 設定: `profiles`
- デフォルト: `{}`

`settings.jsonc` ファイルに `profiles` オブジェクトを追加します。このオブジェクト内の各キーが設定プロファイルの名前で、各値は Gram の任意の設定を含められるオブジェクトです。

例:

```json
"profiles": {
  "Presenting (Dark)": {
    "buffer_font_size": 18.0,
    "theme": "One Dark",
    "ui_font_size": 18.0
  },
  "Presenting (Light)": {
    "buffer_font_size": 18.0,
    "theme": "One Light",
    "ui_font_size": 18.0
  },
  "Writing": {
    "buffer_font_size": 15.0,
    "theme": "Catppuccin Frappé - No Italics",
    "ui_font_size": 15.0,
    "tab_bar": { "show": false },
    "toolbar": { "breadcrumbs": false }
  }
}
```

設定プロファイルをプレビューして有効にするには、{#kb command_palette::Toggle} でコマンドパレットを開き、`settings profile selector: toggle` を検索します。

## An example configuration:

```json
// ~/.config/gram/settings.jsonc
{
  "theme": "cave-light",
  "tab_size": 2,
  "preferred_line_length": 80,
  "soft_wrap": "none",

  "buffer_font_size": 18,
  "buffer_font_family": ".GramMono",

  "autosave": "on_focus_change",
  "format_on_save": "off",
  "vim_mode": false,
  "projects_online_by_default": true,
  "terminal": {
    "font_family": "FiraCode Nerd Font Mono",
    "blinking": "off",
  },
  "languages": {
    "C": {
      "format_on_save": "on",
      "formatter": "language_server",
      "preferred_line_length": 64,
      "soft_wrap": "preferred_line_length",
    },
  },
}
```
