# Jsonnet

Gram での Jsonnet 言語サポートは、コミュニティが保守する
[Jsonnet 拡張機能](https://github.com/narqo/zed-jsonnet) によって提供されます。

- Tree-sitter:
  [sourcegraph/tree-sitter-jsonnet](https://github.com/sourcegraph/tree-sitter-jsonnet)
- 言語サーバー:
  [grafana/jsonnet-language-server](https://github.com/grafana/jsonnet-language-server)

## 構成

ワークスペースの構成オプションは、`settings.jsonc` の `lsp` 設定を通じて言語サーバーに渡せます。

次の例は、`jsonnet-language-server` で [tanka](https://tanka.dev) のインポートパスを解決するサポートを有効にします:

```json
{
  "lsp": {
    "jsonnet-language-server": {
      "settings": {
        "resolve_paths_with_tanka": true,
      },
    },
  },
}
```
