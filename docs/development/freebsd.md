# FreeBSD 向け Gram のビルド

FreeBSD は現在サポート対象のプラットフォームではないため、これは作業中のドキュメントです。

## リポジトリ

[Gram リポジトリ](https://codeberg.org/GramEditor/gram)をクローンします。

## 依存関係

- 必要なシステムパッケージと rustup をインストールします:

  ```sh
  script/freebsd
  ```

  必要であれば、
  [`script/freebsd`](https://github.com/zed-industries/zed/blob/main/script/freebsd)
  を参照して手順を手動で実行してもかまいません。

## ソースからのビルド

依存関係をインストールしたら、[Cargo](https://doc.rust-lang.org/cargo/)を使って Gram をビルドできます。

エディターのデバッグビルド:

```sh
cargo run
```

テストを実行する場合:

```sh
cargo test --workspace
```

リリースモードでは、主要なユーザーインターフェースは `cli` クレートです。開発中に実行するには次のコマンドを使います:

```sh
cargo run -p cli
```

## トラブルシューティング

### 依存関係が不安定な機能を使用しているという Cargo のエラー

`cargo clean` を実行してから `cargo build` を試してください。
