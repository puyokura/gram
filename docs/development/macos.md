# macOS 向け Gram のビルド

## リポジトリ

[Gram リポジトリ](https://codeberg.org/GramEditor/gram)をクローンします。

```zsh
# Your apple ID (email)
export APPLE_ID=""
# App-specific password (create in account.apple.com)
export APPLE_PASSWORD_GRAM=""
# Apple Team ID (find it in XCode)
export APPLE_TEAM_ID=""
# Apple signing key: security find-identity -p codesigning
export APPLE_SIGNING_KEY=""
# Build, sign and notarise the app bundle
./script/bundle-mac
```

## 依存関係

- [rustup](https://www.rust-lang.org/tools/install) をインストールします

- [Xcode](https://apps.apple.com/us/app/xcode/id497799835?mt=12) を macOS App Store から、または [Apple Developer](https://developer.apple.com/download/all/) の Web サイトからインストールします。開発者アカウントが必要であることに注意してください。

> インストール後は必ず Xcode を起動し、既定の選択肢である macOS コンポーネントをインストールしてください。

- [Xcode コマンドラインツール](https://developer.apple.com/xcode/resources/)をインストールします

  ```zsh
  xcode-select --install
  ```

- Xcode コマンドラインツールが、新しくインストールした Xcode を参照していることを確認します:

  ```zsh
  sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer
  sudo xcodebuild -license accept
  ```

- `cmake` をインストールします（[依存クレート](https://docs.rs/wasmtime-c-api-impl/latest/wasmtime_c_api/)が必要とするもの）

  ```zsh
  brew install cmake
  ```

## ソースからの Gram のビルド

依存関係をインストールしたら、[Cargo](https://doc.rust-lang.org/cargo/)を使って Gram をビルドできます。

デバッグビルド:

```zsh
cargo run
```

リリースビルド:

```zsh
cargo run --release
```

テストを実行する場合:

```zsh
cargo test --workspace
```

## トラブルシューティング

### Metal シェーダーのコンパイルエラー

```zsh
error: failed to run custom build command for gpui v0.1.0 (/Users/path/to/gram)`**
xcrun: error: unable to find utility "metal", not a developer tool or in PATH
```

`sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer` を試してください。

macOS 26 を使用している場合は、`xcodebuild -downloadComponent MetalToolchain` を試してください。

### 依存関係が不安定な機能を使用しているという Cargo のエラー

`cargo clean` を実行してから `cargo build` を試してください。

### エラー: 'dispatch/dispatch.h' が見つからない

次のようなエラーが発生した場合:

```zsh
src/platform/mac/dispatch.h:1:10: fatal error: 'dispatch/dispatch.h' file not found

Caused by:
  process didn't exit successfully

  --- stdout
  cargo:rustc-link-lib=framework=System
  cargo:rerun-if-changed=src/platform/mac/dispatch.h
  cargo:rerun-if-env-changed=TARGET
  cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS_aarch64-apple-darwin
  cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS_aarch64_apple_darwin
  cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS
```

このファイルは Xcode の一部です。Xcode コマンドラインツールがインストールされていることを確認し、正しいパスを設定してください:

```zsh
xcode-select --install
sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer
```

さらに、`BINDGEN_EXTRA_CLANG_ARGS` 環境変数を設定します:

```zsh
export BINDGEN_EXTRA_CLANG_ARGS="--sysroot=$(xcrun --show-sdk-path)"
```

その後、プロジェクトをクリーンして再ビルドします:

```zsh
cargo clean
cargo run
```

### `Too many open files (os error 24)` でテストが失敗する

このエラーは OS のリソース制約に起因するようです。`cargo-nextest` をインストールしてテストを実行すれば解決するはずです。

- `cargo install cargo-nextest --locked`
- `cargo nextest run --workspace --no-fail-fast`

## ヒントとコツ

### 継続的な再ビルドの回避

Gram がルートクレートを継続的に再ビルドしていると感じる場合、開発用の Gram をコードベース自体を対象に起動していることが原因かもしれません。

これは、`cargo run` が多数の環境変数をエクスポートし、それが開発版 Gram で動作する `rust-analyzer` に取り込まれるために問題になります。これらの環境変数はさらに `cargo check` に渡され、依存している一部クレートのビルドキャッシュが無効になります。

`cargo run ~/path/to/other/project` を使って、チェックアウトした Gram コードベース以外でビルド済みバイナリを実行すれば、この問題を簡単に避けられます。

### 検証の高速化

Gram を頻繁にビルドしている場合、macOS が新しいビルドを毎回検証するため、イテレーションのサイクルに数秒余計にかかることがあります。

これを解決するには、次の手順を実行します:

- `sudo spctl developer-mode enable-terminal` を実行して、システム設定の「デベロッパーツール」パネルを有効にします。
- システム設定で「デベロッパーツール」を検索し、「デベロッパーツールの使用を許可するアプリケーション」の一覧にターミナル（例: iTerm や Ghostty）を追加します
- ターミナルを再起動します。

この情報を公開してくれた nextest の開発者に感謝します（[リンク](https://nexte.st/docs/installation/macos/#gatekeeper)）。
