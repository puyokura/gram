# GDScript

Gram での Godot [GDScript](https://gdscript.com/) 言語サポートは、コミュニティが
保守する [GDScript 拡張機能](https://github.com/GDQuest/zed-gdscript) によって
提供されます。問題はこちらへ報告してください:
[https://github.com/GDQuest/zed-gdscript/issues](https://github.com/GDQuest/zed-gdscript/issues)

- Tree-sitter:
  [PrestonKnopp/tree-sitter-gdscript](https://github.com/PrestonKnopp/tree-sitter-gdscript)
  および
  [PrestonKnopp/tree-sitter-godot-resource](https://github.com/PrestonKnopp/tree-sitter-godot-resource)
- 言語サーバー:
  [gdscript-language-server](https://github.com/godotengine/godot)

## 前提条件

必要なもの:

- [Godot](https://godotengine.org/download/)。
- システム PATH 上の netcat（`nc` または `ncat`）。

## セットアップ

1. Godot エディター内で Editor Settings を開き、`Text Editor -> External` を探して
   次のオプションを設定します:
   - Exec Path: `/path/to/zed`
   - Exec Flags: `{project} {file}:{line}:{col}`
   - Use External Editor: "✅ On"
2. Godot から任意の \*.gd ファイルを開くと Gram が起動します。

## 使い方

Godot が実行中の場合、GDScript 拡張機能は Godot ランタイムが提供する言語サーバーに
接続し、`jump to definition`、Ctrl/cmd を押したときのホバー表示、その他の
言語サーバー機能を提供します。
