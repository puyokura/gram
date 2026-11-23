# デバッガー拡張機能

[Debug Adapter Protocol](https://microsoft.github.io/debug-adapter-protocol) のサーバーを拡張機能として公開し、[デバッガー](../debugger.md)で使用できます。

## デバッガー拡張機能の定義

1つの拡張機能は、1つ以上の DAP サーバーを提供できます。各 DAP サーバーは `extension.toml` に登録する必要があります:

```toml
[debug_adapters.my-debug-adapter]
# Optional relative path to the JSON schema for the debug adapter configuration schema. Defaults to `debug_adapter_schemas/$DEBUG_ADAPTER_NAME_ID.json`.
# Note that while this field is optional, a schema is mandatory.
schema_path = "relative/path/to/schema.json"
```

次に、拡張機能の Rust コードで `get_dap_binary` メソッドを実装します:

```rust
impl gram::Extension for MyExtension {
    fn get_dap_binary(
        &mut self,
        adapter_name: String,
        config: DebugTaskDefinition,
        user_provided_debug_adapter_path: Option<String>,
        worktree: &Worktree,
    ) -> Result<DebugAdapterBinary, String>;
}
```

このメソッドは、デバッグアダプタープロトコルサーバーを起動するコマンドと、その動作に必要な引数や環境変数を返す必要があります。

GitHub Releases や npm などの外部ソースから DAP サーバーをダウンロードする必要がある場合も、この関数で行えます。この関数は、ユーザーがデバッグアダプターで新しいデバッグセッションを起動するたびに呼び出されるため、更新の確認は定期的に行うだけにしてください。

また `dap_request_kind` も実装する必要があります。この関数は、特定のデバッグシナリオが新しいデバッグ対象を _起動_ するのか、既存のデバッグ対象に _アタッチ_ するのかを判定するために使います。また、特定のデバッグシナリオが _ロケーター_ の実行を必要とするかの判定にも使います。

```rust
impl gram::Extension for MyExtension {
    fn dap_request_kind(
        &mut self,
        _adapter_name: String,
        _config: Value,
    ) -> Result<StartDebuggingRequestArgumentsRequest, String>;
}
```

この2つの関数で、`debug.jsonc` ベースのユーザーワークフローにデバッグアダプターを公開するには十分ですが、`dap_config_to_scenario` も実装することを強くおすすめします。

```rust
impl gram::Extension for MyExtension {
    fn dap_config_to_scenario(
        &mut self,
        _adapter_name: DebugConfig,
    ) -> Result<DebugScenario, String>;
}
```

`dap_config_to_scenario` は、ユーザーが新しいプロセスモーダルの UI からセッションを起動するときに使われます。大まかに言うと、汎用的なデバッグ構成（特定のデバッグアダプターに依存しないもの）を受け取り、それをあなたのアダプター用の具体的なデバッグシナリオに変換しようとします。言い換えれば、「プログラム、引数リスト、現在の作業フォルダー、環境変数が与えられたとき、このデバッグアダプターを起動するための構成はどうなるか」という問いに答えるものです。

## デバップロケーターの定義

Gram は、_デバップロケーター_ を使ってデバッグシナリオを自動作成する仕組みを提供しています。ロケーターはデバッグ対象を特定し、そのためのデバッグセッションを起動する方法を判断します。ロケーターのおかげで、既存のユーザータスク（例: `cargo run`）を自動的にデバッグシナリオ（例: `cargo build` の実行後、`target/debug/my_program` をデバッグ対象プログラムとしてデバッガーを起動する）に変換できます。

> 拡張機能がデバッグアダプターを公開していなくても、独自のデバップロケーターを定義できます。拡張機能がすでに言語タスクを公開している場合は、ユーザーがデバッグアダプターを手動で設定せずにデバッグセッションを起動できるようになるため、そうすることを強くおすすめします。

ロケーターは、使用されるデバッグアダプターに依存しない（ただし、そうする必要はありません）ものにできます。ロケーターは単にデバッグ対象の特定と、そのデバッグセッションの起動方法の判断を担います。これにより、より柔軟で拡張性の高いデバッグ体験が可能になります。

拡張機能は、1つ以上のデバップロケーターを定義できます。各デバップロケーターは `extension.toml` に登録する必要があります:

```toml
[debug_locators.my-debug-locator]
```

ロケーターには2つの要素があります。まず、各ロケーターは利用可能な各タスクに対して実行され、利用可能なロケーターの中に特定のタスク用のデバッグシナリオを提供できるものがあるかを判断します。これは `dap_locator_create_scenario` を呼び出すことで行われます。

```rust
impl gram::Extension for MyExtension {
    fn dap_locator_create_scenario(
        &mut self,
        _locator_name: String,
        _build_task: TaskTemplate,
        _resolved_label: String,
        _debug_adapter_name: String,
    ) -> Option<DebugScenario>;
}
```

この関数は、そのシナリオが特定のユーザータスクに対応するデバッグ用のものを定義する場合に `Some` のデバッグシナリオを返す必要があります。`DebugScenario` には[ビルドタスク](../debugger.md#build-tasks)を含められることに注意してください。ビルドタスクがある場合、それが正常に完了した後に `run_dap_locator` を実行します。

```rust
impl gram::Extension for MyExtension {
    fn run_dap_locator(
        &mut self,
        _locator_name: String,
        _build_task: TaskTemplate,
    ) -> Result<DebugRequest, String>;
}
```

`run_dap_locator` は、ビルドターゲットを決定的に特定できない場合に有用です。一部のビルドシステムは、名前を事前に知ることができない成果物を生成します。ただし、2段階の解決を経る必要は _ない_ ことに注意してください。`dap_locator_create_scenario` だけで完全なデバッグ構成を決定できる場合は、返す `DebugScenario` の `build` プロパティを省略できます。また、あなたのロケーターは、受け入れる可能性が低いタスクでも **呼び出される** ことに注意してください。そのため、高コストな処理を実行する前に、早めに `None` を返すよう努めるべきです。

## テスト

新しい Debug Adapter Protocol サーバー拡張機能をテストするには、それをインストールします。詳しくは[拡張機能の開発](../extensions/developing-extensions.md)を参照してください。
