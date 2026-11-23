# Swift

Gram での Swift 言語サポートは、コミュニティが保守する
[Swift 拡張機能](https://github.com/zed-extensions/swift)によって提供されます。問題はこちらへ報告してください:
[https://github.com/zed-extensions/swift/issues](https://github.com/zed-extensions/swift/issues)

- Tree-sitter:
  [alex-pinkus/tree-sitter-swift](https://github.com/alex-pinkus/tree-sitter-swift)
- 言語サーバー:
  [swiftlang/sourcekit-lsp](https://github.com/swiftlang/sourcekit-lsp)
- デバッグアダプター:
  [`lldb-dap`](https://github.com/swiftlang/llvm-project/blob/next/lldb/tools/lldb-dap/README.md)

## 言語サーバーの設定

ホームディレクトリまたはプロジェクトのルートに `.sourcekit-lsp/config.json` を作成することで、SourceKit LSP の動作を変更できます。完全なドキュメントは
[SourceKit-LSP の設定ファイル](https://github.com/swiftlang/sourcekit-lsp/blob/main/Documentation/Configuration%20File.md)
を参照してください。

## デバッグ

Swift 拡張機能は、Swift コードをデバッグするためのデバッグアダプターを提供します。Gram でのアダプター名（UI と `debug.jsonc` 内）は `Swift` で、内部では Swift ツールチェーンが提供する
[`lldb-dap`](https://github.com/swiftlang/llvm-project/blob/next/lldb/tools/lldb-dap/README.md)
を使用します。拡張機能は `swiftly` の使用、`xcrun` の使用、`$PATH` の検索の順に優先して `lldb-dap` バイナリを探します。`lldb-dap` が見つからない場合、拡張機能はダウンロードを試みません。

- [lldb-dap の設定ドキュメント](https://github.com/llvm/llvm-project/blob/main/lldb/tools/lldb-dap/README.md#configuration-settings-reference)

### 例

#### Swift バイナリのビルドとデバッグ

```json [debug]
[
  {
    "label": "Debug Swift",
    "build": {
      "command": "swift",
      "args": ["build"]
    },
    "program": "$GRAM_WORKTREE_ROOT/swift-app/.build/arm64-apple-macosx/debug/swift-app",
    "request": "launch",
    "adapter": "Swift"
  }
]
```
