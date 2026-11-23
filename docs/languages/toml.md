# TOML

TOML のサポートはエディターに組み込まれています。

- Tree-sitter:
  [tree-sitter/tree-sitter-toml](https://github.com/tree-sitter/tree-sitter-toml)

- クレート: [adot-tree-sitter-toml](https://crates.io/crates/adot-tree-sitter-toml)

[Tombi](https://github.com/tombi-toml/tombi) と [Taplo](https://taplo.tamasfe.dev) の LSP サポートがあります。既定は Tombi です。

`tombi` のみを有効にするには:

```json
  "languages": {
    "TOML": {
      "language_servers": ["!taplo", "tombi", "..."]
    },
  }
```

`taplo` のみを有効にするには:

```json
  "languages": {
    "TOML": {
      "language_servers": ["taplo", "!tombi", "..."]
    },
  }
```

Tombi はフォーマッターとしても使用できます（既定で有効）。代わりに taplo と Prettier を使いたい場合は、フォーマッターの設定を `language_server` から `prettier` に変更します:

```json
  "languages": {
    "TOML": {
      "language_servers": ["taplo", "!tombi", "..."]
      "formatter": "prettier",
      "format_on_save": "off",
    },
  }
```
