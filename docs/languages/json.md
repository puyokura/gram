# JSON

JSON はエディターでネイティブにサポートされています。

- Tree-sitter:
  [tree-sitter/tree-sitter-json](https://github.com/tree-sitter/tree-sitter-json)
- 言語サーバー:
  [zed-industries/json-language-server](https://github.com/zed-industries/json-language-server)

## JSONC

Gram は JSON のスーパーセットである JSONC もサポートしており、JSON ファイル内で
単一行コメント（`//`）を使用できます。これらのファイルを編集中は、`cmd-/`（macOS）
または `ctrl-/`（Linux）で現在の行または選択範囲のコメントを切り替えられます。

## JSONC の Prettier フォーマット

`*.jsonc` 拡張子のファイルで「Format Document」を使う場合や `format_on_save` を
有効にしている場合、Gram はフォーマッターとして Prettier を呼び出します。Prettier には、
`jsonc` 拡張子のファイルに末尾カンマを追加してしまう
[未解決の問題](https://github.com/prettier/prettier/issues/15956) があります。`.json`
拡張子を持つ JSONC ファイルは影響を受けません。

この動作を回避するには、`.prettierrc` 設定ファイルに次を追加します:

```json
{
  "overrides": [
    {
      "files": ["*.jsonc"],
      "options": {
        "parser": "json",
        "trailingComma": "none",
      },
    },
  ],
}
```

## JSON 言語サーバー

Gram は `package.json` と `tsconfig.json` の JSON Schema 検証を標準でサポートして
いますが、`json-language-server` はプロジェクト内のファイル、[JSON Schema Store](https://www.schemastore.org)
やその他の公開 URL にある JSON Schema 定義を使って JSON を検証できます。

### スキーマのインライン指定

JSON ファイル内にスキーマをインラインで指定するには、JSON スキーマファイルへの
リンクを持つトップレベルの `$schema` キーを追加します。

たとえば、[lua-language-server](https://github.com/LuaLS/lua-language-server/) で使う
`.luarc.json` の場合は次のとおりです:

```json
{
  "$schema": "https://raw.githubusercontent.com/sumneko/vscode-lua/master/setting/schema.json",
  "runtime.version": "Lua 5.4",
}
```

### 設定によるスキーマ指定

あるいは、Gram の LSP 設定で JSON Schema をファイルパスに関連付けることもできます。

設定は次のように記述します:

```json
"lsp": {
  "json-language-server": {
    "settings": {
      "json": {
        "schemas": [
          {
            "fileMatch": ["*/*.luarc.json"],
            "url": "https://raw.githubusercontent.com/sumneko/vscode-lua/master/setting/schema.json"
          }
        ]
      }
    }
  }
}
```

[対応している設定](https://github.com/Microsoft/vscode/blob/main/extensions/json-language-features/server/README.md#settings)
も、Gram の `settings.jsonc` に記述すれば json-language-server に渡せます:

<!--
TBD: フォーマッター（prettier）の設定（自動フォーマット、tab_size など）を追加する
-->
