# XML

XML のシンタックスハイライトはネイティブにサポートされています。

- Tree-sitter:
  [tree-sitter-grammars/tree-sitter-xml](https://github.com/tree-sitter-grammars/tree-sitter-xml)

## 設定

XML として自動認識されない追加のファイル拡張子がある場合は、Gram の設定で
[file_types](../configuring-gram.md#file-types) に追加してください:

```json
  "file_types": {
    "XML": ["rdf", "gpx", "kml"]
  }
```

## フォーマッター

[言語固有の設定](../configuring-languages.md)を通じて、XML ファイルに別のフォーマッターを設定することもできます。[xmllint](https://gnome.pages.gitlab.gnome.org/libxml2/xmllint.html) は XML のパースに広く使われるツールで、XML ファイルのフォーマットも行えます。通常は [libxml2](https://gitlab.gnome.org/GNOME/libxml2/-/wikis/home) の一部としてプリインストールされています:

```json
"languages": {
  "XML": {
    "formatter": {
      "external": {
        "command": "xmllint",
        "arguments": ["--format", "-"]
      },
    }
  }
}
```
