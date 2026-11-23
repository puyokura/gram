# Elm

Elm のサポートは
[Elm 拡張機能](https://github.com/zed-extensions/elm) から利用できます。

- Tree-sitter:
  [elm-tooling/tree-sitter-elm](https://github.com/elm-tooling/tree-sitter-elm)
- 言語サーバー:
  [elm-tooling/elm-language-server](https://github.com/elm-tooling/elm-language-server)

## セットアップ

Gram の Elm サポートには `elm`、`elm-format`、`elm-review` のインストールが必要です。

1. [Elm をインストール](https://guide.elm-lang.org/install/elm.html) します
   （macOS では `brew install elm` でも可）。
2. コードリンティングをサポートするため `elm-review` をインストール:
   ```sh
   npm install elm-review --save-dev
   ```
3. 自動フォーマットをサポートするため `elm-format` をインストール
   ```sh
   npm install -g elm-format
   ```

## `elm-language-server` の構成

Elm 言語サーバーは `settings.jsonc` で構成できます。例:

```json
{
  "lsp": {
    "elm-language-server": {
      "initialization_options": {
        "disableElmLSDiagnostics": true,
        "onlyUpdateDiagnosticsOnSave": false,
        "elmReviewDiagnostics": "warning",
      },
    },
  },
}
```

`elm-format`、`elm-review`、`elm` はインストールして環境から利用できるようにするか、
設定で指定する必要があります。
[サーバー設定の完全な一覧はこちら](https://github.com/elm-tooling/elm-language-server?tab=readme-ov-file#server-settings)
を参照してください。
