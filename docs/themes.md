# テーマ

Gram には多数の組み込みテーマが付属しており、追加のテーマは拡張機能として利用できます。

## テーマを選択する

インストール済みのテーマを確認し、プレビューするにはテーマセレクターを使います。コマンドパレットから `theme selector: toggle` で開けます（{#kb theme_selector::Toggle} に割り当て済み）。

テーマ一覧を上下に移動して見ると、テーマはその場で即座に切り替わり、enter キーを押すと設定ファイルに保存されます。

## テーマを追加でインストールする

Gram は Zed のテーマを利用できるため、[zed-themes.com](https://zed-themes.com) から任意のテーマをインストールできます。このギャラリーでは多くのテーマのプレビューを確認できます。

## テーマを設定する

選択したテーマは設定ファイルに保存されます。設定ファイルは、コマンドパレットから {#action gram::OpenSettingsFile}（{#kb gram::OpenSettingsFile} に割り当て済み）で開けます。

既定では、Gram はライトモード用とダークモード用の 2 つのテーマを保持します。現在のシステムモードを無視したい場合は、モードを `"dark"` または `"light"` に設定してください。

```json
{
  "theme": {
    "mode": "system",
    "light": "One Light",
    "dark": "One Dark",
  },
}
```

## テーマの上書き

テーマの特定の属性を上書きするには、`theme_overrides` 設定を使います。この設定ではテーマ固有の上書きを構成できます。

たとえば、エディターの背景色を変更し、コメントとドキュメントコメントをイタリックで表示したい場合は、`settings.json` に次の内容を追加します:

```json
{
  "theme_overrides": {
    "One Dark": {
      "editor.background": "#333",
      "syntax": {
        "comment": {
          "font_style": "italic",
        },
        "comment.doc": {
          "font_style": "italic",
        },
      },
      "accents": [
        "#ff0000",
        "#ff7f00",
        "#ffff00",
        "#00ff00",
        "#0000ff",
        "#8b00ff",
      ],
    },
  },
}
```

キャプチャ（`comment` や `comment.doc` など）の一覧については、
[言語拡張機能: シンタックスハイライト](./extensions/languages.md#syntax-highlighting) を参照してください。

利用可能なテーマ属性の一覧は、各テーマの JSON ファイルで確認できます。

## ローカルテーマ

新しいテーマをローカルに保存するには、`~/.config/gram/themes` ディレクトリ（macOS と Linux）または `%USERPROFILE%\AppData\Roaming\Gram\themes\`（Windows）に配置します。

たとえば `my-cool-theme` という新しいテーマを作るには、そのディレクトリに `my-cool-theme.json` というファイルを作成します。次回 Gram を起動したときにテーマセレクターで利用できます。

## テーマの開発

参照: [Gram テーマの開発](./extensions/themes.md)
