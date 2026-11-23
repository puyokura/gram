# Erlang

Erlang のサポートはエディターに組み込まれています。

- Tree-sitter:
  [WhatsApp/tree-sitter-erlang](https://github.com/WhatsApp/tree-sitter-erlang)
- 言語サーバー:
  - [erlang-ls/erlang_ls](https://github.com/erlang-ls/erlang_ls)
  - [WhatsApp/erlang-language-platform](https://github.com/WhatsApp/erlang-language-platform)

## 言語サーバーの選択

Erlang 拡張機能は `erlang_ls` と `erlang-language-platform` の言語サーバーを
サポートしています。

既定では `erlang_ls` が有効です。

`erlang-language-platform` に切り替えるには、`settings.jsonc` に次を追加します:

```json
{
  "languages": {
    "Erlang": {
      "language_servers": ["elp", "!erlang-ls", "..."],
    },
  },
}
```

## 関連項目:

- [Elixir](./elixir.md)
- [Gleam](./gleam.md)
