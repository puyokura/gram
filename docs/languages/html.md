# HTML

HTML のサポート（シンタックスハイライトと言語サーバー）はエディターに組み込まれて
います。

- [Tree-sitter 文法](https://github.com/tree-sitter/tree-sitter-html)

言語サーバーによるサポートは、2 つの異なるサーバーから利用できます:

- [SuperHTML](https://github.com/kristoff-it/superhtml)
- [microsoft/vscode-html-languageservice](https://github.com/microsoft/vscode-html-languageservice)

既定では Microsoft の言語サーバーは無効で、SuperHTML が有効です。言語サーバーを
切り替えるには、`settings.jsonc` に次を追加します:

```json
{
  "languages": {
    "HTML": {
      "language_servers": ["vscode-html-language-server", "!superhtml", "..."],
    },
  },
}
```

## フォーマット

既定では Gram は HTML のフォーマットに [Prettier](https://prettier.io/) を使用します。

Gram の `settings.jsonc` に次を追加すると、`format_on_save` を無効にできます:

```json
  "languages": {
    "HTML": {
      "format_on_save": "off",
    }
  }
```

{#kb editor::Format} で手動フォーマットを実行することも、コマンドパレット
（{#kb command_palette::Toggle}）を開いて "Format Document" を選ぶこともできます。

### LSP によるフォーマット

Prettier の代わりに言語サーバーの自動フォーマットを使うには、Gram の設定に次を
追加します:

```json
  "languages": {
    "HTML": {
      "formatter": "language_server",
    }
  }
```

Gram の `settings.json` で、`vscode-html-language-server` のさまざまな
[フォーマットオプション](https://code.visualstudio.com/docs/languages/html#_formatting)
をカスタマイズできます:

```json
  "lsp": {
    "vscode-html-language-server": {
      "settings": {
        "html": {
          "format": {
            // <html> と <head> の中身をインデントする（既定: false）
            "indentInnerHtml": true,
            // <svg> や <script> の中をフォーマットしない
            "contentUnformatted": "svg,script",
            // <div> と <p> の前に空行を追加する
            "extraLiners": "div,p"
          }
        }
      }
    }
  }
```

## 関連項目

- [CSS](./css.md)
- [JavaScript](./javascript.md)
- [TypeScript](./typescript.md)
