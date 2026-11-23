# C#

注意: 設定で使用する言語名は `CSharp` であり、`C#` ではありません。

C# のサポートは
[C# 拡張機能](https://github.com/zed-extensions/csharp) から利用できます。

- Tree-sitter:
  [tree-sitter/tree-sitter-c-sharp](https://github.com/tree-sitter/tree-sitter-c-sharp)
- 言語サーバー:
  [OmniSharp/omnisharp-roslyn](https://github.com/OmniSharp/omnisharp-roslyn)

## 構成

`OmniSharp` バイナリは Gram 設定ファイルで次のように構成できます:

```json
{
  "lsp": {
    "omnisharp": {
      "binary": {
        "path": "/path/to/OmniSharp",
        "arguments": ["optional", "additional", "args", "-lsp"],
      },
    },
  },
}
```
