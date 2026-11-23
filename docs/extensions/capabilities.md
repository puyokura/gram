# 拡張機能のケーパビリティ

Gram の拡張機能が実行できる操作は、ケーパビリティ（権限）システムによって制御されます。

## ケーパビリティの制限

ユーザーは、拡張機能に付与されるケーパビリティを制限するオプションを持っています。

これは `granted_extension_capabilities` 設定で制御します。

ケーパビリティを制限または削除すると、拡張機能が十分なケーパビリティなしで対応する拡張 API を呼び出そうとしたときにエラーが返されます。

たとえば、ダウンロードを GitHub 上のファイルのみに制限したい場合は、`download_file` ケーパビリティの `host` を変更できます:

```diff
{
  "granted_extension_capabilities": [
    { "kind": "process:exec", "command": "*", "args": ["**"] },
-   { "kind": "download_file", "host": "*", "path": ["**"] },
+   { "kind": "download_file", "host": "github.com", "path": ["**"] },
    { "kind": "npm:install", "package": "*" }
  ]
}
```

拡張機能が _いかなる_ ケーパビリティも実行できるようにしたくない場合は、付与したケーパビリティをすべて削除できます:

```json
{
  "granted_extension_capabilities": []
}
```

> これにより、少なくとも既定の構成では、多くの拡張機能が機能しなくなる可能性が高いことに注意してください。

## ケーパビリティ

### `process:exec`

`process:exec` ケーパビリティは、拡張機能に [`zed_extension_api::process::Command`](https://docs.rs/zed_extension_api/latest/zed_extension_api/process/struct.Command.html) を使ってコマンドを呼び出す権限を付与します。

#### 例

任意のコマンドを任意の引数で実行できるようにするには:

```toml
{ kind = "process:exec", command = "*", args = ["**"] }
```

特定のコマンド（例: `gem`）を任意の引数で実行できるようにするには:

```toml
{ kind = "process:exec", command = "gem", args = ["**"] }
```

### `download_file`

`download_file` ケーパビリティは、拡張機能に [`zed_extension_api::download_file`](https://docs.rs/zed_extension_api/latest/zed_extension_api/fn.download_file.html) を使ってファイルをダウンロードする権限を付与します。

#### 例

任意のファイルをダウンロードできるようにするには:

```toml
{ kind = "download_file", host = "github.com", path = ["**"] }
```

`github.com` から任意のファイルをダウンロードできるようにするには:

```toml
{ kind = "download_file", host = "github.com", path = ["**"] }
```

特定の GitHub リポジトリから任意のファイルをダウンロードできるようにするには:

```toml
{ kind = "download_file", host = "github.com", path = ["zed-industries", "zed", "**"] }
```

### `npm:install`

`npm:install` ケーパビリティは、拡張機能に [`zed_extension_api::npm_install_package`](https://docs.rs/zed_extension_api/latest/zed_extension_api/fn.npm_install_package.html) を使って npm パッケージをインストールする権限を付与します。

#### 例

任意の npm パッケージをインストールできるようにするには:

```toml
{ kind = "npm:install", package = "*" }
```

特定の npm パッケージ（例: `typescript`）をインストールできるようにするには:

```toml
{ kind = "npm:install", package = "typescript" }
```
