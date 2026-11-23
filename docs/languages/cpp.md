# C++

C++ のサポートは Gram にネイティブで組み込まれています。

- Tree-sitter:
  [tree-sitter/tree-sitter-cpp](https://github.com/tree-sitter/tree-sitter-cpp)
- 言語サーバー: [clangd/clangd](https://github.com/clangd/clangd)

## バイナリ

Gram が使用する `clangd` バイナリを設定できます。

既定では、Gram は `$PATH` から `clangd` を探して使用します。そのバイナリが正常に
実行できればそれを使用し、そうでなければ Gram が独自にインストールした `clangd`
バージョンにフォールバックします。

プレリリース版の `clangd` をインストールしたい場合は、`settings.jsonc` で
`pre_release` を `true` に設定して Gram に指示できます:

```json
{
  "lsp": {
    "clangd": {
      "fetch": {
        "pre_release": true,
      },
    },
  },
}
```

Gram による `clangd` バイナリの探索を無効にしたい場合は、`settings.jsonc` で
`ignore_system_version` を `true` に設定します:

```json
{
  "lsp": {
    "clangd": {
      "binary": {
        "ignore_system_version": true,
      },
    },
  },
}
```

独自の場所にあるバイナリを使いたい場合は、`path` と任意の `arguments` を
指定できます:

```json
{
  "lsp": {
    "clangd": {
      "binary": {
        "path": "/path/to/clangd",
        "arguments": [],
      },
    },
  },
}
```

この `"path"` は絶対パスである必要があります。

## 引数

clangd には任意の数の引数を渡せます。利用可能なオプションの完全な一覧は、
コマンドラインから `clangd --help` を実行して確認してください。たとえば
`--function-arg-placeholders=0` を指定すると、補完には関数呼び出しの括弧だけが
含まれます。一方、既定値（`--function-arg-placeholders=1`）では、メソッドの
パラメーターのプレースホルダーも含まれます。

```json
{
  "lsp": {
    "clangd": {
      "binary": {
        "path": "/path/to/clangd",
        "arguments": ["--function-arg-placeholders=0"],
      },
    },
  },
}
```

## フォーマット

既定では、Gram は C++ コードのフォーマットに `clangd` 言語サーバーを使用します。
Clangd は `clang-format` CLI ツールと同じものです。設定を変えるには
`.clang-format` ファイルを追加します。例:

```yaml
# yaml-language-server: $schema=https://json.schemastore.org/clang-format-21.x.json
---
BasedOnStyle: LLVM
IndentWidth: 4
---
Language: Cpp
# C++ ではポインターを型側に揃える。
DerivePointerAlignment: false
PointerAlignment: Left
---
```

オプションの完全な一覧は
[Clang-Format のスタイルオプション](https://clang.llvm.org/docs/ClangFormatStyleOptions.html)
を参照してください。

フォーマットは {#kb editor::Format} で、またはコマンドパレットの `editor: format`
アクションで、あるいは Gram 設定に `format_on_save` を追加して実行できます:

```json
  "languages": {
    "C++": {
      "format_on_save": "on",
      "tab_size": 2
    }
  }
```

## その他のサーバー構成

プロジェクトのルートに `.clangd` ファイルを作成して追加の設定を行うのが一般的です。

```yaml
# yaml-language-server: $schema=https://json.schemastore.org/clangd.json
CompileFlags:
  Add:
    - "--include-directory=/path/to/include"
Diagnostics:
  MissingIncludes: Strict
  UnusedIncludes: Strict
```

clangd 設定ファイルのより高度な使い方については、
[公式ページ](https://clangd.llvm.org/config.html) を参照してください。

## コンパイルコマンド

プロジェクトによっては、Clangd が正しく解析するために `compile_commands.json`
ファイルが必要です。このファイルには、プロジェクトをどうビルドすべきかを clangd に
伝えるコンパイルデータベースが含まれます。

### CMake のコンパイルコマンド

CMake では、`CMakeLists.txt` に次の行を追加すると `compile_commands.json` を
自動生成できます:

```cmake
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
```

プロジェクトをビルドすると、CMake がビルドディレクトリに `compile_commands.json` を
生成し、clangd が自動的にそれを読み取ります。

## デバッグ

CodeLLDB または GDB を使ってネイティブバイナリをデバッグできます。（ビルドプロセスが
C++ コンパイラーに `-g` を渡し、生成されるバイナリにデバッグ情報を含めるように
してください。）`.gram/debug.json` に追加できるデバッグ構成の例を以下に示します。

- [CodeLLDB 設定ドキュメント](https://github.com/vadimcn/codelldb/blob/master/MANUAL.md#starting-a-new-debug-session)
- [GDB 設定ドキュメント](https://sourceware.org/gdb/current/onlinedocs/gdb.html/Debugger-Adapter-Protocol.html)
  - GDB は v14.1 以上が必要です

### バイナリのビルドとデバッグ

```json [debug]
[
  {
    "label": "Debug native binary",
    "build": {
      "command": "make",
      "args": ["-j8"],
      "cwd": "$GRAM_WORKTREE_ROOT"
    },
    "program": "$GRAM_WORKTREE_ROOT/build/prog",
    "request": "launch",
    "adapter": "CodeLLDB"
  }
]
```

## プロトコル拡張

Zed は現在、次の `clangd`
[拡張](https://clangd.llvm.org/extensions) を実装しています:

### 非アクティブな領域

プリプロセッサ指令（`#if`、`#ifdef`、`#ifndef` など）によって偽と評価された
非アクティブなコード領域を自動的に淡色表示します。

### ソースファイルとヘッダーファイルの切り替え

対応する C++ ソースファイル（`.cpp` など）とヘッダーファイル（`.h` など）を
切り替えられます。コマンドパレットから {#action
editor::SwitchSourceHeader} コマンドを実行するか、`editor::SwitchSourceHeader`
アクションにキーバインドを設定します。

```json
{
  "context": "Editor",
  "bindings": {
    "alt-enter": "editor::SwitchSourceHeader",
  },
}
```
