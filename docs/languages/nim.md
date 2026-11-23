# Nim

Gram での Nim 言語サポートは、コミュニティが管理する
[Nim 拡張機能](https://github.com/foxoman/zed-nim) によって提供されます。問題の報告先:
[https://github.com/foxoman/zed-nim/issues](https://github.com/foxoman/zed-nim/issues)

- Tree-sitter:
  [alaviss/tree-sitter-nim](https://github.com/alaviss/tree-sitter-nim)
- 言語サーバー: [nim-lang/langserver](https://github.com/nim-lang/langserver)

## フォーマット

フォーマッターとして [arnetheduck/nph](https://github.com/arnetheduck/nph) を使用するには、
[nph のインストール手順](https://github.com/arnetheduck/nph?tab=readme-ov-file#installation)
に従い、Gram の `settings.json` に次を追加します:

```json
  "languages": {
    "Nim": {
      "formatter": {
        "external": {
          "command": "nph",
          "arguments": ["-"]
        }
      }
    }
  }
```
