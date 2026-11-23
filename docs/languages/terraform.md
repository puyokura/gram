# Terraform

Terraform のサポートは、[Terraform 拡張機能](https://github.com/zed-extensions/terraform)を通じて利用できます。

- Tree-sitter:
  [MichaHoffmann/tree-sitter-hcl](https://github.com/MichaHoffmann/tree-sitter-hcl)
- 言語サーバー:
  [hashicorp/terraform-ls](https://github.com/hashicorp/terraform-ls)

## 設定

<!--
TBD: Add example using `rootModulePaths` to match upstream example https://github.com/hashicorp/terraform-ls/blob/main/docs/SETTINGS.md#vs-code
-->

Terraform 言語サーバーは `settings.jsonc` で設定できます。例:

```json
{
  "lsp": {
    "terraform-ls": {
      "initialization_options": {
        "experimentalFeatures": {
          "prefillRequiredFields": true,
        },
      },
    },
  },
}
```

サーバー設定の全一覧は
[こちら](https://github.com/hashicorp/terraform-ls/blob/main/docs/SETTINGS.md)を参照してください。
