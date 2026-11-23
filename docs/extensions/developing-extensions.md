# 拡張機能の開発

## 拡張機能が提供できる機能

拡張機能は次の機能を Gram に提供できます:

- [言語](../extensions/languages.md)
- [デバッガー](../extensions/debugger-extensions.md)
- [テーマ](../extensions/themes.md)
- [アイコンテーマ](../extensions/icon-themes.md)

## 拡張機能の開発

Gram 用の拡張機能の開発を始める前に、必ず [rustup 経由で Rust をインストール](https://www.rust-lang.org/tools/install)してください。

注: [Homebrew](https://brew.sh) 経由で rustup をインストールした場合、コマンドが正しく動作するために `rustup-init` の実行が必要になることがあります。

拡張機能を開発する際は、インストールして Gram で使用できます:

拡張機能のページで `Install Local` ボタン（または {#action gram::InstallExtensionFromFolder} アクション）をクリックし、拡張機能を含むフォルダーを選択します。

トラブルシューティングが必要な場合は、追加の出力について Gram.log（{#action gram::OpenLog}）を確認できます。デバッグ出力が必要な場合は、コマンドラインからフラグ `gram --foreground` を付けてエディターを終了・再起動してください。より詳細な INFO レベルのログが表示されます。

## Gram 拡張機能のフォルダー構成

Gram の拡張機能は、`extension.toml` を含む Git リポジトリです。このファイルには、拡張機能に関する基本情報を含める必要があります:

```toml
id = "my-extension"
name = "My extension"
version = "0.0.1"
schema_version = 1
authors = ["Your Name <you@example.com>"]
description = "My cool extension"
repository = "https://github.com/your-name/my-gram-extension"
```

これに加えて、Gram の拡張機能に機能を追加するための任意のファイルとフォルダーがいくつかあります。すべての機能を提供する拡張機能のフォルダー構成の例は次のとおりです:

```
my-extension/
  extension.toml
  Cargo.toml
  src/
    lib.rs
  languages/
    my-language/
      config.toml
      highlights.scm
  themes/
    my-theme.json
```

## WebAssembly

拡張機能の手続き的な部分は Rust で書かれ、WebAssembly にコンパイルされます。カスタムコードを含む拡張機能を開発するには、次のような `Cargo.toml` を含めます:

```toml
[package]
name = "my-extension"
version = "0.0.1"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
zed_extension_api = "0.1.0"
```

crates.io で公開されている最新版の [`zed_extension_api`](https://crates.io/crates/zed_extension_api) を使用してください。サポートしたい Gram のバージョンと互換性があることを確認してください。

Rust クレートの `src/lib.rs` ファイルで、拡張機能用の構造体を定義し、`Extension` トレイトを実装する必要があります。また `register_extension!` マクロを使って拡張機能を登録します:

```rs
use zed_extension_api as zed;

struct MyExtension {
    // ... state
}

impl gram::Extension for MyExtension {
    // ...
}

gram::register_extension!(MyExtension);
```

> `stdout`/`stderr` は Gram プロセスに直接転送されます。拡張機能の `println!`/`dbg!` の出力を確認するには、ターミナルで `--foreground` フラグを付けて Gram を起動します。

## 拡張機能のライセンス

拡張機能のリポジトリにはライセンスを含めるべきです。次のライセンスを推奨します:

- [GNU GPLv3](https://www.gnu.org/licenses/gpl-3.0.en.html)
- [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0)
- [BSD 3-Clause](https://opensource.org/license/bsd-3-clause)
- [MIT](https://opensource.org/license/mit)

ライセンスファイルは拡張機能リポジトリのルートに置く必要があります。ファイル名は `LICENCE` または `LICENSE` を先頭に持つもの（大文字小文字を区別しません）です。

> 注: このライセンスは拡張機能のコード自体にのみ適用されます。言語サーバーやその他の外部依存関係など、拡張機能がダウンロードまたは連携する可能性のあるツールには適用されません。リポジトリに拡張機能のコードと他のプロジェクト（言語サーバーなど）の両方が含まれる場合、それらの他のプロジェクトを再ライセンスする必要はありません。
