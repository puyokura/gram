# C

C のサポートは Gram にネイティブで組み込まれています。

- Tree-sitter:
  [tree-sitter/tree-sitter-c](https://github.com/tree-sitter/tree-sitter-c)
- 言語サーバー: [clangd/clangd](https://github.com/clangd/clangd)
- デバッグアダプター: [CodeLLDB](https://github.com/vadimcn)（プライマリ）、
  [GDB](https://sourceware.org/gdb/)（セカンダリ、Apple シリコンでは利用不可）

## Clangd: C として強制検出する

Clangd は既定で C++/C が混在するプロジェクトを想定します。C のみのプロジェクトでは、
`-xc` フラグを使ってすべてのファイルを C として扱うよう clangd に指示できます。
そのためには、プロジェクトのルートに次の内容で `.clangd` ファイルを作成します:

```yaml
# yaml-language-server: $schema=https://json.schemastore.org/clangd.json
CompileFlags:
  Add: [-xc]
```

既定では clang と gcc は `*.C` と `*.H`（大文字の拡張子）を C ではなく C++ として
認識するため、Gram もこの慣習に従います。C のみのプロジェクト（`FILENAME.C` のような
レガシーな大文字パスを使う場合など）を扱うなら、設定に次を追加してこの動作を
上書きできます:

```json
{
  "file_types": {
    "C": ["C", "H"],
  },
}
```

## フォーマット

既定では、エディターは `clang-format` CLI ツールと同様に、C コードのフォーマットに
`clangd` 言語サーバーを使用します。設定を変えるには `.clang-format` ファイルを
追加します。例:

```yaml
# yaml-language-server: $schema=https://json.schemastore.org/clang-format-21.x.json
---
BasedOnStyle: GNU
IndentWidth: 2
---
```

オプションの完全な一覧は
[Clang-Format のスタイルオプション](https://clang.llvm.org/docs/ClangFormatStyleOptions.html)
を参照してください。

フォーマットは {#kb editor::Format} で、またはコマンドパレットの `editor: format`
アクションで、あるいは Gram 設定に `format_on_save` を追加して実行できます:

```json
  "languages": {
    "C": {
      "format_on_save": "on",
      "tab_size": 2
    }
  }
```

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
C コンパイラーに `-g` を渡し、生成されるバイナリにデバッグ情報を含めるようにしてください。）
`.gram/debug.jsonc` に追加できるデバッグ構成の例を以下に示します。

- [CodeLLDB 設定ドキュメント](https://github.com/vadimcn/codelldb/blob/master/MANUAL.md#starting-a-new-debug-session)
- [GDB 設定ドキュメント](https://sourceware.org/gdb/current/onlinedocs/gdb.html/Debugger-Adapter-Protocol.html)

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
