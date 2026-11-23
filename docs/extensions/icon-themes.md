# アイコンテーマ

拡張機能は、Gram がフォルダーとファイルに使うアイコンを変更するためのアイコンテーマを提供できます。

## 拡張機能の例

[Material Icon Theme](https://github.com/zed-extensions/material-icon-theme) は、アイコンテーマを含む拡張機能の構造の例として参考になります。

## フォルダー構成

アイコンテーマの拡張機能には、重要なフォルダーが2つあります:

- `icon_themes`: このフォルダーには、アイコンテーマの定義を含む1つ以上の JSON ファイルを置きます。
- `icons`: このフォルダーには、拡張機能とともに配布されるアイコンアセットを置きます。必要に応じて、このフォルダー内にサブフォルダーを作成できます。

各アイコンテーマファイルは、[`https://zed.dev/schema/icon_themes/v0.3.0.json`](https://zed.dev/schema/icon_themes/v0.3.0.json) で定義された JSON スキーマに従う必要があります。

アイコンテーマの構造の例を次に示します:

```json [icon-theme]
{
  "$schema": "https://zed.dev/schema/icon_themes/v0.3.0.json",
  "name": "My Icon Theme",
  "author": "Your Name",
  "themes": [
    {
      "name": "My Icon Theme",
      "appearance": "dark",
      "directory_icons": {
        "collapsed": "./icons/folder.svg",
        "expanded": "./icons/folder-open.svg"
      },
      "named_directory_icons": {
        "stylesheets": {
          "collapsed": "./icons/folder-stylesheets.svg",
          "expanded": "./icons/folder-stylesheets-open.svg"
        }
      },
      "chevron_icons": {
        "collapsed": "./icons/chevron-right.svg",
        "expanded": "./icons/chevron-down.svg"
      },
      "file_stems": {
        "Makefile": "make"
      },
      "file_suffixes": {
        "mp3": "audio",
        "rs": "rust"
      },
      "file_icons": {
        "audio": { "path": "./icons/audio.svg" },
        "default": { "path": "./icons/file.svg" },
        "make": { "path": "./icons/make.svg" },
        "rust": { "path": "./icons/rust.svg" }
        // ...
      }
    }
  ]
}
```

各アイコンのパスは、拡張機能フォルダーのルートからの相対パスで解決されます。

この例では、拡張機能は次のような構造になります:

```
extension.toml
icon_themes/
  my-icon-theme.json
icons/
  audio.svg
  chevron-down.svg
  chevron-right.svg
  file.svg
  folder-open.svg
  folder.svg
  rust.svg
```
