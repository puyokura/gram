# Scheme

## フォーマット

コードを自動でフォーマットするには、Scheme のコードフォーマッターである
[raviqqe/schemat](https://github.com/raviqqe/schemat) を使用できます。

インストール:

```bash
cargo install schemat
```

次に、Gram の `settings.json` に以下を追加します:

```json
"languages": {
  "Scheme": {
    "formatter": {
      "external": {
        "command": "schemat",
        "arguments": [""]
      }
    }
  }
}
```

## Tree-sitter

- Tree-sitter:
  [6cdh/tree-sitter-scheme](https://github.com/6cdh/tree-sitter-scheme)
