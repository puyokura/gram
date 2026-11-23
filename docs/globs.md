# Glob（グロブ）

[Glob パターン](https://en.wikipedia.org/wiki/Glob_(programming))は、sh、bash、zsh などでサポートされている `*.md` や `docs/**/*.md` のような Unix シェル形式のパス一致ワイルドカードの正式名称です。Glob は [正規表現（regex）](https://en.wikipedia.org/wiki/Regular_expression) と似ていますが別物です。Gram では主にファイル名の一致判定に使用されます。

## Glob の種類

Gram では、Glob パターンの照合に 2 つの異なる Rust クレートを使用しています。

- `.gitignore` ファイル内に記述された Glob パターンの照合には [ignore クレート](https://docs.rs/ignore/latest/ignore/)
- Gram 内でのファイルパスの照合には [glob クレート](https://docs.rs/glob/latest/glob/)

単純な表現（例: `ls *.py` や gitignore 内の `*.tmp`）は異なる環境間でも共通して機能しますが、より高度な機能（文字クラス、除外、`**` など）のサポートや構文には実装間で大きな違いがあります。本ドキュメントの以降の部分では、`glob` クレートの実装を介して Gram でサポートされている Glob について説明します。`.gitignore`、シェル、その他のプログラミング言語における Glob パターン構文のドキュメントリンクについては、後述の [参考文献](#references) を参照してください。

`glob` クレートは完全に Rust だけで実装されており、libc が提供する `glob` や `fnmatch` のインターフェイスに依存していません。そのため、Gram の Glob はプラットフォームを問わず同様に動作します。

## はじめに

Glob の「パターン」は、ファイル名や完全なファイルパスを照合するために使用されます。たとえば、「全ファイル検索」{#kb project_search::ToggleFocus} を使用しているときに、じょうご型の「フィルターの切り替え」ボタンをクリックするか {#kb project_search::ToggleFilters} を押すと、「Include」および「Exclude」の追加検索フィールドが表示され、ファイルパスやファイル名を照合するための Glob パターンを指定できます。

Glob パターンを作成する際は、1 つまたは複数の特殊文字を使用できます。

| 特殊文字 | 意味 |
| ----------------- | ----------------------------------------------------------------- |
| `?` | 任意の 1 文字に一致 |
| `*` | 任意の（空を含む）文字列に一致 |
| `**` | 現在のディレクトリおよび任意のサブディレクトリに一致 |
| `[abc]` | 括弧内の任意の 1 文字に一致 |
| `[a-z]` | 指定した範囲の文字のいずれかに一致（Unicode 順） |
| `[!...]` | `[...]` の否定（括弧内に含まれない文字に一致） |

注意点:

1. `{a,b,c}` のようなシェル形式の中括弧展開（ブレース展開）はサポートされていません。
2. 括弧内でリテラルの `-` 文字に一致させるには、先頭 `[-abc]` または末尾 `[abc-]` に配置する必要があります。
3. リテラルの `[` 文字に一致させるには、`[[]` を使用するか、グループの先頭文字 `[[abc]` として配置します。
4. リテラルの `]` 文字に一致させるには、`[]]` を使用するか、グループの末尾文字 `[abc]]` として配置します。

## 使用例

### ファイル拡張子の一致

Markdown ファイルのみを検索したい場合は、「Include」検索フィールドに `*.md` を入力します。

### 大文字・小文字を区別しない一致

Gram の Glob は大文字と小文字を区別するため、`*.c` は `main.C` に一致しません（macOS の HFS+/APFS のように大文字と小文字を区別しないファイルシステムであっても同様です）。代わりに括弧を使用して文字を照合します。つまり、`*.c` の代わりに `*.[cC]` を使用します。

### ディレクトリの一致

[Gram リポジトリ](https://codeberg.org/GramEditor/gram) 内で [言語サーバーの設定](./configuring-languages#configuring-language-servers.md)（Gram の `settings.jsonc` 内の `"lsp"` 項目）の例を検索したい場合、検索ボックスに `"lsp"` と入力し、「Include」フィルターに `docs/**/*.md` を指定します。これにより、`docs` ディレクトリまたはその下層の任意のサブディレクトリ `**/` 配下にあり、ファイル名が `.md` で終わるファイルのみが一致します。

代わりに、Gram の言語別ドキュメントページのみに対象を絞り込みたい場合は、`docs/languages/*.md` というより限定的なパターンを定義できます。これは `docs/languages/rust.md` や `docs/languages/cpp.md` に一致しますが、`docs/configuring-languages.md` には一致しません。

### 暗黙的なワイルドカード

プロジェクト検索の「Include」/「Exclude」フィルターを使用する場合、各 Glob は暗黙的なワイルドカードで囲まれます。たとえば、パスやファイル名に license を含むすべてのファイルを検索から除外したい場合は、除外ボックスに `license` と入力するだけです。内部的に Gram は `license` を `**license**` に変換します。つまり、`license.*` や `*.license` という名前のファイル、あるいは `license` サブディレクトリ内のファイルがすべて除外されます。これにより、毎回 `**/*.ts` と入力しなくても、手軽に `*.ts` でフィルターをかけることができます。

一方、Gram 設定で特定のディレクトリにのみ適用される [`file_types`](./configuring-gram.md#file-types) の上書きを行いたい場合は、明示的にワイルドカード Glob を含める必要があります。たとえば、拡張子 `html` のテンプレートファイルが置かれたディレクトリがあり、それを Jinja2 テンプレートとして認識させたい場合は、以下のように記述します。

```json
{
  "file_types": {
    "C++": ["[cC]"],
    "Jinja2": ["**/templates/*.html"],
  },
}
```

## 参考文献

Gram での Glob は上記のように実装されていますが、他のプログラミング言語で Glob を使用したコードを書く際は、各プラットフォームの Glob ドキュメントを参照してください。

- [macOS fnmatch](https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man3/fnmatch.3.html)（BSD C 標準ライブラリ）
- [Linux fnmatch](https://www.gnu.org/software/libc/manual/html_node/Wildcard-Matching.html)（GNU C 標準ライブラリ）
- [POSIX fnmatch](https://pubs.opengroup.org/onlinepubs/9699919799/functions/fnmatch.html)（POSIX 仕様）
- [node-glob](https://github.com/isaacs/node-glob)（Node.js `glob` パッケージ）
- [Python glob](https://docs.python.org/3/library/glob.html)（Python 標準ライブラリ）
- [Golang glob](https://pkg.go.dev/path/filepath#Match)（Go 標準ライブラリ）
- [gitignore patterns](https://git-scm.com/docs/gitignore)（Gitignore パターンフォーマット）
- [PowerShell: About Wildcards](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_wildcards)（PowerShell におけるワイルドカード）
