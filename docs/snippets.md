# スニペット

{#action snippets::ConfigureSnippets} アクションを使用して、指定した [スコープ](#scopes) の新しいスニペットファイルを作成したり、既存のスニペットファイルを編集したりできます。

スニペットファイルは `~/.config/gram/snippets` ディレクトリに配置されており、{#action snippets::OpenFolder} アクションを使用してそのフォルダーを開くことができます。

## 設定例

```json
{
  // 各スニペットには name と body が必須ですが、prefix と description は省略可能です。
  // prefix はスニペットを呼び出すトリガーとして使用されます。省略した場合は name が使用されます。
  // タブストップを定義するには、$1、$2、または ${1:defaultValue} のようなプレースホルダーを使用します。
  // $0 は最終的なカーソル位置を指定します。
  // 同じ値を持つプレースホルダーは連動します。
  "Log to console": {
    "prefix": "log",
    "body": ["console.info(\"Hello, ${1:World}!\")", "$0"],
    "description": "Logs to console",
  },
}
```

## スコープ

スコープは小文字の言語名で決定されます（例: Python の場合は `python.json`、Shell Script の場合は `shell script.json`）。ただし、この規則にはいくつかの例外があります:

| スコープ | ファイル名 |
| ---------- | --------------- |
| Global | snippets.json |
| JSX | javascript.json |
| Plain Text | plaintext.json |

JSX のスニペットを作成するには、`jsx.json` ではなく `javascript.json` スニペットファイルを使用する必要があります。ただし、これは TSX や TypeScript には適用されず、通常の規則に従います。

## 既知の制限事項

- prefix のリストが渡された場合、先頭の prefix のみが使用されます。
- `simple-completion-language-server` は `json` と `toml` の両方のファイル形式をサポートしていますが、現在のところ `json` スニペットファイル形式のみがサポートされています。

## 関連項目

`simple-completion-language-server` の `feature_paths` オプションはデフォルトで無効になっています。

これを有効にしたい場合は、`settings.json` に以下を追加してください:

```json
{
  "lsp": {
    "snippet-completion-server": {
      "settings": {
        "feature_paths": true,
      },
    },
  },
}
```

設定の詳細については、[`simple-completion-language-server` の説明書](https://github.com/zed-industries/simple-completion-language-server/tree/main) を参照してください。
