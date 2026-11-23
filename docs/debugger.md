# デバッガー

Gram では、複数のプログラミング言語にまたがるデバッグ機能を提供するために [Debug Adapter Protocol (DAP)](https://microsoft.github.io/debug-adapter-protocol/) を採用しています。DAP は、デバッガー、エディター、および IDE 間の通信方法を定義する標準化されたプロトコルです。これにより、Gram は言語固有のデバッグロジックを個別に実装することなく、さまざまなデバッガーをサポートできます。Gram はプロトコルのクライアント側を実装し、各種 _デバッグアダプター_ がサーバー側を実装します。

このプロトコルにより、ブレークポイントの設定、コードのステップ実行、変数の検査などの機能が、異なるプログラミング言語やランタイム環境間で一貫した方法で実現されます。

## サポートされている言語

特定の言語で書かれたコードをデバッグするには、Gram がその言語用のデバッグアダプターを検出できる必要があります。一部のデバッグアダプターは追加の設定なしに Gram に組み込まれており、一部は [言語拡張機能](./extensions/debugger-extensions.md) によって提供されます。現在、以下の言語でデバッグアダプターが利用可能です。

- [C](./languages/c.md#debugging)（組み込み）
- [C++](./languages/cpp.md#debugging)（組み込み）
- [Go](./languages/go.md#debugging)（組み込み）
- [Java](./languages/java.md#debugging)（拡張機能により提供）
- [JavaScript](./languages/javascript.md#debugging)（組み込み）
- [PHP](./languages/php.md#debugging)（組み込み）
- [Python](./languages/python.md#debugging)（組み込み）
- [Ruby](./languages/ruby.md#debugging)（拡張機能により提供）
- [Rust](./languages/rust.md#debugging)（組み込み）
- [Swift](./languages/swift.md#debugging)（拡張機能により提供）
- [TypeScript](./languages/typescript.md#debugging)（組み込み）

> お使いの言語が記載されていない場合でも、[デバッグアダプターの追加](./extensions/debugger-extensions.md) を通じて貢献していただけます。

各言語およびアダプター固有の情報や設定例については上記リンクを参照してください。以下では、すべてのアダプターに共通する Gram の全般的なデバッグ機能について説明します。

## はじめに

多くの言語において、最も迅速にデバッグを開始する方法は {#action debugger::Start}（{#kb debugger::Start}）を実行することです。これにより _新規プロセスモーダル_ が開き、現在のプロジェクト用に事前構成されたデバッグタスクが文脈に応じて一覧表示されます。デバッグタスクは、テスト、エントリーポイント（`main` 関数など）、その他のソースから生成されます。サポート状況の詳細については、各言語のドキュメントを参照してください。

デバッグパネルの右上にある「プラス（+）」ボタンをクリックしても同じモーダルを開くことができます。

事前構成されたデバッグタスクが提供されない言語（C、C++、および一部の拡張機能でサポートされる言語など）では、プロジェクトのルートにある `.gram/debug.jsonc` ファイルにデバッグ構成を定義できます。このファイルは構成オブジェクトの配列として記述します。

```json [debug]
[
  {
    "adapter": "CodeLLDB",
    "label": "First configuration"
    // ...
  },
  {
    "adapter": "Debugpy",
    "label": "Second configuration"
    // ...
  }
]
```

一般的なユースケースをカバーする構成例については、各言語のドキュメントを確認してください。`.gram/debug.jsonc` に構成を追加すると、新規プロセスモーダルのリストに表示されるようになります。

また、`.gram/debug.jsonc` に構成が見つからない場合、Gram は `.vscode/launch.json` からもデバッグ構成を読み込み、新規プロセスモーダルに表示します。

#### グローバルデバッグ構成

複数のプロジェクトで同じ起動プロファイルを使用する場合は、ユーザー設定に一度保存しておくだけで共有できます。コマンドパレットから {#action gram::OpenDebugTasks} を呼び出して、グローバルな `debug.jsonc` ファイルを開きます。Gram はこれをユーザーの `settings.jsonc` と同じ場所に作成し、デバッガー UI と同期を保ちます。ファイルの配置場所は以下のとおりです。

- **macOS:** `~/Library/Application Support/Gram/debug.jsonc`
- **Linux/BSD:** `$XDG_CONFIG_HOME/gram/debug.jsonc`（見つからない場合は `~/.config/gram/debug.jsonc`）
- **Windows:** `%APPDATA%\Gram\debug.jsonc`

このファイルに、`.gram/debug.jsonc` と同様のオブジェクト配列を記述します。ここで定義されたシナリオはすべてのワークスペースにマージされるため、お気に入りの起動プリセットが「New Debug Session」ダイアログに自動的に表示されます。

### 起動（Launch）とアタッチ（Attach）

Gram デバッガーには、プログラムをデバッグするための 2 つの方法があります。プログラムの新しいインスタンスを _起動（launch）_ するか、既存のプロセスに _アタッチ（attach）_ するかです。どちらを選択するかは、目的によって異なります。

新しいインスタンスを起動する場合、Gram（および基盤となるデバッグアダプター）はプログラム全体のライフタイムを制御できるため、既存のプロセスにアタッチする場合と比べて、多くの場合より適切にデバッグ情報を収集できます。単体テストの実行やアプリケーションのデバッグビルドを実行する場合は、起動（launch）が適しています。

起動と比較すると既存プロセスへのアタッチは劣るように思えるかもしれませんが、決してそうではありません。たとえば本番環境以外では再現しないバグがある場合など、プログラムを再起動できない状況ではアタッチが不可欠です。

## 構成

Gram では、すべてのデバッグタスクにおいて `adapter` フィールドと `label` フィールドが必須です。さらに、Gram はデバッガー起動前に必要なセットアップ手順を実行するために `build` フィールドを使用し（[後述](#build-tasks)）、既存のプロセスに接続するために `tcp_connection` フィールドを受け入れることができます。

その他すべてのフィールドはデバッグアダプターによって提供され、[タスク変数](./tasks.md#variables) を含めることができます。ほとんどのアダプターは `request`、`program`、`cwd` をサポートしています。

```json [debug]
[
  {
    // デバッグ構成のラベル。デバッグパネルおよび新規プロセスモーダル内でデバッグセッションを識別するために使用されます
    "label": "Example Start debugger config",
    // Gram がプログラムのデバッグに使用するデバッグアダプター
    "adapter": "Example adapter name",
    // Request:
    //  - launch: 指定された場合、Gram はプログラムを起動するか、適切な構成でデバッグターミナルを表示します
    //  - attach: Gram は実行中のプログラムにアタッチしてデバッグします。process_id が指定されていない場合はプロセスピッカーを表示します（現在は Node のみサポート）
    "request": "launch",
    // デバッグ対象のプログラム。このフィールドは ~ や . 記号によるパス解決をサポートしています
    "program": "path_to_program",
    // cwd: デフォルトはプロジェクトの現在の作業ディレクトリ（$GRAM_WORKTREE_ROOT）です
    "cwd": "$GRAM_WORKTREE_ROOT"
  }
]
```

サポートされているフィールドの詳細については、各デバッグアダプターのドキュメントを確認してください。

### ビルドタスク

Gram では、デバッガーが起動する前に実行される Gram タスクを `build` フィールドに埋め込むことができます。これは、環境の準備や、デバッガー起動前に必要なセットアップ手順を実行するのに便利です。

```json [debug]
[
  {
    "label": "Build Binary",
    "adapter": "CodeLLDB",
    "program": "path_to_program",
    "request": "launch",
    "build": {
      "command": "make",
      "args": ["build", "-j8"]
    }
  }
]
```

ビルドタスクは、置換前のラベルによって既存のタスクを参照することもできます。

```json [debug]
[
  {
    "label": "Build Binary",
    "adapter": "CodeLLDB",
    "program": "path_to_program",
    "request": "launch",
    "build": "my build task" // または "my build task for $GRAM_FILE"
  }
]
```

### シナリオの自動生成

Gram タスクが与えられた場合、Gram は自動的にシナリオを作成できます。このシナリオ自動生成機能は、ガターからのシナリオ生成にも利用されています。現在、Rust、Go、Python、JavaScript、TypeScript でシナリオの自動生成がサポートされています。

## ブレークポイント

ブレークポイントを設定するには、エディターのガターにある行番号の横をクリックするだけです。ブレークポイントは必要に応じて細かく調整できます。特定のブレークポイントの追加オプションにアクセスするには、ガター内のブレークポイントアイコンを右クリックして目的のオプションを選択します。現在、以下の操作が可能です。

- ブレークポイントにログを追加: そのブレークポイントに到達するたびにログメッセージを出力します。
- 条件付きブレークポイントにする: 条件が満たされた場合にのみブレークポイントで停止します。条件の構文はアダプター固有です。
- ヒットカウントを追加: 一定回数ヒットした後にのみブレークポイントで停止します。
- ブレークポイントを無効化: ガターに表示を残したまま、ヒットしないようにします。

一部のデバッグアダプター（CodeLLDB や JavaScript など）は、ブレークポイントがヒット可能かどうかを _検証（verify）_ することもできます。ヒットできないブレークポイントは UI 上でより目立つように表示されます。

特定のプロジェクトで有効になっているすべてのブレークポイントは、デバッグセッション UI の「Breakpoints」項目にも一覧表示されます。UI の「Breakpoints」項目からは、例外ブレークポイントを管理することもできます。これにより、特定の種類の例外が発生するたびにデバッグアダプターが停止します。どの例外タイプがサポートされているかは、デバッグアダプターによって異なります。

## 設定

デバッガーの設定は、`settings.jsonc` の `debugger` キーの下にまとめられています。

- `dock`: UI 内でのデバッグパネルの位置を決定します。
- `stepping_granularity`: ステップ実行の粒度を決定します。
- `save_breakpoints`: Gram のセッション間でブレークポイントを保持・再利用するかどうか。
- `button`: ステータスバーにデバッグボタンを表示するかどうか。
- `timeout`: TCP デバッグアダプターに接続する際のタイムアウトエラーまでの時間（ミリ秒）。
- `log_dap_communications`: アクティブなデバッグアダプターと Gram 間のメッセージをログに記録するかどうか。
- `format_dap_log_messages`: デバッグアダプターロガーに追加する際に DAP メッセージを整形（フォーマット）するかどうか。

### Dock

- 説明: UI 内でのデバッグパネルの位置。
- デフォルト: `bottom`
- 設定: `debugger.dock`

**選択肢**

1. `left`: デバッグパネルが UI の左側にドックされます。
2. `right`: デバッグパネルが UI の右側にドックされます。
3. `bottom`: デバッグパネルが UI の下部にドックされます。

```json
"debugger": {
  "dock": "bottom"
},
```

### ステップ実行の粒度（Stepping granularity）

- 説明: デバッガーが使用するステップ実行の粒度。
- デフォルト: `line`
- 設定: `debugger.stepping_granularity`

**選択肢**

1. Statement: 現在の文（statement）の実行が完了するまでプログラムを実行します。文の意味はアダプターによって定義され、行と同等とみなされる場合もあります。たとえば `for(int i = 0; i < 10; i++)` は、`int i = 0`、`i < 10`、`i++` という 3 つの文を持つとみなされることがあります。

```json
{
  "debugger": {
    "stepping_granularity": "statement",
  },
}
```

2. Line: 現在のソースコード行が実行されるまでプログラムを実行します。

```json
{
  "debugger": {
    "stepping_granularity": "line",
  },
}
```

3. Instruction: 1 つの命令（x86 命令など）が実行されるまでプログラムを実行します。

```json
{
  "debugger": {
    "stepping_granularity": "instruction",
  },
}
```

### ブレークポイントの保存（Save Breakpoints）

- 説明: Gram のセッション間でブレークポイントを保存するかどうか。
- デフォルト: `true`
- 設定: `debugger.save_breakpoints`

**選択肢**

`boolean` 値

```json
{
  "debugger": {
    "save_breakpoints": true,
  },
}
```

### ボタン（Button）

- 説明: デバッガーツールバーにボタンを表示するかどうか。
- デフォルト: `true`
- 設定: `debugger.show_button`

**選択肢**

`boolean` 値

```json
{
  "debugger": {
    "show_button": true,
  },
}
```

### タイムアウト（Timeout）

- 説明: TCP デバッグアダプターに接続する際のタイムアウトエラーまでのミリ秒単位の時間。
- デフォルト: `2000`
- 設定: `debugger.timeout`

**選択肢**

`integer` 値

```json
{
  "debugger": {
    "timeout": 3000,
  },
}
```

### インライン値（Inline Values）

- 説明: デバッグセッション中にコード内の変数の値をインラインで表示するインレイヒントを有効にするかどうか。
- デフォルト: `true`
- 設定: `inlay_hints.show_value_hints`

**選択肢**

```json
{
  "inlay_hints": {
    "show_value_hints": false,
  },
}
```

インライン値ヒントは、エディターツールバーの「Editor Controls」メニューから切り替えることもできます。

### DAP 通信のログ記録（Log Dap Communications）

- 説明: アクティブなデバッグアダプターと Gram 間のメッセージをログに記録するかどうか（DAP 開発用）。
- デフォルト: `false`
- 設定: `debugger.log_dap_communications`

**選択肢**

`boolean` 値

```json
{
  "debugger": {
    "log_dap_communications": true,
  },
}
```

### DAP ログメッセージの整形（Format Dap Log Messages）

- 説明: デバッグアダプターロガーに追加する際に DAP メッセージを整形するかどうか（DAP 開発用）。
- デフォルト: `false`
- 設定: `debugger.format_dap_log_messages`

**選択肢**

`boolean` 値

```json
{
  "debugger": {
    "format_dap_log_messages": true,
  },
}
```

### デバッグアダプターのカスタマイズ

- 説明: Gram が特定のデバッグアダプターを起動する方法をオーバーライドするためのカスタムプログラムパスと引数。
- デフォルト: アダプター依存
- 設定: `dap.$ADAPTER.binary` および `dap.$ADAPTER.args`

`binary`、`args`、またはその両方を指定できます。`binary` には、_デバッガー_ 自体（`lldb` など）ではなく、_デバッグアダプター_（`lldb-dap` など）へのパスを指定する必要があります。`args` 設定は、Gram が通常アダプターに渡す引数をすべて上書きします。

```json
{
  "dap": {
    "CodeLLDB": {
      "binary": "/Users/name/bin/lldb-dap",
      "args": ["--wait-for-debugger"],
      "env": {"KEY": "whatever"},
      "ignore_system_version": false,
      "allow_binary_download": false,
      "enable_auto_updates": false,
    },
  },
}
```

## テーマ

デバッガーは以下のテーマオプションをサポートしています。

- `debugger.accent`: ブレークポイントおよびブレークポイント関連シンボルの強調表示に使用される色
- `editor.debugger_active_line.background`: アクティブなデバッグ行の背景色

## トラブルシューティング

デバッガーで問題が発生した場合、問題に関する詳細情報を収集するために役立つ機能がいくつかあります。

- デバッグパネルでセッションが実行中の場合、{#action dev::CopyDebugAdapterArguments} アクションを実行して、Gram がセッションをどのように初期化したかを説明する JSON データをクリップボードにコピーできます。これはセッションの開始に失敗したときに特に有用で、GitHub の Issue を作成する際に有益な文脈情報となります。
- また、{#action dev::OpenDebugAdapterLogs} アクションを使用すると、直近のデバッグセッション中に Gram がデバッグアダプターと行ったすべての通信のトレースログを確認できます。
