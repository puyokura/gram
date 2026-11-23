# Rust

Rust のサポートは Gram にネイティブで組み込まれています。

- Tree-sitter:
  [tree-sitter/tree-sitter-rust](https://github.com/tree-sitter/tree-sitter-rust)
- 言語サーバー:
  [rust-lang/rust-analyzer](https://github.com/rust-lang/rust-analyzer)
- デバッグアダプター: [CodeLLDB](https://github.com/vadimcn/codelldb)（プライマリ）、
  [GDB](https://sourceware.org/gdb/)（セカンダリ、Apple シリコンでは利用不可）

<!--
TBD: Polish Rust Docs. Gram is a good rust editor, good Rust docs make it look like we care about Rust (we do!)
TBD: Users may not know what inlayHints, don't start there.
TBD: Provide explicit examples not just `....`
-->

## インレイヒント

Rust における `rust-analyzer` のインレイヒント設定を変更するには、次の設定を使用できます:

```json
{
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "inlayHints": {
          "maxLength": null,
          "lifetimeElisionHints": {
            "enable": "skip_trivial",
            "useParameterNames": true,
          },
          "closureReturnTypeHints": {
            "enable": "always",
          },
        },
      },
    },
  },
}
```

詳細については、Rust Analyzer マニュアルの
[インレイヒント](https://rust-analyzer.github.io/book/features.html#inlay-hints)
を参照してください。

## ターゲットディレクトリ

`rust-analyzer` のターゲットディレクトリは `initialization_options` で設定できます:

```json
{
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "rust": {
          "analyzerTargetDir": true,
        },
      },
    },
  },
}
```

`true` に設定すると、ターゲットディレクトリが `target/rust-analyzer` に設定されます。`true` の代わりに `"target/analyzer"` のような文字列を指定してカスタムディレクトリを設定することもできます。

## バイナリ

Gram が使用する `rust-analyzer` バイナリを設定できます。

既定では、Gram は `$PATH` 内で `rust-analyzer` を検索し、それを使用しようとします。そのバイナリが `rust-analyzer --help` を正常に実行できた場合、それが使用されます。

`rust-toolchain.toml` 内のプロジェクト固有設定を尊重して rustup で指定されたバイナリを使用するには、rustup が管理するシンボリックリンクを指定します:

```json
{
  "lsp": {
    "rust-analyzer": {
      "binary": {
        "path": "~/.cargo/bin/rust-analyzer",
      },
    },
  },
}
```

`rust-analyzer` の `allow_binary_download` 設定を有効にすると、エディターが自動的に最新バージョンをダウンロードしてインストールします。

代わりにプレリリース版の `rust-analyzer` をインストールしたい場合は、`settings.json` で `pre_release` を `true` に設定することで Gram に指示できます:

```json
{
  "lsp": {
    "rust-analyzer": {
      "fetch": {
        "pre_release": true,
      },
    },
  },
}
```

Gram による `rust-analyzer` バイナリの検索を無効にしたい場合は、`settings.json` で `ignore_system_version` を `true` に設定します:

```json
{
  "lsp": {
    "rust-analyzer": {
      "binary": {
        "ignore_system_version": true,
      },
    },
  },
}
```

カスタムの場所にあるバイナリを使用したい場合は、`path` と任意の `arguments` を指定できます:

```json
{
  "lsp": {
    "rust-analyzer": {
      "binary": {
        "path": "/Users/example/bin/rust-analyzer",
        "arguments": [],
      },
    },
  },
}
```

この `"path"` は絶対パスである必要があります。

## 代替ターゲット

現在のプラットフォーム以外のターゲット（例: macOS 上で実行している場合の Windows 向けなど）に対して rust-analyzer から診断を提供させたい場合は、次の Gram LSP 設定を使用できます:

```json
{
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "cargo": {
          "target": "x86_64-pc-windows-msvc",
        },
      },
    },
  },
}
```

`rustup` を使用している場合、次を実行することで利用可能なターゲットトリプル（`aarch64-apple-darwin`、`x86_64-unknown-linux-gnu` など）の一覧を確認できます:

```sh
rustup target list --installed
```

## LSP タスク

Gram は tree-sitter を使用したタスクを提供しますが、rust-analyzer には LSP を介してファイル関連のタスクを問い合わせる LSP 拡張メソッドがあります。これは既定で有効になっており、次のように設定できます:

```json
"lsp": {
  "rust-analyzer": {
    "enable_lsp_tasks": true,
  }
}
```

## 手動での Cargo 診断の取得

既定では、rust-analyzer は `checkOnSave: true` が有効になっており、バッファーを保存するたびに `cargo check --workspace --all-targets` コマンドがトリガーされます。`checkOnSave: false` で無効にした場合（上記のサーバー設定 JSON の例を参照）でも、Rust ファイル内で `editor: run/clear/cancel flycheck` コマンドを使用して手動で診断を取得し、cargo の診断を更新することができます。設定が有効になっている場合、プロジェクト診断エディターも `editor: run flycheck` コマンドで cargo の診断を更新します。

## その他のサーバー設定

<!--
TBD: Is it possible to specify RUSTFLAGS? https://github.com/zed-industries/zed/issues/14334
-->

Rust-analyzer [マニュアル](https://rust-analyzer.github.io/book/) では、rust-analyzer 言語サーバーのさまざまな機能と設定オプションについて説明されています。Gram の rust-analyzer は既定のパラメーターで実行されます。

### 大規模プロジェクトとパフォーマンス

大規模プロジェクトで大量のリソース消費を引き起こす可能性がある主な要因の 1 つは、以下の機能の組み合わせです:

```
rust-analyzer.checkOnSave (既定値: true)
    保存時に診断用の check コマンドを実行します。
```

```
rust-analyzer.check.workspace (既定値: true)
    cargo check に --workspace を渡すかどうか。false の場合、代わりに -p <package> が渡されます。
```

```
rust-analyzer.cargo.allTargets (既定値: true)
    cargo 呼び出しに --all-targets を渡します。
```

これは、Gram で保存するたびに `cargo check --workspace --all-targets` コマンドが実行され、プロジェクト全体（ワークスペース）、lib、doc、test、bin、bench、および
[その他のターゲット](https://doc.rust-lang.org/cargo/reference/cargo-targets.html)
がチェックされることを意味します。

小規模なプロジェクトでは問題なく動作しますが、大規模になるとうまくスケールしません。

代替策としては、Gram がすでに提供している `cargo check --workspace --all-targets` タスクとターミナル出力を Cmd/Ctrl+クリックしてエラーに移動する機能を利用し、保存時のチェック機能を制限または完全にオフにするという方法（[タスク](../tasks.md) を使用）があります。

保存時チェック機能は cargo check の出力に基づく診断の一部を返す役割を担っているため、これをオフにすると rust-analyzer は独自の
[診断](https://rust-analyzer.github.io/book/diagnostics.html)
のみに制限されます。

より細かな設定を行うには、マニュアルの `rust-analyzer.cargo.`、`rust-analyzer.check.`、`rust-analyzer.diagnostics.` 設定を参照してください。以下は Gram の `settings.json` 向けの snippet です（`lsp.rust-analyzer` セクションを編集して保存すると、言語サーバーは自動的に再起動します）:

```json
{
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        // rust-analyzer からより多くの（cargo なしの）診断を取得します。
        // これには誤検知が含まれる場合があります（名前で無効化可能）。
        "diagnostics": {
          "experimental": {
            "enable": true,
          },
        },
        // チェックを完全に無効化する場合
        // (以下の cargo および check 設定はすべて無視されます)
        "checkOnSave": false,
        // `lib` ターゲットのみをチェックする場合。
        "cargo": {
          "allTargets": false,
        },
        // cargo check に `--workspace` の代わりに `-p` を使用
        "check": {
          "workspace": false,
        },
      },
    },
  },
}
```

### 複数プロジェクトのワークスペース

Cargo ワークスペースの `[members]` に記載されていない同一フォルダー内の複数の Rust プロジェクトを rust-analyzer で解析したい場合は、ローカルプロジェクト設定の `linkedProjects` にそれらを列挙できます:

```json
{
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "linkedProjects": ["./path/to/a/Cargo.toml", "./path/to/b/Cargo.toml"],
      },
    },
  },
}
```

### スニペット

rust-analyzer からカスタム補完項目を取得し、スニペット本文に従ってコードを変換する方法があります:

```json
{
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "completion": {
          "snippets": {
            "custom": {
              "Arc::new": {
                "postfix": "arc",
                "body": ["Arc::new(${receiver})"],
                "requires": "std::sync::Arc",
                "scope": "expr",
              },
              "Some": {
                "postfix": "some",
                "body": ["Some(${receiver})"],
                "scope": "expr",
              },
              "Ok": {
                "postfix": "ok",
                "body": ["Ok(${receiver})"],
                "scope": "expr",
              },
              "Rc::new": {
                "postfix": "rc",
                "body": ["Rc::new(${receiver})"],
                "requires": "std::rc::Rc",
                "scope": "expr",
              },
              "Box::pin": {
                "postfix": "boxpin",
                "body": ["Box::pin(${receiver})"],
                "requires": "std::boxed::Box",
                "scope": "expr",
              },
              "vec!": {
                "postfix": "vec",
                "body": ["vec![${receiver}]"],
                "description": "vec![]",
                "scope": "expr",
              },
            },
          },
        },
      },
    },
  },
}
```

## デバッグ

Gram は `CodeLLDB` および `GDB` を使用した Rust バイナリとテストのデバッグを初期状態でサポートしています。{#action debugger::Start} ({#kb debugger::Start}) を実行して、これらの事前設定されたデバッグタスクのいずれかを起動します。

より詳細に制御するには、`.gram/debug.jsonc` にデバッグ設定を追加できます。以下の例を参照してください。

- [CodeLLDB 設定ドキュメント](https://github.com/vadimcn/codelldb/blob/master/MANUAL.md#starting-a-new-debug-session)
- [GDB 設定ドキュメント](https://sourceware.org/gdb/current/onlinedocs/gdb.html/Debugger-Adapter-Protocol.html)

### バイナリをビルドしてからデバッグ

```json [debug]
[
  {
    "label": "Build & Debug native binary",
    "build": {
      "command": "cargo",
      "args": ["build"]
    },
    "program": "$GRAM_WORKTREE_ROOT/target/debug/binary",
    // Rust 使用時に CodeLLDB（GDB ではなく）で sourceLanguages が必要
    "sourceLanguages": ["rust"],
    "request": "launch",
    "adapter": "CodeLLDB"
  }
]
```

### ビルドコマンドに基づいてデバッグターゲットを自動特定

ビルドコマンドとして `cargo build` または `cargo test` を使用すると、Gram は出力バイナリへのパスを推測できます。

```json [debug]
[
  {
    "label": "Build & Debug native binary",
    "adapter": "CodeLLDB",
    "build": {
      "command": "cargo",
      "args": ["build"]
    },
    // Rust 使用時に CodeLLDB（GDB ではなく）で sourceLanguages が必要
    "sourceLanguages": ["rust"]
  }
]
```
