# キーバインド

## 定義済みキーマップ

特定のエディターのデフォルト操作に慣れている場合は、設定ウィンドウ（{#kb gram::OpenSettings}）または `settings.jsonc` ファイル（{#kb gram::OpenSettingsFile}）から直接 `base_keymap` を変更できます。

現在、以下のキーマップスタイルがサポートされています。

- Emacs（デフォルト）
- VS Code
- Atom
- JetBrains
- Sublime Text
- TextMate
- Minimal（Vim/Helix 向け）
- None（_すべての_ キーバインドを無効化）

この設定は、コマンドパレットから `gram: toggle base keymap selector` アクションを実行して変更することもできます。

また、モーダルバインドを追加する `vim_mode` や `helix_mode` を有効にすることも可能です。詳細については、[Vim モード](./vim.md) および [Helix モード](./helix.md) のドキュメントを参照してください。他のアプリケーションのキーバインドを排して純粋な Vim/Helix 体験を望む場合は、Minimal ベースキーマップを使用することをおすすめします。

## キーマップエディター

キーマップエディターは、{#kb gram::OpenKeymap} を押すか、コマンドパレットから {#action gram::OpenKeymap} アクションを実行して開くことができます。コマンドパレットの左下隅にある「Change Keybinding」または「Add Keybinding」ボタンから、アクションに対するキーバインドの追加や変更を簡単に行えます。

キーマップエディターでは、Gram に存在するすべての既存アクションと、デフォルトで設定されている関連キーバインドを確認できます。

また、各アクションにマウスホバーしたときに表示される鉛筆アイコンをクリックするか、アクションの行をダブルクリックするか、`Enter` キーを押すことで、その場で直接カスタマイズできます。

キーマップエディターで行った操作はすべて `keymap.jsonc` ファイルにも反映されます。

## ユーザーキーマップ

キーマップファイルは、各プラットフォームの以下の場所に保存されます。

- macOS/Linux: `~/.config/gram/keymap.jsonc`
- Windows: `~\AppData\Roaming\Gram/keymap.jsonc`

コマンドパレットから {#action gram::OpenKeymapFile} アクションを実行してキーマップを開くことができます。

このファイルには、`"bindings"` を含むオブジェクトの JSON 配列が記述されます。`"context"` が設定されていない場合、そのバインドは常にアクティブになります。設定されている場合、[コンテキストが一致](#contexts) した場合にのみバインドが有効になります。

各バインドセクション内では、[キーシーケンス](#keybinding-syntax) が [アクション](#actions) にマッピングされます。衝突が検出された場合は、[後述のルール](#precedence) に従って解決されます。

非 QWERTY 配列のラテン文字キーボードを使用している場合は、`use_key_equivalents` を `true` に設定するとよいでしょう。詳細については [非 QWERTY キーボード](#non-qwerty-keyboards) を参照してください。

設定例:

```json
[
  {
    "bindings": {
      "ctrl-right": "editor::SelectLargerSyntaxNode",
      "ctrl-left": "editor::SelectSmallerSyntaxNode",
    },
  },
  {
    "context": "ProjectPanel && not_editing",
    "bindings": {
      "o": "project_panel::Open",
    },
  },
]
```

カスタムキーマップに関する問題をデバッグしたい場合は、コマンドパレットから `dev: Open Key Context View` を使用できます。

### キーバインド構文

Gram では、単一のキー入力だけでなく、順番に入力されるキーのシーケンスとも照合できます。`"bindings"` マップ内の各キーは、スペースで区切られたキー入力のシーケンスです。

各キー入力は、修飾キーのシーケンスの後にキーが続く形式です。修飾キーは以下のとおりです。

- `ctrl-`: Control キー
- `cmd-`, `win-`, `super-`: プラットフォーム修飾キー（macOS では Command、Windows では Windows キー、Linux では Super キー）。
- `alt-`: Alt キー（macOS では Option）
- `shift-`: Shift キー
- `fn-`: Function キー
- `secondary-`: Gram が macOS 上で動作しているときは `cmd`、Windows および Linux 上では `ctrl` と同等

キーには、キーボードが生成する任意の単一 Unicode コードポイント（例: `a`, `0`, `£`, `ç`）、または任意の名前付きキー（`tab`, `f1`, `shift`, `cmd`）を指定できます。キリル文字などの非ラテン配列を使用している場合は、キリル文字自体、または `cmd` を押しながらそのキーを押したときに生成されるラテン文字のいずれかにバインドできます。

いくつかの例:

```json
"bindings": {
  "cmd-k cmd-s": "gram::OpenKeymap", // ⌘-k の後に ⌘-s を押す
  "space e": "editor::Complete", // Space の後に e を押す
  "ç": "editor::Complete", // ⌥-c に一致
  "shift shift": "file_finder::Toggle", // Shift を 2 回連続で押して離す
}
```

`shift-` 修飾キーは、大文字であることを示すために文字と組み合わせてのみ使用できます。たとえば、`shift-g` は `G` の入力に一致します。多くのキーボードでは `(` などの記号を入力するために Shift を使用しますが、そのキー入力は修飾キー付きとはみなされないため、`shift-(` には一致しません。

`alt-` 修飾キーは、多くの配列で別のキーを生成するために使用されます。たとえば、macOS の US キーボードでは、`alt-c` の組み合わせで `ç` が入力されます。キーマップファイルではどちらでも照合できますが、慣例として Gram ではこの組み合わせを `alt-c` と記述します。

修飾キー単体の入力と照合することも可能です。たとえば、JetBrains の「Search Everywhere」ショートカットを実装するために `shift shift` を使用できます。この場合、バインドはキーを押した瞬間ではなく、キーを離した（release）ときにトリガーされます。

### コンテキスト

バインドグループに `"context"` キーが含まれている場合、Gram 内で現在アクティブなコンテキストと照合されます。

Gram のコンテキストはツリー構造を成しており、ルートは `Workspace` です。Workspace には Pane（ペイン）や Panel（パネル）が含まれ、Pane には Editor（エディター）などが含まれます。特定の時点でどのコンテキストがアクティブであるかを確認する最も簡単な方法はキーコンテキストビューであり、コマンドパレットの `dev: open key context view` コマンドからアクセスできます。

表示例:

```
# エディター内では以下のようになります:
Workspace os=macos keyboard_layout=com.apple.keylayout.QWERTY
  Pane
    Editor mode=full extension=md vim_mode=insert

# プロジェクトパネル内:
Workspace os=macos
  Dock
    ProjectPanel not_editing
```

コンテキスト式には以下の構文を使用できます。

- `X && Y`, `X || Y`: 2 つの条件の AND / OR
- `!X`: 条件が false であることを確認
- `(X)`: グループ化
- `X > Y`: ツリー内の祖先が X に一致し、現在の層が Y に一致する場合に照合

例:

- `"context": "Editor"`: 任意のエディター（インライン入力欄を含む）に一致
- `"context": "Editor && mode == full"`: コード編集に使用されるメインエディターに一致
- `"context": "!Editor && !Terminal"`: Editor または Terminal にフォーカスがある場所以外のすべてに一致
- `"context": "os == macos > Editor"`: macOS 上の任意のエディターに一致

属性は、それが定義されているノード上でのみ利用可能である点に注意してください。つまり、たとえばデバッガーが停止しており、かつ Vim のノーマルモードである場合にのみキーバインドを有効にしたい場合は、`debugger_stopped > vim_mode == normal` と記述する必要があります。

> 注意: Gram v0.197.x より前は、`!` 演算子は一度に 1 つのノードのみを対象とし、`>` は「祖先」ではなく「親」を意味していました。そのため、`!Editor` が `Workspace > Pane > Editor` というコンテキストに一致してしまったり（紛らわしいことに Pane が `!Editor` に一致するため）、中間に `Pane` ノードが存在するために `os == macos > Editor` が `Workspace > Pane > Editor` に一致しなかったりしていました。

Vim モードをご利用の場合は、[Vim モードがコンテキストに与える影響](./vim.md#contexts) を参照してください。Helix モードは Vim モードを基盤として構築されており、同じコンテキストを使用します。

### アクション

Gram の機能のほぼすべてがアクションとして公開されています。

ほとんどのアクションは引数を必要としないため、文字列としてバインドできます: `"ctrl-a": "language_selector::Toggle"`。単一の引数を必要とするものもあり、配列としてバインドする必要があります: `"cmd-1": ["workspace::ActivatePane", 0]`。一部のアクションは複数の引数を必要とし、文字列とオブジェクトの配列としてバインドします: `"ctrl-a": ["pane::DeploySearch", { "replace_enabled": true }]`。

### 優先順位

複数のキーバインドが同じキーストロークを持ち、同時にアクティブである場合、優先順位は次の 2 つの方法で解決されます。

- コンテキストツリーのより下層のノードで一致するバインドが優先されます。つまり、`Editor` コンテキストを持つバインドは、`Workspace` コンテキストを持つバインドよりも優先されます。コンテキストのないバインドは、ツリーの最下層で一致します。
- ツリーの同じ階層で一致する複数のバインドがある場合は、後から定義されたバインドが優先されます。ユーザーキーバインドはシステムキーバインドの後に読み込まれるため、これによりユーザーバインドが組み込みキーバインドよりも優先されます。

もうひとつの衝突のパターンは、2 つのバインドのうち一方が他方のプレフィックス（接頭辞）になっている場合です。たとえば、`"ctrl-w":"editor::DeleteToNextWordEnd"` と `"ctrl-w left":"editor::DeleteToEndOfLine"` がある場合です。

このような状況が発生し、現在のコンテキストで両方のバインドがアクティブである場合、Gram は `ctrl-w` を入力した後に 1 秒間待機し、続けて `left` が入力されるかどうかを確認します。何も入力されないか、別のキーが入力された場合は `DeleteToNextWordEnd` がトリガーされます。`left` が入力された場合は `DeleteToEndOfLine` がトリガーされます。

### 非 QWERTY キーボード

Gram における非 QWERTY キーボードのサポートは現在も開発途上です。

キーボードが完全な ASCII 範囲を入力できる場合（DVORAK、COLEMAK など）、ショートカットは期待どおりに動作するはずです。

それ以外の場合は、以下をお読みください。

#### macOS

キリル文字、ヘブライ語、アルメニア語など、大半が非 ASCII であるキーボードでは、macOS は `cmd` を押している間にキーを ASCII 範囲に自動マッピングします。Gram はこれをさらに推し進め、修飾キーや `use_key_equivalents` の設定に関係なく、キーストロークを常に ASCII 配列または実際の配列のいずれに対しても照合できます。たとえばタイ語では、`ctrl-ๆ` を押すと `ctrl-q` または `ctrl-ๆ` に関連付けられたバインドに一致します。

拡張ラテン文字をサポートするキーボード（フランス語 AZERTY、ドイツ語 QWERTZ など）では、多くの場合 `option` なしでは ASCII 範囲全体を入力できません。これにより曖昧さが生じます（例: `option-2` で `@` が入力されるなど）。これらのキーボードでもすべての組み込みキーボードショートカットを入力できるように、キーバインドの再配置を行っています。たとえば、QWERTY で `@` に割り当てられているショートカットは、スペイン語配列では `"` に移動されます。このマッピングは macOS システムのデフォルトに基づいており、コマンドパレットから `dev: open key context view` を実行することで確認できます。

個人用キーマップでショートカットを定義する際、キーマップで `use_key_equivalents` を `true` に設定することで、キー等価マッピングを有効化できます。

```json
[
  {
    "use_key_equivalents": true,
    "bindings": {
      "ctrl->": "editor::Indent", // ドイツ語 QWERTZ キーボードがアクティブな場合は ctrl-: として解釈されます
    },
  },
]
```

### Linux

Linux では v0.196.0 以降、入力したキーが ASCII 文字を生成しない場合、キーボードショートカットには QWERTY 配列の同等キーを使用します。これにより、多くの配列で多くのショートカットが入力可能になります。

すべての配列ですべての組み込みショートカットが確実に入力できるようにするためのショートカットの移動処理はまだ行われていないため、入力できない ASCII 文字があり、キーボード配列上でそれらを入力するために必要なキーに異なる ASCII 文字が割り当てられている場合は、動作させるためにカスタムキーバインドを追加する必要がある場合があります。この問題はいずれ修正する予定であり、ご協力は大歓迎です。

## ヒントとテクニック

### バインドの無効化

特定のコンテキストで特定のバインドを何もしないようにしたい場合は、アクションとして `null` を使用できます。これは、誤ってキーバインドを押してしまうのを防ぐために無効化したい場合や、そのシーケンスによって入力されるはずの文字を入力したい場合、またはそのキーで始まるマルチキーバインドを無効化したい場合に役立ちます。

```json
[
  {
    "context": "Workspace",
    "bindings": {
      "cmd-r": null, // Workspace コンテキストがアクティブなとき、cmd-r は何もしません
    },
  },
]
```

`null` バインドは通常のアクションと同じ優先順位ルールに従うため、ツリーのより上位で一致するはずのすべてのバインドも無効化されます。ツリーの上位で一致するバインドを下位のバインドよりも優先させたい場合は、目的のコンテキストで目的のアクションに再バインドする必要があります。

これは、指定したアクションが条件付きで伝播する場合に、Gram がデフォルトのキーバインドにフォールバックするのを防ぐのに便利です。たとえば、`buffer_search::DeployReplace` は検索バーが表示されていない場合にのみトリガーされます。検索バーが表示されている場合、キー入力が伝播して、右ドックを開くなどそのキーバインドに設定されたデフォルトのアクションがトリガーされてしまいます。これを防ぐには次のように記述します。

```json
[
  {
    "context": "Workspace",
    "bindings": {
      "cmd-r": null, // 検索バーが表示されているとき、cmd-r は何もしません
    },
  },
  {
    "context": "Workspace",
    "bindings": {
      "cmd-r": "buffer_search::DeployReplace", // 検索バーが表示されていないとき、cmd-r は置換を展開します
    },
  },
]
```

### キーのリマッピング

よくある要望として、単一のキーストロークから一連のシーケンスへのマッピングがあります。これは `workspace::SendKeystrokes` アクションで実現できます。

```json
[
  {
    "bindings": {
      // 4 回下に移動
      "alt-down": ["workspace::SendKeystrokes", "down down down down"],
      // 選択範囲を拡大（editor::SelectLargerSyntaxNode）、
      // クリップボードにコピーし、選択範囲の拡大を取り消す
      "cmd-alt-c": [
        "workspace::SendKeystrokes",
        "ctrl-shift-right ctrl-shift-right ctrl-shift-right cmd-c ctrl-shift-left ctrl-shift-left ctrl-shift-left",
      ],
    },
  },
  {
    "context": "Editor && vim_mode == insert",
    "bindings": {
      "j k": ["workspace::SendKeystrokes", "escape"],
    },
  },
]
```

これにはいくつかの制限事項があります。主なものは以下のとおりです。

- すべてのキーバインドがディスパッチされるまで、非同期操作は実行されません。たとえば、バインドを使用してファイルを開くことはできますが、追加のキーストロークを送信して新しいビューに解釈させることはできません。
- 非同期処理のその他の例: コマンドパレットを開く、言語サーバーと通信する、バッファーの言語を変更する、ネットワークにアクセスする処理など。
- 一度にシミュレートできるキーは 100 個までに制限されています。

`SendKeystrokes` への引数は、スペース区切りのキーストロークリストです（上記と同じ構文を使用します）。キーストロークの解析方法の都合上、キー入力として認識されないセグメントは、現在フォーカスされている入力フィールドにそのまま verbatim で送信されます。

`SendKeystrokes` の引数にそれをトリガーするために使用されたバインド自体が含まれている場合、そのバインドの次に優先度の高い定義が使用されます。これにより、キーバインドのデフォルトの動作を拡張することができます。

### キーをターミナルへ転送

Linux や Windows では、キーの組み合わせが Gram に処理されるのではなく、組み込みターミナルに転送されるようにしたい場合があります。

たとえば、Linux の Gram では `ctrl-n` で新しいタブが作成されます。組み込みターミナルにフォーカスがあるときに `ctrl-n` をターミナルに送信したい場合は、キーマップに以下を追加します。

```json
{
  "context": "Terminal",
  "bindings": {
    "ctrl-n": ["terminal::SendKeystroke", "ctrl-n"],
  },
}
```

### タスクのキーバインド

`tasks.jsonc` で定義された Gram タスクを起動するためのキーをバインドすることもできます。詳細については [タスクのドキュメント](./tasks.md#custom-keybindings-for-tasks) を参照してください。
