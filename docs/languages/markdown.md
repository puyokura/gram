# Markdown

Markdown のサポートは Gram にネイティブで組み込まれています。

- Tree-sitter:
  [tree-sitter-markdown](https://github.com/tree-sitter-grammars/tree-sitter-markdown)
- 言語サーバー: [zk](https://zk-org.github.io/zk/index.html)

## コードブロックのシンタックスハイライト

Gram は [tree-sitter の言語文法](../extensions/languages.md#grammar) を利用して、
Markdown のコードブロックを言語別にシンタックスハイライトします。公式拡張機能や
コミュニティ拡張機能が提供するものを含め、[Gram がサポートする言語](../languages.md)
はすべて Markdown のコードブロックで利用できます。開始する <kbd>```</kbd>
コードフェンスの後に言語名を書くだけです:

````python
```python
import functools as ft

@ft.lru_cache(maxsize=500)
def fib(n):
    return n if n < 2 else fib(n - 1) + fib(n - 2)
```
````

## 構成

### 言語サーバー（zk）

Gram には `zk` zettelkasten 言語サーバーのサポートが組み込まれています。

有効にするには、settings.json に次を追加します:

```json
  "languages": {
    "Markdown": {
      "language_servers": ["zk"],
    }
  },
```

`zk` とメモ取り用の zettelkasten システムの詳細については
[ドキュメント](https://zk-org.github.io/zk/index.html) を参照してください。

### フォーマット

Gram は Prettier による Markdown 文書の自動再フォーマットをサポートしています。
{#action editor::Format} アクションで手動実行するか、{#kb editor::Format}
キーボードショートカットで実行できます。あるいは、settings.json で
[`format_on_save`](../configuring-gram.md#format-on-save) を有効にすると自動で
フォーマットできます:

```json
  "languages": {
    "Markdown": {
      "format_on_save": "on"
    }
  },
```

### 行末の空白

既定では Gram は保存時に末尾の空白を削除します。Markdown ファイルで不可視の末尾
空白が `<br />` に変換されることに依存している場合は、次の設定でこの動作を無効に
できます:

```json
  "languages": {
    "Markdown": {
      "remove_trailing_whitespace_on_save": false
    }
  },
```
