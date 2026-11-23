# R

R のサポートは複数の R 向け Gram 拡張機能を通じて利用できます:

- [ocsmit/zed-r](https://github.com/ocsmit/zed-r)

  - Tree-sitter: [r-lib/tree-sitter-r](https://github.com/r-lib/tree-sitter-r)
  - 言語サーバー:
    [REditorSupport/languageserver](https://github.com/REditorSupport/languageserver)

- [posit-dev/air](https://github.com/posit-dev/air/tree/main/editors/zed)
  - フォーマッター: [posit-dev/air](https://posit-dev.github.io/air/)

## インストール

1. [R のダウンロードとインストール](https://cloud.r-project.org/)。
2. R パッケージの `languageserver` と `lintr` をインストールします:

```R
install.packages("languageserver")
install.packages("lintr")
```

3. 基本的な R 言語サポート（構文ハイライト、tree-sitter サポート）および
   [REditorSupport/languageserver](https://github.com/REditorSupport/languageserver)
   サポートのために、Gram の拡張機能マネージャーから [R](https://github.com/ocsmit/zed-r) 拡張機能をインストールします。

4. Air による R コードフォーマットのために、Gram の拡張機能マネージャーから [Air](https://posit-dev.github.io/air/) 拡張機能をインストールします。

## リント

`REditorSupport/languageserver` にはリンターとして
[r-lib/lintr](https://github.com/r-lib/lintr) のサポートがバンドルされています。これはプロジェクト内の `.lintr`（またはグローバルな既定値用のホームディレクトリ内）を使用して設定できます。

```r
linters: linters_with_defaults(
    line_length_linter(120),
    commented_code_linter = NULL
  )
exclusions: list(
    "inst/doc/creating_linters.R" = 1,
    "inst/example/bad.R",
    "tests/testthat/exclusions-test"
  )
```

あるいは、すべてのリントから除外することもできます:

```r
exclusions: list(".")
```

オプションの完全な一覧については、
[Using lintr](https://lintr.r-lib.org/articles/lintr.html)
を参照してください。

## フォーマット

### Air

[Air](https://posit-dev.github.io/air/) は、フォーマット時保存のサポートを含め、R のコードフォーマットを提供します。
[Gram 向け Air ドキュメント](https://posit-dev.github.io/air/editor-zed.html)
には、Gram で Air を実行するための最新情報が記載されています。

[ocsmit/zed-r](https://github.com/ocsmit/zed-r) 拡張機能（Gram での一般的な R 言語認識用）と [Air](https://posit-dev.github.io/air/) 拡張機能の両方がインストールされていることを確認してください。

`settings.json` で Air を有効化します:

```json
{
  "languages": {
    "R": {
      "language_servers": ["air"],
    },
  },
}
```

`REditorSupport/languageserver` の `"r_language_server"` を使用しつつ、フォーマットには Air を使用したい場合は、次の設定を使用します:

```json
{
  "languages": {
    "R": {
      "language_servers": ["air", "r_language_server"],
      "use_on_type_format": false,
    },
  },
}
```

この一覧では `"air"` が先頭にある必要があります。そうでない場合、`"r_language_server"` を介して
[r-lib/styler](https://github.com/r-lib/styler) が呼び出されます。

`"r_language_server"` は、Air のフォーマットルールとは異なる入力時フォーマットを提供します。これを完全に回避し、R ファイルのフォーマットを Air に完全に任せるには、上記のように `"use_on_type_format": false` も設定してください。

#### Air の設定

Air はプロジェクトのルートディレクトリに配置された `air.toml` ファイルを介して最小限の設定が可能です:

```toml
[format]
line-width = 80
indent-width = 2
```

詳細については、
[設定](https://posit-dev.github.io/air/configuration.html)
に関する Air のドキュメントを参照してください。

### Styler

`REditorSupport/languageserver` にはフォーマッターとして
[r-lib/styler](https://github.com/r-lib/styler) のサポートがバンドルされています。その動作をカスタマイズする方法の詳細については、
[Customizing Styler](https://cran.r-project.org/web/packages/styler/vignettes/customizing_styler.html)
を参照してください。

<!--
TBD: Get this working

### REditorSupport/languageserver Configuration

You can configure the [R languageserver settings](https://github.com/REditorSupport/languageserver#settings) via Gram Project Settings `.gram/settings.json` or Gram User Settings `~/.config/gram/settings.json`:

For example to disable Lintr linting and suppress code snippet suggestions (both enabled by default):

```json
{
  "lsp": {
    "r_language_server": {
      "settings": {
        "r": {
          "lsp": {
            "diagnostics": false,
            "snippet_support": false
          }
        }
      }
    }
  }
}
```

-->

<!--
TBD: R REPL Docs

## REPL

### Ark Installation

To use the Gram REPL with R you need to install [Ark](https://github.com/posit-dev/ark), an R Kernel for Jupyter applications.
You can down the latest version from the [Ark GitHub Releases](https://github.com/posit-dev/ark/releases) and then extract the `ark` binary to a directory in your `PATH`.

For example to install the latest non-debug build:

```sh
# macOS
cd /tmp
curl -L -o ark-latest-darwin.zip \
    $(curl -s "https://api.github.com/repos/posit-dev/ark/releases/latest" | \
    jq -r '.assets[] | select(.name | contains("darwin-universal") and (contains("debug") | not)) | .browser_download_url')
unzip ark-latest-darwin.zip ark
sudo mv /tmp/ark /usr/local/bin/
```

```sh
# Linux X86_64
cd /tmp
curl -L -o ark-latest-linux.zip \
    $(curl -s "https://api.github.com/repos/posit-dev/ark/releases/latest" \
        | jq -r '.assets[] | select(.name | contains("linux-x64") and (contains("debug") | not)) | .browser_download_url'
    )
unzip ark-latest-linux.zip ark
sudo mv /tmp/ark /usr/local/bin/
```

-->
