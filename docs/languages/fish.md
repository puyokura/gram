# Fish

Gram での Fish 言語サポートは、コミュニティが保守する
[Fish 拡張機能](https://github.com/hasit/zed-fish) によって提供されます。
問題はこちらへ報告してください:
[https://github.com/hasit/zed-fish/issues](https://github.com/hasit/zed-fish/issues)

- Tree-sitter:
  [ram02z/tree-sitter-fish](https://github.com/ram02z/tree-sitter-fish)

### フォーマット

Gram は fish に同梱されている
[`fish_indent`](https://fishshell.com/docs/current/cmds/fish_indent.html) などの
外部ツールを使って、fish コードの自動フォーマットをサポートしています。

1. `fish_indent` が PATH にあることを確認し、バージョンを確認:

```sh
which fish_indent
fish_indent --version
```

2. `fish_indent` で fish コードを自動フォーマットするよう Gram を構成:

```json
  "languages": {
    "Fish": {
      "formatter": {
        "external": {
          "command": "fish_indent"
        }
      }
    }
  },
```
