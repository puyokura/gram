# Dart

Dart のサポートは
[Dart 拡張機能](https://github.com/zed-extensions/dart) から利用できます。

- Tree-sitter:
  [UserNobody14/tree-sitter-dart](https://github.com/UserNobody14/tree-sitter-dart)
- 言語サーバー: [dart language-server](https://github.com/dart-lang/sdk)

## 前提条件

Dart SDK をインストールする必要があります。

dart は [dart.dev/get-dart](https://dart.dev/get-dart) から、または
[Flutter Version Management CLI (fvm)](https://fvm.app/documentation/getting-started/installation)
経由でインストールできます。

## 構成

`dart` が PATH にあれば、Dart 拡張機能に設定は不要です:

```sh
which dart
dart --version
```

特定の dart バイナリを使いたい場合や、FVM 経由で dart を使う場合は、
Gram の settings.jsonc ファイルで `dart` バイナリを指定できます:

```json
{
  "lsp": {
    "dart": {
      "binary": {
        "path": "/opt/homebrew/bin/fvm",
        "arguments": ["dart", "language-server", "--protocol=lsp"],
      },
    },
  },
}
```

### フォーマット

Dart は既定で非常に保守的な最大行長（80）を使います。自動フォーマット時に
dart LSP にもっと長い行長を許可したい場合は、Gram の settings.jsonc に次を
追加してください:

```json
{
  "lsp": {
    "dart": {
      "settings": {
        "lineLength": 140,
      },
    },
  },
}
```

dart 言語サーバーの機能について詳しくは、Dart のドキュメント
[dart language-server capabilities](https://github.com/dart-lang/sdk/blob/main/pkg/analysis_server/tool/lsp_spec/README.md)
を参照してください。
