# Elixir

Elixir のサポートは
[Elixir 拡張機能](https://github.com/zed-extensions/elixir) から利用できます。

- Tree-sitter:
  - [elixir-lang/tree-sitter-elixir](https://github.com/elixir-lang/tree-sitter-elixir)
  - [phoenixframework/tree-sitter-heex](https://github.com/phoenixframework/tree-sitter-heex)
- 言語サーバー:
  - [elixir-lang/expert](https://github.com/elixir-lang/expert)
  - [elixir-lsp/elixir-ls](https://github.com/elixir-lsp/elixir-ls)
  - [elixir-tools/next-ls](https://github.com/elixir-tools/next-ls)
  - [lexical-lsp/lexical](https://github.com/lexical-lsp/lexical)

## 言語サーバーの選択

Elixir 拡張機能は `expert`、`elixir-ls`、`next-ls`、`lexical` の言語サーバーを
サポートしています。

既定では `elixir-ls` が有効です。

### Expert

`expert` に切り替えるには、`settings.jsonc` に次を追加します:

```json
  "languages": {
    "Elixir": {
      "language_servers": ["expert", "!elixir-ls", "!next-ls", "!lexical", "..."]
    },
    "HEEX": {
      "language_servers": ["expert", "!elixir-ls", "!next-ls", "!lexical", "..."]
    }
  }
```

### Next LS

`next-ls` に切り替えるには、`settings.jsonc` に次を追加します:

```json
  "languages": {
    "Elixir": {
      "language_servers": ["next-ls", "!expert", "!elixir-ls", "!lexical", "..."]
    },
    "HEEX": {
      "language_servers": ["next-ls", "!expert", "!elixir-ls", "!lexical", "..."]
    }
  }
```

### Lexical

`lexical` に切り替えるには、`settings.jsonc` に次を追加します:

```json
  "languages": {
    "Elixir": {
      "language_servers": ["lexical", "!expert", "!elixir-ls", "!next-ls", "..."]
    },
    "HEEX": {
      "language_servers": ["lexical", "!expert", "!elixir-ls", "!next-ls", "..."]
    }
  }
```

## `elixir-ls` のセットアップ

1. `elixir` をインストール:

```sh
brew install elixir
```

2. `elixir-ls` をインストール:

```sh
brew install elixir-ls
```

3. Gram を再起動

> `elixir-ls` が elixir プロジェクトで動作していない場合は、コマンドパレットの
> `gram: open log` アクションでエラーログを確認してください。次のメッセージを含む
> エラーが見つかった場合:
> `invalid LSP message header "Shall I install Hex? (if running non-interactively, use \"mix local.hex --force\") [Yn]`,
> [`Hex`](https://hex.pm) をインストールする必要があるかもしれません。コマンドラインから
> `elixir-ls` を実行し、`Hex` のインストールを求めるプロンプトを承認してください。

### Mix でのフォーマット

[Mix](https://hexdocs.pm/mix/Mix.html) でコードをフォーマットしたい場合は、
`settings.jsonc` ファイルで次のスニペットを使って外部フォーマッターとして設定します。
フォーマットはファイル保存時に実行されます。

```json
{
  "languages": {
    "Elixir": {
      "format_on_save": "on",
      "formatter": {
        "external": {
          "command": "mix",
          "arguments": ["format", "--stdin-filename", "{buffer_path}", "-"],
        },
      },
    },
  },
}
```

### 追加のワークスペース構成オプション

追加の elixir-ls ワークスペース構成オプションは、`settings.jsonc` の lsp 設定で
渡せます。

次の例は dialyzer を無効にします:

```json
  "lsp": {
    "elixir-ls": {
      "settings": {
        "dialyzerEnabled": false
      }
    }
  }
```

より多くのオプションは
[ElixirLS の設定項目](https://github.com/elixir-lsp/elixir-ls#elixirls-configuration-settings)
を参照してください。

### HEEx

Gram は HEEx テンプレートもサポートしています。HEEx は
[EEx](https://hexdocs.pm/eex/1.12.3/EEx.html)（Embedded Elixir）と HTML を
組み合わせたもので、Phoenix LiveView アプリケーションで使われます。

- Tree-sitter:
  [phoenixframework/tree-sitter-heex](https://github.com/phoenixframework/tree-sitter-heex)
