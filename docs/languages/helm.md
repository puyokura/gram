# Helm

Gram での Helm のサポートは、コミュニティが保守する
[Helm 拡張機能](https://github.com/cabrinha/helm.zed) によって提供されます。

- Tree-sitter:
  [tree-sitter-go-template](https://github.com/ngalaiko/tree-sitter-go-template/tree/master)
- 言語サーバー: [mrjosh/helm-ls](https://github.com/mrjosh/helm-ls)

## セットアップ

`.gram/settings.jsonc` を編集して次を追加すると、Helm ファイルで Helm 言語を有効にできます:

```json
  "file_types": {
    "Helm": [
      "**/templates/**/*.tpl",
      "**/templates/**/*.yaml",
      "**/templates/**/*.yml",
      "**/helmfile.d/**/*.yaml",
      "**/helmfile.d/**/*.yml",
      "**/values*.yaml"
    ]
  }
```

helm-ls がサポートしているため、これにより values.yaml ファイルも helm 型としてマークされます。
