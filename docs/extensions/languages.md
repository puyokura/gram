# 言語拡張機能

Gram の言語サポートは、いくつかの要素で構成されます:

- 言語のメタデータと設定
- 文法
- クエリ
- 言語サーバー

## 言語のメタデータ

Gram がサポートする各言語は、拡張機能の `languages` フォルダー内のサブフォルダーで定義する必要があります。

このサブフォルダーには、次の構造の `config.toml` ファイルを含める必要があります:

```toml
name = "My Language"
grammar = "my-language"
path_suffixes = ["myl"]
line_comments = ["# "]
```

- `name`（必須）は、言語選択のドロップダウンに表示される人間が読める名前です。
- `grammar`（必須）は文法の名前です。文法は別途登録され、後述します。
- `path_suffixes` は、この言語に関連付けるファイルサフィックスの配列です。設定の `file_types` と異なり、glob パターンはサポートしません。
- `line_comments` は、その言語の行コメントを識別するために使う文字列の配列です。これは `editor::ToggleComments` キーバインド（{#kb editor::ToggleComments}）で、コードの行のコメントを切り替えるために使われます。
- `tab_size` は、この言語で使うインデント/タブのサイズを定義します（既定値は `4`）。
- `hard_tabs` は、タブ（`true`）とスペース（`false`、既定）のどちらでインデントするかを指定します。
- `first_line_pattern` は正規表現で、上記の `path_suffixes` や設定の `file_types` に加えて、この言語を使うべきファイルを特定するために使用できます。たとえば Gram は、スクリプトの1行目の[シバン行](https://github.com/zed-industries/zed/blob/main/crates/languages/src/bash/config.toml)にマッチさせることで、シェルスクリプトを識別します。
- `debuggers` は、その言語のデバッガーを識別するために使う文字列の配列です。デバッガーの `New Process Modal` を起動すると、Gram はこの配列のエントリー順に利用可能なデバッガーを並べます。

<!--
TBD: Document `language_name/config.toml` keys

- autoclose_before
- brackets (start, end, close, newline, not_in: ["comment", "string"])
- word_characters
- prettier_parser_name
- opt_into_language_servers
- code_fence_block_name
- scope_opt_in_language_servers
- increase_indent_pattern, decrease_indent_pattern
- collapsed_placeholder
- auto_indent_on_paste, auto_indent_using_last_non_empty_line
- overrides: `[overrides.element]`, `[overrides.string]`
-->

## 文法

Gram は、組み込みの言語固有機能を提供するために [Tree-sitter](https://tree-sitter.github.io) パーサーライブラリを使っています。多くの言語向けの文法が利用可能で、[自分で文法を開発する](https://tree-sitter.github.io/tree-sitter/creating-parsers#writing-the-grammar)こともできます。Gram の機能は、Tree-sitter クエリを使った構文木へのパターンマッチングで次々と実装されています。前述のとおり、拡張機能で定義する各言語は、パースに使用する Tree-sitter 文法の名前を指定する必要があります。これらの文法は、拡張機能の `extension.toml` ファイルに次のように別途登録します:

```toml
[grammars.gleam]
repository = "https://github.com/gleam-lang/tree-sitter-gleam"
rev = "58b7cac8fc14c92b0677c542610d8738c373fa81"
```

`repository` フィールドには Tree-sitter 文法の読み込み元リポジトリを指定する必要があり、`rev` フィールドには使用する Git リビジョン（Git コミットの SHA など）を含める必要があります。拡張機能をローカルで開発し、ローカルファイルシステムから文法を読み込みたい場合は、`repository` に `file://` URL を使用できます。拡張機能は、複数の tree-sitter リポジトリを参照することで複数の文法を提供できます。

## Tree-sitter クエリ

Gram は、[Tree-sitter](https://tree-sitter.github.io) クエリ言語が生成する構文木を使って、いくつかの機能を実装しています:

- シンタックスハイライト
- 括弧のマッチング
- コードのアウトライン/構造
- 自動インデント
- コードインジェクション
- 構文のオーバーライド
- テキストの伏字（レダクション）
- 実行可能コードの検出
- クラスや関数などの選択

次の各セクションでは、[Tree-sitter クエリ](https://tree-sitter.github.io/tree-sitter/using-parsers#query-syntax)によってこれらの機能が Gram でどう実現されるかを、[JSON 構文](https://www.json.org/json-en.html)を例にとって詳しく説明します。

### シンタックスハイライト

Tree-sitter では、`highlights.scm` ファイルが特定の構文に対するシンタックスハイライトのルールを定義します。

JSON 用の `highlights.scm` の例を次に示します:

```scheme
(string) @string

(pair
  key: (string) @property.json_key)

(number) @number
```

このクエリは、文字列、オブジェクトのキー、数値をハイライト対象としてマークします。テーマがサポートするキャプチャの完全な一覧は次のとおりです:

| キャプチャ               | 説明                                   |
| ------------------------ | -------------------------------------- |
| @attribute               | 属性をキャプチャする                   |
| @boolean                 | 真偽値をキャプチャする                 |
| @comment                 | コメントをキャプチャする               |
| @comment.doc             | ドキュメントコメントをキャプチャする   |
| @constant                | 定数をキャプチャする                   |
| @constructor             | コンストラクターをキャプチャする       |
| @embedded                | 埋め込みコンテンツをキャプチャする     |
| @emphasis                | 強調テキストをキャプチャする           |
| @emphasis.strong         | 強く強調したテキストをキャプチャする   |
| @enum                    | 列挙型をキャプチャする                 |
| @function                | 関数をキャプチャする                   |
| @hint                    | ヒントをキャプチャする                 |
| @keyword                 | キーワードをキャプチャする             |
| @label                   | ラベルをキャプチャする                 |
| @link_text               | リンクテキストをキャプチャする         |
| @link_uri                | リンク URI をキャプチャする            |
| @number                  | 数値をキャプチャする                   |
| @operator                | 演算子をキャプチャする                 |
| @predictive              | 予測テキストをキャプチャする           |
| @preproc                 | プリプロセッサ指令をキャプチャする     |
| @primary                 | 主要な要素をキャプチャする             |
| @property                | プロパティをキャプチャする             |
| @punctuation             | 句読点をキャプチャする                 |
| @punctuation.bracket     | 括弧をキャプチャする                   |
| @punctuation.delimiter   | 区切り文字をキャプチャする             |
| @punctuation.list_marker | リストのマーカーをキャプチャする       |
| @punctuation.special     | 特殊な句読点をキャプチャする           |
| @string                  | 文字列リテラルをキャプチャする         |
| @string.escape           | 文字列内のエスケープ文字をキャプチャする |
| @string.regex            | 正規表現をキャプチャする               |
| @string.special          | 特殊な文字列をキャプチャする           |
| @string.special.symbol   | 特殊なシンボルをキャプチャする         |
| @tag                     | タグをキャプチャする                   |
| @tag.doctype             | doctype（HTML などでの）をキャプチャする |
| @text.literal            | リテラルテキストをキャプチャする       |
| @title                   | タイトルをキャプチャする               |
| @type                    | 型をキャプチャする                     |
| @variable                | 変数をキャプチャする                   |
| @variable.special        | 特殊な変数をキャプチャする             |
| @variant                 | バリアントをキャプチャする             |

### 括弧のマッチング

`brackets.scm` ファイルは対応する括弧を定義します。

JSON 用の `brackets.scm` ファイルの例を次に示します:

```scheme
("[" @open "]" @close)
("{" @open "}" @close)
("\"" @open "\"" @close)
```

このクエリは、開き括弧と閉じ括弧、波括弧、引用符を識別します。

| キャプチャ | 説明                                       |
| ---------- | ------------------------------------------ |
| @open      | 開き括弧、波括弧、引用符をキャプチャする   |
| @close     | 閉じ括弧、波括弧、引用符をキャプチャする   |

Gram はこれらを使って対応する括弧をハイライトします。各括弧ペアを異なる色で描画し（「レインボーブラケット」）、カーソルがその括弧ペアの内側にある場合は括弧をハイライトします。

レインボーブラケットの色付けを無効にするには、対応する `brackets.scm` のエントリーに次のように追加します:

```scheme
(("\"" @open "\"" @close) (#set! rainbow.exclude))
```

### コードのアウトライン/構造

`outline.scm` ファイルはコードのアウトラインの構造を定義します。

JSON 用の `outline.scm` ファイルの例を次に示します:

```scheme
(pair
  key: (string (string_content) @name)) @item
```

このクエリは、アウトライン構造用にオブジェクトのキーをキャプチャします。

| キャプチャ     | 説明                                                                                 |
| -------------- | ------------------------------------------------------------------------------------ |
| @name          | オブジェクトのキーの内容をキャプチャする                                             |
| @item          | キーと値のペア全体をキャプチャする                                                   |
| @context       | アウトライン項目のコンテキストを提供する要素をキャプチャする                         |
| @context.extra | アウトライン項目の追加のコンテキスト情報をキャプチャする                             |
| @annotation    | アウトライン項目に注釈を付けるノード（ドキュメントコメント、属性、デコレーター）をキャプチャする[^1] |

[^1]: これらの注釈は、Assistant がコード変更の手順を生成するときに使われます。

### 自動インデント

`indents.scm` ファイルはインデントのルールを定義します。

JSON 用の `indents.scm` ファイルの例を次に示します:

```scheme
(array "]" @end) @indent
(object "}" @end) @indent
```

このクエリは、インデントのために配列とオブジェクトの終端をマークします。

| キャプチャ | 説明                                       |
| ---------- | ------------------------------------------ |
| @end       | 閉じ括弧と波括弧をキャプチャする           |
| @indent    | インデントのために配列とオブジェクト全体をキャプチャする |

### コードインジェクション

`injections.scm` ファイルは、Markdown 内のコードブロックや Python 文字列内の SQL クエリのように、ある言語を別の言語内に埋め込むためのルールを定義します。

Markdown 用の `injections.scm` ファイルの例を次に示します:

```scheme
(fenced_code_block
  (info_string
    (language) @injection.language)
  (code_fence_content) @injection.content)

((inline) @content
 (#set! injection.language "markdown-inline"))
```

このクエリはフェンス付きコードブロックを識別し、info 文字列で指定された言語とブロック内のコンテンツをキャプチャします。また、インラインコンテンツをキャプチャし、その言語を "markdown-inline" に設定します。

| キャプチャ          | 説明                                                 |
| ------------------- | ---------------------------------------------------- |
| @injection.language | コードブロックの言語識別子をキャプチャする           |
| @injection.content  | 別の言語として扱うコンテンツをキャプチャする         |

なお、JSON は言語インジェクションをサポートしていないため、ここでは例として JSON を使用できませんでした。

### 構文のオーバーライド

`overrides.scm` ファイルは、特定の言語構文の内部で特定のエディター設定をオーバーライドするために使える構文上の _スコープ_ を定義します。

たとえば、どの非英字文字を単語の一部とみなすかを制御する `word_characters` という言語固有の設定があります。これは変数をダブルクリックで選択するときなどに使われます。JavaScript では、「$」と「#」が単語文字とみなされます。

オートコンプリート候補をトリガーする文字を制御する `completion_query_characters` という言語固有の設定もあります。JavaScript では、カーソルが _文字列_ 内にあるとき、「-」を補完クエリ文字とみなすべきです。これを実現するため、JavaScript の `overrides.scm` ファイルには次のパターンが含まれています:

```scheme
[
  (string)
  (template_string)
] @string
```

そして JavaScript の `config.toml` にはこの設定が含まれています:

```toml
word_characters = ["#", "$"]

[overrides.string]
completion_query_characters = ["-"]
```

特定のスコープ内で特定の自動閉じ括弧を無効にすることもできます。たとえば、文字列内で `'` を自動で閉じないようにするには、JavaScript の `config.toml` に次のように記述します:

```toml
brackets = [
  { start = "'", end = "'", close = true, newline = false, not_in = ["string"] },
  # other pairs...
]
```

#### 範囲の包含

既定では、`overrides.scm` で定義された範囲は _排他的_ です。したがって上の例では、カーソルが文字列を区切る引用符の _外側_ にある場合、`string` スコープは有効になりません。範囲を _包含的_ にしたい場合があります。その場合は、クエリのキャプチャ名に `.inclusive` サフィックスを付けることで実現できます。

たとえば JavaScript では、コメント内でシングルクォートの自動閉じも無効にしています。また、コメントスコープは行コメント後の改行まで拡張される必要があります。これを実現するため、JavaScript の `overrides.scm` には次のパターンが含まれています:

```scheme
(comment) @comment.inclusive
```

### テキストオブジェクト

`textobjects.scm` ファイルは、テキストオブジェクト単位でナビゲートするためのルールを定義します。これは Gram v0.165 で追加され、現在は Vim モードでのみ使用されています。

Vim はファイル内のナビゲーションに2段階の粒度を提供します。`[]` などによるセクション単位の移動と、`]m` などによるメソッド単位の移動です。関数やクラスをサポートしない言語でも、似た概念を定義すればうまく機能します。たとえば CSS は、ルールセットをメソッド、メディアクエリをクラスとして定義します。

クロージャを持つ言語では、通常、クロージャは Gram の関数として数えるべきではありません。ただし JavaScript のような言語は、クロージャとトップレベル関数宣言を構文的に区別しないため、これはベストエフォートの対応です。

C のような宣言を持つ言語では、`@class.around` または `@function.around` にマッチするクエリを提供してください。`if` と `ic` テキストオブジェクトは、inside がない場合にこれらを既定で使います。

textobjects.scm に何を書けばよいか分からない場合は、[nvim-treesitter-textobjects](https://github.com/nvim-treesitter/nvim-treesitter-textobjects) と [Helix エディター](https://github.com/helix-editor/helix) に多くの言語向けのクエリがあります。Gram の[組み込み言語](https://github.com/zed-industries/zed/tree/main/crates/languages/src)を参照して、それらの適応方法を確認できます。

| キャプチャ       | 説明                                                                    | Vim モード                                       |
| ---------------- | ----------------------------------------------------------------------- | ------------------------------------------------ |
| @function.around | 関数定義全体、またはそれに相当するファイルの小さな範囲                  | `[m`、`]m`、`[M`、`]M` の移動。`af` テキストオブジェクト |
| @function.inside | 関数本体（波括弧内の内容）                                              | `if` テキストオブジェクト                        |
| @class.around    | クラス定義全体、またはそれに相当するファイルの大きな範囲                | `[[`、`]]`、`[]`、`][` の移動。`ac` テキストオブジェクト |
| @class.inside    | クラス定義の内容                                                        | `ic` テキストオブジェクト                        |
| @comment.around  | コメント全体（隣接するすべての行コメント、またはブロックコメント）      | `gc` テキストオブジェクト                        |
| @comment.inside  | コメントの内容                                                          | `igc` テキストオブジェクト（サポートされることはまれ） |

例:

```scheme
; include only the content of the method in the function
(method_definition
    body: (_
        "{"
        (_)* @function.inside
        "}")) @function.around

; match function.around for declarations with no body
(function_signature_item) @function.around

; join all adjacent comments into one
(comment)+ @comment.around
```

### 実行可能コードの検出

`runnables.scm` ファイルは、実行可能なコードを検出するためのルールを定義します。

JSON 用の `runnables.scm` ファイルの例を次に示します:

```scheme
(
    (document
        (object
            (pair
                key: (string
                    (string_content) @_name
                    (#eq? @_name "scripts")
                )
                value: (object
                    (pair
                        key: (string (string_content) @run @script)
                    )
                )
            )
        )
    )
    (#set! tag package-script)
    (#set! tag composer-script)
)
```

このクエリは、package.json と composer.json ファイル内の実行可能なスクリプトを検出します。

`@run` キャプチャは、エディター内で実行ボタンを表示する位置を指定します。アンダースコアが付いたものを除く他のキャプチャは、コードを実行するときに `GRAM_CUSTOM_$(capture_name)` というプレフィックスを持つ環境変数として公開されます。

| キャプチャ | 説明                                                 |
| ---------- | ---------------------------------------------------- |
| @\_name    | "scripts" キーをキャプチャする                       |
| @run       | スクリプト名をキャプチャする                         |
| @script    | スクリプト名もキャプチャする（別の目的のため）       |

<!--
TBD: `#set! tag`
-->

## 言語サーバー

Gram は、高度な言語サポートを提供するために [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) を使っています。

拡張機能は、任意の数の言語サーバーを提供できます。拡張機能から言語サーバーを提供するには、`extension.toml` に、言語サーバーの名前と適用対象の言語を記したエントリーを追加します。`languages` の一覧のエントリーは、その言語の `config.toml` ファイルの `name` フィールドと一致している必要があります:

```toml
[language_servers.my-language-server]
name = "My Language LSP"
languages = ["My Language"]
```

次に、拡張機能の Rust コードで `language_server_command` メソッドを実装します:

```rust
impl gram::Extension for MyExtension {
    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &gram::Worktree,
    ) -> Result<gram::Command> {
        Ok(gram::Command {
            command: get_path_to_language_server_executable()?,
            args: get_args_for_language_server()?,
            env: get_env_for_language_server()?,
        })
    }
}
```

言語サーバーの扱いは、`Extension` トレイトのいくつかの任意メソッドでカスタマイズできます。たとえば `label_for_completion` メソッドを使って補完の表示スタイルを制御できます。メソッドの完全な一覧は [Gram 拡張 API の API ドキュメント](https://docs.rs/zed_extension_api)を参照してください。

### 複数言語のサポート

言語サーバーが追加の言語をサポートしている場合は、`language_ids` を使って Gram の `languages` を目的の [LSP 固有の `languageId`](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentItem) 識別子にマッピングできます:

```toml
[language-servers.my-language-server]
name = "Whatever LSP"
languages = ["JavaScript", "HTML", "CSS"]

[language-servers.my-language-server.language_ids]
"JavaScript" = "javascript"
"TSX" = "typescriptreact"
"HTML" = "html"
"CSS" = "css"
```
