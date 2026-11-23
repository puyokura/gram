# 補完

「コード補完」は、Gram によって自動的にインストールされる言語サーバー（LSP）、または [Gram の言語拡張機能](./languages.md) 経由で提供されます。

## 言語サーバーによるコード補完 {#code-completions}

適切な言語サーバーが利用可能な場合、Gram は現在のファイル内の変数名、関数、その他のシンボルの補完候補を提示します。これを無効化するには、Gram の `settings.jsonc` ファイルに以下を追加します。

```json
"show_completions_on_input": false
```

`ctrl-space` を押すか、コマンドパレットから `editor::ShowCompletions` アクションを実行することで、手動で補完を呼び出すこともできます。

コード補完では、オプションで項目の種類（関数、プロパティ、定数など）を示すテキストアイコンを表示できます。デフォルトは `"Off"` です。有効にするには次のように設定します。

```json
"completion_menu_item_kind": "symbol"
```

詳細については、以下を参照してください。

- [対応言語の設定](./configuring-languages.md)
- [Gram が対応している言語一覧](./languages.md)
