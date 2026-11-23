# Go

Go のサポートは Gram にネイティブで組み込まれています。

- Tree-sitter:
  [tree-sitter/tree-sitter-go](https://github.com/tree-sitter/tree-sitter-go)
- 言語サーバー:
  [golang/tools/tree/master/gopls](https://github.com/golang/tools/tree/master/gopls)
- デバッグアダプター: [delve](https://github.com/go-delve/delve)

## セットアップ

gopls は Homebrew や各 Linux ディストリビューションのパッケージマネージャーではなく、
Go のパッケージマネージャーでインストールすることをおすすめします。

1. パッケージマネージャーでインストールした gopls をすべてアンインストールします:

```sh
# macOS の Homebrew
brew remove gopls
# Ubuntu
sudo apt-get remove gopls
sudo snap remove gopls
# Arch
sudo pacman -R gopls
```

2. Go のモジュールツールを使って `gopls` を最新版にインストール/更新します:

```sh
go install golang.org/x/tools/gopls@latest
```

3. `gopls` が PATH に入っていることを確認します:

```sh
which gopls
gopls version
```

`gopls` が見つからない場合は、`export PATH="$PATH:$HOME/go/bin"` を
`.zshrc` / `.bash_profile` に追加する必要があるでしょう。

## インラインヒント

Gram は、設定でインラインヒントが有効になっているときに言語サーバーがインラインヒントを
返すよう、次の初期化オプションを設定します:

```json
"hints": {
    "assignVariableTypes": true,
    "compositeLiteralFields": true,
    "compositeLiteralTypes": true,
    "constantValues": true,
    "functionTypeParameters": true,
    "parameterNames": true,
    "rangeVariableTypes": true
}
```

これらの設定を上書きするには、次を使用します:

```json
"lsp": {
    "gopls": {
        "initialization_options": {
            "hints": {
                // ....
            }
        }
    }
}
```

詳しくは
[gopls の inlayHints ドキュメント](https://github.com/golang/tools/blob/master/gopls/doc/inlayHints.md)
を参照してください。

## デバッグ

Gram は Delve を使って、Go のテストとエントリーポイント（`func main`）を設定なしで
デバッグできます。{#action debugger::Start}（{#kb debugger::Start}）を実行すると、
あらかじめ構成されたデバッグタスクがコンテキストに応じて一覧表示されます。

さらに細かく制御するには、`.gram/debug.jsonc` にデバッグ構成を追加します。例は以下に
示します。

- [Delve の構成ドキュメント](https://github.com/go-delve/delve/blob/master/Documentation/api/dap/README.md#launch-and-attach-configurations)

### Go パッケージのデバッグ

特定のパッケージをデバッグするには、Delve のモードを "debug" に設定します。この場合、
"program" にはパッケージ名を指定します。

```json [debug]
[
  {
    "label": "Go (Delve)",
    "adapter": "Delve",
    "program": "$GRAM_FILE",
    "request": "launch",
    "mode": "debug"
  },
  {
    "label": "Run server",
    "adapter": "Delve",
    "request": "launch",
    "mode": "debug",
    // Delve では、program にパッケージ名を指定できます
    "program": "./cmd/server"
    // "args": [],
    // "buildFlags": [],
  }
]
```

### Go テストのデバッグ

パッケージのテストをデバッグするには、Delve のモードを "test" に設定します。"program"
は引き続きパッケージ名で、"buildFlags" を使ってタグを設定したり、"args" でテスト
バイナリに引数を渡したりできます（この方法の詳細は `go help testflags` を参照して
ください）。

```json [debug]
[
  {
    "label": "Run integration tests",
    "adapter": "Delve",
    "request": "launch",
    "mode": "test",
    "program": ".",
    "buildFlags": ["-tags", "integration"]
    // カーソルがあるテストだけに絞り込むには:
    // "args": ["-test.run", "$GRAM_SYMBOL"]
  }
]
```

### ビルドとデバッグを分けて行う

アプリケーションを特定のコマンドでビルドする必要がある場合は、Delve の "exec" モードを
使えます。この場合 "program" は実行ファイルを指し、"build" コマンドでその実行ファイルを
ビルドします。

```json [debug]
[
  {
    "label": "Debug Prebuilt Unit Tests",
    "adapter": "Delve",
    "request": "launch",
    "mode": "exec",
    "program": "${GRAM_WORKTREE_ROOT}/__debug_unit",
    "args": ["-test.v", "-test.run=${GRAM_SYMBOL}"],
    "build": {
      "command": "go",
      "args": [
        "test",
        "-c",
        "-tags",
        "unit",
        "-gcflags\"all=-N -l\"",
        "-o",
        "__debug_unit",
        "./pkg/..."
      ]
    }
  }
]
```

### 既存の Delve インスタンスへの接続

自分のマシンで動いているとは限らない既存の Delve インスタンスに接続したい場合があります。
その場合は `tcp_arguments` を使って、Gram から Delve への接続を設定できます。

```json [debug]
[
  {
    "adapter": "Delve",
    "label": "Connect to a running Delve instance",
    "program": "/Users/gram/Projects/language_repositories/golang/hello/hello",
    "cwd": "/Users/gram/Projects/language_repositories/golang/hello",
    "args": [],
    "env": {},
    "request": "launch",
    "mode": "exec",
    "stopOnEntry": false,
    "tcp_connection": { "host": "127.0.0.1", "port": 53412 }
  }
]
```

この場合、Gram は既存のインスタンスを使うため、新しい Delve インスタンスを起動しません。
その結果、Gram には*ターミナルが表示されません*。デバッグ対象の stdin/stdout は Delve が
扱うため、Delve インスタンスと直接やり取りする必要があります。

## Go Mod

- Tree-sitter:
  [camdencheek/tree-sitter-go-mod](https://github.com/camdencheek/tree-sitter-go-mod)
- 言語サーバー: 該当なし

## Go Sum

- Tree-sitter:
  [amaanq/tree-sitter-go-sum](https://github.com/amaanq/tree-sitter-go-sum)
- 言語サーバー: 該当なし

## Go Work

- Tree-sitter: [tree-sitter-go-work](https://github.com/d1y/tree-sitter-go-work)
- 言語サーバー: 該当なし
