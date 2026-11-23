# Ruby

Ruby のサポートは
[Ruby 拡張機能](https://github.com/zed-extensions/ruby) を通じて利用できます。

- Tree-sitter:
  - [tree-sitter-ruby](https://github.com/tree-sitter/tree-sitter-ruby)
  - [tree-sitter-embedded-template](https://github.com/tree-sitter/tree-sitter-embedded-template)
- 言語サーバー:
  - [ruby-lsp](https://github.com/Shopify/ruby-lsp)
  - [solargraph](https://github.com/castwide/solargraph)
  - [rubocop](https://github.com/rubocop/rubocop)
  - [Herb](https://herb-tools.dev)
- デバッグアダプター: [`rdbg`](https://github.com/ruby/debug)

Ruby 拡張機能は ERB ファイルのサポートも提供します。

## 言語サーバー

Ruby 向けには複数の言語サーバーが利用可能です。Gram は以下の 2 つをサポートしています:

- [solargraph](https://github.com/castwide/solargraph)
- [ruby-lsp](https://github.com/Shopify/ruby-lsp)

これらは両方とも自動補完、診断、コードアクションなどの重複する機能セットを備えており、どちらを使用するかはユーザーが決定できます。なお、両方を同時に使用することはできません。

これら 2 つの言語サーバーに加えて、Gram は以下もサポートしています:

- [rubocop](https://github.com/rubocop/rubocop): Ruby 向けの静的コードアナライザーおよびリンター。内部的には Gram によって言語サーバーとしても使用されますが、その機能は solargraph や ruby-lsp を補完するものです。
- [sorbet](https://sorbet.org/): カスタムの漸進的型システムを備えた Ruby 向けの静的型チェッカー。
- [steep](https://github.com/soutaro/steep): Ruby Signature (RBS) を活用する Ruby 向けの静的型チェッカー。
- [Herb](https://herb-tools.dev): ERB ファイル向けの言語サーバー。

言語サーバーを設定する際は、「dev: Open Language Server Logs」コマンドを使用して LSP ログウィンドウを開くと便利です。対応する言語インスタンスを選択して、ログに記録された情報を確認できます。

## 言語サーバーの設定

[Ruby 拡張機能](https://github.com/zed-extensions/ruby) は、`solargraph` と `ruby-lsp` の両方の言語サーバーサポートを提供します。

<a id="language-server-activation"></a>
### 言語サーバーのアクティブ化

サポートされているすべての Ruby 言語サーバー（`solargraph`、`ruby-lsp`、`rubocop`、`sorbet`、`steep`）について、Ruby 拡張機能は次のアクティブ化手順に従います:

1. プロジェクトの `Gemfile` 内に言語サーバーが見つかった場合、`bundle exec` 経由で使用されます。
2. `Gemfile` 内に見つからない場合、Ruby 拡張機能はシステムの `PATH` 内で実行可能ファイルを探します。
3. どちらの場所にも言語サーバーが見つからない場合、Ruby 拡張機能はグローバル gem として自動的にインストールします（注意: 現在の Ruby gemset にはインストールされません）。

設定で `use_bundler` を `false` に設定することで、ステップ 1 をスキップしてシステムの実行可能ファイルを強制的に使用できます:

```json
{
  "lsp": {
    "<SERVER_NAME>": {
      "settings": {
        "use_bundler": false,
      },
    },
  },
}
```

### `solargraph` の使用

Ruby 拡張機能では、既定で `solargraph` が有効になっています。

### `ruby-lsp` の使用

`ruby-lsp` に切り替えるには、`settings.json` に次を追加します:

```json
{
  "languages": {
    "Ruby": {
      "language_servers": ["ruby-lsp", "!solargraph", "!rubocop", "..."],
    },
    // *.html.erb ファイル向けに herb と ruby-lsp を有効化
    "HTML+ERB": {
      "language_servers": ["herb", "ruby-lsp", "..."],
    },
    // *.js.erb ファイル向けに ruby-lsp を有効化
    "JS+ERB": {
      "language_servers": ["ruby-lsp", "..."],
    },
    // *.yaml.erb ファイル向けに ruby-lsp を有効化
    "YAML+ERB": {
      "language_servers": ["ruby-lsp", "..."],
    },
  },
}
```

これにより、`solargraph` と `rubocop` が無効化され、`ruby-lsp` が有効化されます。

### `rubocop` の使用

Ruby 拡張機能は、違反の検出と自動修正のための `rubocop` 言語サーバーのサポートも提供します。

有効にするには、`settings.json` に次を追加します:

```json
{
  "languages": {
    "Ruby": {
      "language_servers": ["ruby-lsp", "rubocop", "!solargraph", "..."],
    },
  },
}
```

あるいは逆に、`settings.json` に次を追加することで、`ruby-lsp` を無効にして `solargraph` と `rubocop` を有効にすることもできます:

```json
{
  "languages": {
    "Ruby": {
      "language_servers": ["solargraph", "rubocop", "!ruby-lsp", "..."],
    },
  },
}
```

## `solargraph` のセットアップ

Solargraph では、フォーマットと診断が既定で無効になっています。Gram の `settings.json` に次を追加することで、これらを有効にするよう指示できます:

```json
{
  "lsp": {
    "solargraph": {
      "initialization_options": {
        "diagnostics": true,
        "formatting": true,
      },
    },
  },
}
```

### 設定

Solargraph はプロジェクトのルートにある `.solargraph.yml` というファイルから設定を読み込みます。このファイルの詳細については、
[Solargraph の設定ドキュメント](https://solargraph.org/guides/configuration)
を参照してください。

## `ruby-lsp` のセットアップ

`initialization_options` に Ruby LSP の設定を渡すことができます（例）:

```json
{
  "languages": {
    "Ruby": {
      "language_servers": ["ruby-lsp", "!solargraph", "..."],
    },
  },
  "lsp": {
    "ruby-lsp": {
      "initialization_options": {
        "enabledFeatures": {
          // "someFeature": false
        },
      },
    },
  },
}
```

LSP の `settings` や `initialization_options` はプロジェクト固有にすることもできます。たとえば、特定のプロジェクトで [standardrb/standard](https://github.com/standardrb/standard) をフォーマッターおよびリンターとして使用するには、プロジェクトリポジトリ内の `.gram/settings.jsonc` に次を追加します:

```json
{
  "lsp": {
    "ruby-lsp": {
      "initialization_options": {
        "formatter": "standard",
        "linters": ["standard"],
      },
    },
  },
}
```

## `rubocop` LSP のセットアップ

Rubocop では安全でない自動修正が既定で無効になっています。Gram の `settings.json` に次を追加することで、これを有効にするよう指示できます:

```json
{
  "languages": {
    "Ruby": {
      // プライマリ言語サーバーとして ruby-lsp、セカンダリとして rubocop を使用
      "language_servers": ["ruby-lsp", "rubocop", "!solargraph", "..."],
    },
  },
  "lsp": {
    "rubocop": {
      "initialization_options": {
        "safeAutocorrect": false,
      },
    },
    "ruby-lsp": {
      "initialization_options": {
        "enabledFeatures": {
          "diagnostics": false,
        },
      },
    },
  },
}
```

## Sorbet のセットアップ

[Sorbet](https://sorbet.org/) は言語サーバーを含む、人気のある Ruby 向け静的型チェッカーです。

Sorbet を有効にするには、`settings.json` の Ruby の `language_servers` リストに `"sorbet"` を追加します。Sorbet をプライマリ LSP として使用する場合、あるいは型チェックなどの特定機能のために別の LSP と併用する場合は、他の言語サーバーを無効化するとよいでしょう。

```json
{
  "languages": {
    "Ruby": {
      "language_servers": [
        "ruby-lsp",
        "sorbet",
        "!rubocop",
        "!solargraph",
        "...",
      ],
    },
  },
}
```

Sorbet のインストール、プロジェクトへのセットアップ、動作の設定に関するあらゆる事項については、
[Sorbet 公式ドキュメント](https://sorbet.org/docs/overview)
を参照してください。

## Steep のセットアップ

[Steep](https://github.com/soutaro/steep) は RBS ファイルを使用して型を定義する Ruby 向けの静的型チェッカーです。

Steep を有効にするには、`settings.json` の Ruby の `language_servers` リストに `"steep"` を追加します。希望する構成に応じて、順序を調整したり他の LSP を無効化したりする必要がある場合があります。

```json
{
  "languages": {
    "Ruby": {
      "language_servers": [
        "ruby-lsp",
        "steep",
        "!solargraph",
        "!rubocop",
        "...",
      ],
    },
  },
}
```

## Herb のセットアップ

`HTML+ERB` 言語では、既定で `Herb` が有効になっています。

## Ruby での Tailwind CSS 言語サーバーの使用

Ruby および ERB ファイルで
[Tailwind CSS 言語サーバー](https://github.com/tailwindlabs/tailwindcss-intellisense/tree/HEAD/packages/tailwindcss-language-server#readme)
を使用することができます。

これを行うには、`settings.json` に次を追加して、Ruby/ERB ファイル内で CSS クラスをどこで探すべきかを言語サーバーに伝える必要があります:

```json
{
  "languages": {
    "Ruby": {
      "language_servers": ["tailwindcss-language-server", "..."],
    },
  },
  "lsp": {
    "tailwindcss-language-server": {
      "settings": {
        "experimental": {
          "classRegex": ["\\bclass:\\s*['\"]([^'\"]*)['\"]"],
        },
      },
    },
  },
}
```

これらの設定により、ERB ファイル内の HTML 属性、および `class:` キーの後に続く Ruby/ERB 文字列内で Tailwind CSS クラスの補完が行われます。例:

```rb
# Ruby ファイル:
def method
  div(class: "pl-2 <completion here>") do
    p(class: "mt-2 <completion here>") { "Hello World" }
  end
end

# ERB ファイル:
<%= link_to "Hello", "/hello", class: "pl-2 <completion here>" %>
<a href="/hello" class="pl-2 <completion here>">Hello</a>
```

## テストの実行

Ruby プロジェクトでテストを実行するには、ローカルの `.gram/tasks.jsonc` 設定ファイルでカスタムタスクをセットアップできます。これらのタスクは、Minitest、RSpec、quickdraw、tldr などのさまざまなテストフレームワークと連携するように定義できます。以下は、エディター内からテストを実行するためのタスク設定例です。

### Rails での Minitest

```json [tasks]
[
  {
    "label": "test $GRAM_RELATIVE_FILE -n /$GRAM_CUSTOM_RUBY_TEST_NAME/",
    "command": "bin/rails",
    "args": [
      "test",
      "$GRAM_RELATIVE_FILE",
      "-n",
      "\"$GRAM_CUSTOM_RUBY_TEST_NAME\""
    ],
    "cwd": "$GRAM_WORKTREE_ROOT",
    "tags": ["ruby-test"]
  }
]
```

### Minitest

プレーンな Minitest は行番号によるテストの実行をサポートしておらず、名前による実行のみをサポートしているため、代わりに `$GRAM_CUSTOM_RUBY_TEST_NAME` を使用する必要があります:

```json [tasks]
[
  {
    "label": "-Itest $GRAM_RELATIVE_FILE -n /$GRAM_CUSTOM_RUBY_TEST_NAME/",
    "command": "bundle",
    "args": [
      "exec",
      "ruby",
      "-Itest",
      "$GRAM_RELATIVE_FILE",
      "-n",
      "\"$GRAM_CUSTOM_RUBY_TEST_NAME\""
    ],
    "cwd": "$GRAM_WORKTREE_ROOT",
    "tags": ["ruby-test"]
  }
]
```

### RSpec

```json [tasks]
[
  {
    "label": "test $GRAM_RELATIVE_FILE:$GRAM_ROW",
    "command": "bundle",
    "args": ["exec", "rspec", "\"$GRAM_RELATIVE_FILE:$GRAM_ROW\""],
    "cwd": "$GRAM_WORKTREE_ROOT",
    "tags": ["ruby-test"]
  }
]
```

`quickdraw` や `tldr` などの他のテストフレームワークでも同様のタスク構文を使用できます。

## デバッグ

Ruby 拡張機能は、Ruby コードをデバッグするためのデバッグアダプターを提供します。UI および `debug.json` における Gram のアダプター名は `rdbg` であり、内部的には [`debug`](https://github.com/ruby/debug) gem を使用しています。拡張機能は、言語サーバーと
[同じアクティブ化ロジック](#language-server-activation)
を使用します。

### 例

#### Ruby スクリプトのデバッグ

```json [debug]
[
  {
    "label": "Debug current file",
    "adapter": "rdbg",
    "request": "launch",
    "script": "$GRAM_FILE",
    "cwd": "$GRAM_WORKTREE_ROOT"
  }
]
```

#### Rails サーバーのデバッグ

```json [debug]
[
  {
    "label": "Debug Rails server",
    "adapter": "rdbg",
    "request": "launch",
    "command": "./bin/rails",
    "args": ["server"],
    "cwd": "$GRAM_WORKTREE_ROOT",
    "env": {
      "RUBY_DEBUG_OPEN": "true"
    }
  }
]
```

## フォーマッター

### `erb-formatter`

ERB テンプレートをフォーマットするには、`erb-formatter` フォーマッターを使用できます。このフォーマッターは
[`erb-formatter`](https://rubygems.org/gems/erb-formatter)
gem を使用して ERB テンプレートをフォーマットします。

```json
{
  "HTML+ERB": {
    "formatter": {
      "external": {
        "command": "erb-formatter",
        "arguments": ["--stdin-filename", "{buffer_path}"],
      },
    },
  },
}
```
