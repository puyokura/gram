# Diff

Diff のサポートは Gram にネイティブで組み込まれています。

- Tree-sitter:
  [zed-industries/the-mikedavis/tree-sitter-diff](https://github.com/the-mikedavis/tree-sitter-diff)

## 構成

Gram は diff ファイルのフォーマットを試みず、
[`remove_trailing_whitespace_on_save`](../configuring-gram.md#remove-trailing-whitespace-on-save)
と
[`ensure-final-newline-on-save`](../configuring-gram.md#ensure-final-newline-on-save)
を false に設定しています。

Gram は `patch` と `diff` 拡張子のファイルを Diff ファイルとして自動認識します。
他の拡張子を認識させるには、Gram の `settings.jsonc` の `file_types` に追加します:

```json
  "file_types": {
    "Diff": ["dif"]
  },
```
