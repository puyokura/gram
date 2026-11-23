# SuperTab

Vim の [SuperTab](https://www.vim.org/scripts/script.php?script_id=1643) プラグインに着想を得たこの機能は、Tab キーにコンテキスト依存の動作を持たせることで、同じキーでインデント、タブ文字の直接挿入、コード補完のすべてをまかなえるようにします。

基本的な考え方は、ユーザーが Tab キーを押したときの現在のコンテキスト（文脈）に基づいて、最適な動作を判断するというものです。カーソルがテキストの末尾にある場合、SuperTab を起動すると補完メニューが開きます。カーソルが行頭にあるか空白文字のみが前にある場合は、その行をインデントします。それ以外の状況では、タブ文字を挿入します。

SuperTab を有効にするには、`keymap.jsonc` で Tab キーを {#action editor::SuperTab} に割り当ててください:

```json
{
  "context": "Editor && mode == full",
  "use_key_equivalents": true,
  "bindings": {
    "tab": "editor::SuperTab"
  },
}
```

## フォールバックモード

デフォルトでは、行頭でも補完ポイントでもない場合、SuperTab はリテラルの `tab` 文字の挿入にフォールバックします。補完中以外は常にインデントさせたい場合は、`settings.jsonc` または設定インターフェースで `supertab_fallback` を設定できます:

```json
{
  "supertab_fallback": "indent", // デフォルトは "tab"
}
```
