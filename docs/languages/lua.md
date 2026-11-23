# Lua

Lua のサポートはエディターに組み込まれています。

- Tree-sitter:
  [tree-sitter-grammars/tree-sitter-lua](https://github.com/tree-sitter-grammars/tree-sitter-lua)
- 言語サーバー:
  [LuaLS/lua-language-server](https://github.com/LuaLS/lua-language-server)

## luarc.json

LuaLS を構成するには、ワークスペースのルートに `.luarc.json` ファイルを作成します。

```json
{
  "$schema": "https://raw.githubusercontent.com/LuaLS/vscode-lua/master/setting/schema.json",
  "runtime.version": "Lua 5.4",
  "format.enable": true,
  "workspace.library": ["../somedir/library"],
}
```

利用できる構成オプションのすべては
[LuaLS の設定ドキュメント](https://luals.github.io/wiki/settings/) を参照してください。
Gram でこのファイルを編集するときは、利用可能な設定オプションが自動補完されます
（たとえば `runtime.version` では `"Lua 5.1"`、`"Lua 5.2"`、`"Lua 5.3"`、`"Lua 5.4"`、
`"LuaJIT"` が許容値として表示されます）。VS Code から設定オプションを移す場合は、
`Lua.` プレフィックスを削除してください（たとえば `Lua.runtime.version` ではなく
`runtime.version`）。

### LuaCATS の定義

LuaLS は、LuaCATS（Lua Comment and Type System）の定義を利用して、LSP の自動補完候補と
型検証を強化できます。これらの定義は多くの一般的な Lua ライブラリ向けに公開されており、
定義を含むローカルパスは `luarc.json` の `workspace.library` で指定できます。定義を
プロジェクトの親ディレクトリ（`../playdate-luacats`、`../love2d` など）にチェックアウト
すれば、相対パスで指定できます。あるいは、LuaCATS の定義リポジトリごとにプロジェクト内へ
サブモジュールを作成する方法もあります。

### LÖVE (Love2D) {#love2d}

Gram で [LÖVE (Love2D)](https://love2d.org/) を使うには、
[LuaCATS/love2d](https://github.com/LuaCATS/love2d) をプロジェクトの親フォルダにある
`love2d-luacats` フォルダへチェックアウトします:

```sh
cd .. && git clone https://github.com/LuaCATS/love2d love2d-luacats
```

続いて `.luarc.json` に次を記述します:

```
{
  "$schema": "https://raw.githubusercontent.com/LuaLS/vscode-lua/master/setting/schema.json",
  "runtime.version": "Lua 5.4",
  "workspace.library": ["../love2d-luacats"],
  "runtime.special": {
    "love.filesystem.load": "loadfile"
  }
}
```

### PlaydateSDK

Gram で [Playdate Lua SDK](https://play.date/dev/) を使うには、
[playdate-luacats](https://github.com/notpeter/playdate-luacats) をプロジェクトの親
フォルダへチェックアウトします:

```sh
cd .. && git clone https://github.com/notpeter/playdate-luacats
```

続いて `.luarc.json` に次を記述します:

```json
{
  "$schema": "https://raw.githubusercontent.com/LuaLS/vscode-lua/master/setting/schema.json",
  "runtime.version": "Lua 5.4",
  "runtime.nonstandardSymbol": [
    "+=",
    "-=",
    "*=",
    "/=",
    "//=",
    "%=",
    "<<=",
    ">>=",
    "&=",
    "|=",
    "^=",
  ],
  "diagnostics.severity": { "duplicate-set-field": "Hint" },
  "diagnostics.globals": ["import"],
  "workspace.library": ["../playdate-luacats"],
  "format.defaultConfig": {
    "indent_style": "space",
    "indent_size": "4",
  },
  "format.enable": true,
  "runtime.builtin": { "io": "disable", "os": "disable", "package": "disable" },
}
```

### インラインヒント

Gram で LuaLS の[インラインヒント](../configuring-languages.md#inlay-hints)を有効にする
には、次の手順を実行します。

1. Gram の settings.jsonc に次を追加します:

```json
  "languages": {
    "Lua": {
      "inlay_hints": {
        "enabled": true,
        "show_type_hints": true,
        "show_parameter_hints": true,
        "show_other_hints": true
      }
    }
  }
```

2. `.luarc.json` に `"hint.enable": true` を追加します。

## フォーマット

### LuaLS によるフォーマット

LuaLS（[CppCXY/EmmyLuaCodeStyle](https://github.com/CppCXY/EmmyLuaCodeStyle) が提供）で
自動フォーマットを有効にするには、.luarc.json に `"format.enable": true,` があることを
確認します:

```json
{
  "$schema": "https://raw.githubusercontent.com/sumneko/vscode-lua/master/setting/schema.json",
  "format.enable": true,
}
```

続いて、Gram の `settings.jsonc` に次を追加します:

```json
{
  "languages": {
    "Lua": {
      "format_on_save": "on",
      "formatter": "language_server",
    },
  },
}
```

EmmyLuaCodeStyle の各種スタイルオプションは `.editorconfig` でカスタマイズできます。
利用できるオプションのすべては
[lua.template.editorconfig](https://github.com/CppCXY/EmmyLuaCodeStyle/blob/master/lua.template.editorconfig)
を参照してください。

### StyLua によるフォーマット

自動フォーマットに [StyLua](https://github.com/JohnnyMorganz/StyLua) を使う場合は、
次のようにします。

1. [StyLua](https://github.com/JohnnyMorganz/StyLua) をインストールします:
   `brew install stylua` または
   `cargo install stylua --features lua52,lua53,lua54,luau,luajit`（不要な Lua バージョンは
   自由に削除できます）。
2. `settings.jsonc` に次を追加します:

```json
{
  "languages": {
    "Lua": {
      "format_on_save": "on",
      "formatter": {
        "external": {
          "command": "stylua",
          "arguments": [
            "--syntax=Lua54",
            "--respect-ignores",
            "--stdin-filepath",
            "{buffer_path}",
            "-",
          ],
        },
      },
    },
  },
}
```

StyLua の各種オプションは、上記のコマンドライン（`--syntax=Lua54` など）または
ワークスペース内の `stylua.toml` で指定できます:

```toml
syntax = "Lua54"
column_width = 100
line_endings = "Unix"
indent_type = "Spaces"
indent_width = 4
quote_style = "AutoPreferDouble"
call_parentheses = "Always"
collapse_simple_statement = "All"

[sort_requires]
enabled = true
```

利用できるオプションの完全な一覧は
[StyLua のオプション](https://github.com/JohnnyMorganz/StyLua?tab=readme-ov-file#options)
を参照してください。
