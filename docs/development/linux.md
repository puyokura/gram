# Linux 向け Gram のビルド

## リポジトリ

[Gram リポジトリ](https://codeberg.org/GramEditor/gram)をクローンします。

## 依存関係

- [rustup](https://www.rust-lang.org/tools/install) をインストールします

- 必要なシステムライブラリをインストールします:

  ```sh
  script/linux
  ```

  システムライブラリを手動でインストールしたい場合は、必要なパッケージの一覧を `script/linux` ファイルで確認できます。

### リンカー {#linker}

Linux では、Rust の既定のリンカーは [LLVM の `lld`](https://blog.rust-lang.org/2025/09/18/Rust-1.90.0/) です。代替のリンカー、とくに [Wild](https://github.com/davidlattimore/wild) と [Mold](https://github.com/rui314/mold) は、クリーンビルドとインクリメンタルビルドの時間を大幅に改善できます。

ローカル開発では、Mold より 5〜20% 高速なため Wild を推奨します。

これらのリンカーは `script/install-mold` と `script/install-wild` でインストールできます。

Wild を既定として使うには、次の行を `~/.cargo/config.toml` に追加します:

```toml
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=--ld-path=wild"]

[target.aarch64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=--ld-path=wild"]
```

Mold を既定として使うには:

```toml
[target.'cfg(target_os = "linux")']
rustflags = ["-C", "link-arg=-fuse-ld=mold"]
```

## ソースからのビルド

依存関係をインストールしたら、[Cargo](https://doc.rust-lang.org/cargo/)を使ってビルドできます。

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

## 開発用ビルドのインストール

ローカルでビルドしたものをマシンにインストールするには、次のコマンドを実行します:

```sh
./script/install.sh --build
```

これにより gram と cli がリリースモードでビルドされ、`~/.local/bin/gram` から利用できるようになり、`.desktop` ファイルが `~/.local/share` にインストールされます。

> **_注_**: 次のようなリンカーエラーが発生した場合:
>
> ```bash
> error: linking with `cc` failed: exit status: 1 ...
> = note: /usr/bin/ld: /tmp/rustcISMaod/libaws_lc_sys-79f08eb6d32e546e.rlib(f8e4fd781484bd36-bcm.o): in function `aws_lc_0_25_0_handle_cpu_env':
>           /aws-lc/crypto/fipsmodule/cpucap/cpu_intel.c:(.text.aws_lc_0_25_0_handle_cpu_env+0x63): undefined reference to `__isoc23_sscanf'
>           /usr/bin/ld: /tmp/rustcISMaod/libaws_lc_sys-79f08eb6d32e546e.rlib(f8e4fd781484bd36-bcm.o): in function `pkey_rsa_ctrl_str':
>           /aws-lc/crypto/fipsmodule/evp/p_rsa.c:741:(.text.pkey_rsa_ctrl_str+0x20d): undefined reference to `__isoc23_strtol'
>           /usr/bin/ld: /aws-lc/crypto/fipsmodule/evp/p_rsa.c:752:(.text.pkey_rsa_ctrl_str+0x258): undefined reference to `__isoc23_strtol'
>           collect2: error: ld returned 1 exit status
>   = note: some `extern` functions couldn't be found; some native libraries may need to be installed or have their path specified
>   = note: use the `-l` flag to specify native libraries to link
>   = note: use the `cargo:rustc-link-lib` directive to specify the native libraries to link with Cargo (see https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-lib)
> error: could not compile `remote_server` (bin "remote_server") due to 1 previous error
> ```
>
> **原因**: aws-lc-rs の既知のバグ（GCC 14 以降をサポートしていない）が原因です:
> [FIPS fails to build with GCC >= 14](https://github.com/aws/aws-lc-rs/issues/569)
> &
> [GCC-14 - build failure for FIPS module](https://github.com/aws/aws-lc/issues/2010)
>
> 詳細は
> [linux: Linker error for remote_server when using script/install-linux](https://github.com/zed-industries/zed/issues/24880)
> を参照してください。
>
> **回避策**: リモートサーバーのターゲットを次のように `x86_64-unknown-linux-gnu` に設定します
> `export REMOTE_SERVER_TARGET=x86_64-unknown-linux-gnu; script/install-linux`

## Wayland と X11

Gram は X11 と Wayland の両方をサポートしています。既定では、実行時に見つかったほうを使用します。Wayland 上で X11 モードで実行したい場合は、環境変数 `WAYLAND_DISPLAY=''` を使用します。

## Gram のパッケージング

Gram には2つの主要なバイナリがあります:

- `crates/cli` をビルドし、そのバイナリを `gram` という名前で `$PATH` から利用できるようにする必要があります。
- `crates/gram` をビルドし、`$PATH/to/cli/../../libexec/gram-editor` に配置する必要があります。たとえば cli を `~/.local/bin/gram` に置く場合は、gram を `~/.local/libexec/gram-editor` に置きます。一部の Linux ディストリビューション（とくに Arch）は `libexec` の使用を推奨していないため、代わりにこのバイナリを `$PATH/to/cli/../../lib/gram/gram-editor`（例: `~/.local/lib/gram/gram-editor`）に置くこともできます。
- `.desktop` ファイルを提供する場合は、テンプレートが `crates/gram/resources/gram.desktop.in` にあります。`envsubst` を使って必要な値を埋めてください。また、ファイルが[FreeDesktop 標準](https://github.com/zed-industries/zed/issues/12707#issuecomment-2168742761)に従うよう、ファイル名を `$APP_ID.desktop` に変更する必要があります。安定版ビルドには `app.liten.Gram` を、開発版ビルドには `app.liten.Gram-Dev` を使用し、ウィンドウのアプリケーション ID と `StartupWMClass` に合わせてください。`share/applications` 以下にインストールされるデスクトップエントリーはモード `644` を使用できます。
- アプリケーションのアイコンは `share/icons/hicolor/scalable/apps` に、シンボリックアイコンは `share/icons/hicolor/symbolic/apps` にインストールします。`Icon` はアイコン名のままにしておき、デスクトップが選択中のアイコンテーマを通じて解決できるようにしてください。
- 必要なライブラリがインストールされていることを確認する必要があります。現在の一覧は、システム上で[ビルド済みバイナリを検査する](https://github.com/zed-industries/zed/blob/935cf542aebf55122ce6ed1c91d0fe8711970c82/script/bundle-linux#L65-L67)ことで取得できます。
- 完全なビルドスクリプトの例は [script/bundle-linux](https://github.com/zed-industries/zed/blob/935cf542aebf55122ce6ed1c91d0fe8711970c82/script/bundle-linux) を参照してください。
- 環境変数 `GRAM_NO_BUNDLED_UNINSTALL` に「Use flatpak to uninstall」のような値を設定して、CLI のアンインストールコマンドがアンインストールスクリプトを実行しようとしないようにします。

### その他の注意点

Zed には、（私たちの見解では）かなり重大な問題があります:

- rustup/rbenv/pyenv などと同じように、ユーザーへの明示的な確認なしにバイナリを自動でダウンロードしてインストールします。彼らは「これが議論の的になることは理解している」と言いながらも、それを続けると主張しています。[こちらを参照](https://github.com/zed-industries/zed/issues/12589)。
- 明示的な許可や、何がインストールされるかを検査する手段なしに、[zed-industries/extensions](https://github.com/zed-industries/extensions) から拡張機能のバイナリをダウンロードしてインストールします。これらの拡張機能はさらに、明示的な許可や制御なしに追加のソフトウェアをインストールする可能性があります。詳細は[こちらを参照](https://github.com/zed-industries/zed/issues/12358)。
- サンドボックスとの相性が悪いです。[こちらを参照](https://github.com/zed-industries/zed/pull/12006#issuecomment-2130421220)

私たち（Gram）の意図は、これらの問題をすべて解決することです。

## ディストリビューションパッケージ

Gram プロジェクトには、`.deb`、`.rpm`、`AppImage` など、さまざまなターゲット向けのバイナリパッケージをビルドするスクリプトが含まれています。これらの共通ドライバーは `scripts/bundle-linux` スクリプトで、次のフラグ（など）を受け取ります:

- `--deb`（[`cargo-deb`](https://github.com/kornelski/cargo-deb) が必要）
- `--rpm`（[`cargo-generate-rpm`](https://github.com/cat-in-136/cargo-generate-rpm) が必要）
- `--appimage`（[`pkg2appimage`](https://github.com/AppImageCommunity/pkg2appimage) が必要）

いずれかを追加すると、対応するパッケージがビルドされます。`AppImage` は `.deb.` パッケージから作成されるため、`--appimage` フラグだけを指定しても `.deb` も自動的にビルドされることに注意してください。

これらのパッケージのビルドには、上記のとおりそれぞれ追加の依存関係が必要です。

## Flatpak

> Gram の現在の Flatpak 連携は、起動時にサンドボックスを抜け出します。Flatpak のサンドボックスに依存するワークフローは、期待どおりに動作しない可能性があります。

Flatpak パッケージをローカルでビルドしてインストールするには、次の手順に従います:

1. お使いのディストリビューション向けに Flatpak を[こちら](https://flathub.org/setup)の手順でインストールします。
2. `script/linux` を実行して Linux の依存関係をインストールします。
3. `script/flatpak/deps` スクリプトを実行して必要な依存関係をインストールします。
4. `script/flatpak/bundle-flatpak` を実行します。
5. これでパッケージがインストールされ、バンドルが `target/release/{app-id}.flatpak` に生成されます。

## メモリプロファイリング

[`heaptrack`](https://github.com/KDE/heaptrack) はメモリリークの診断に非常に有用です。インストールするには:

```sh
$ sudo apt install heaptrack heaptrack-gui
$ cargo install cargo-heaptrack
```

次に、プロファイラーを接続した状態で Gram をビルドして実行します:

```sh
$ cargo heaptrack -b gram
```

このエディターインスタンスを終了すると、ターミナル出力に、`*.raw.zst` プロファイルを `*.zst` ファイルに変換する `heaptrack_interpret` の実行コマンドが表示されます。このファイルは表示用に `heaptrack_gui` に渡せます。

## perf による記録

このセクションでは、実行中のエディターインスタンスから、シンボルが解決されたフレームグラフを生成する方法を説明します。これはエディターが CPU を大量に使用しているときのデバッグに使えます。ハングやメモリ使用量のデバッグには役立ちません。

### 問題の発生中に

- PID（プロセス ID）を次のコマンドで特定します:
  `ps -eo size,pid,comm | grep gram | sort | head -n 1 | cut -d ' ' -f 2` あるいは、htop/btop/top などで、RAM 使用量が最も多い gram-editor コマンドの PID を探します。

- perf をインストールします: Ubuntu（および派生版）では `sudo apt install linux-tools` を実行します。

- perf record を実行します: `sudo perf record -p <先ほど特定した PID>` を実行し、数秒待ってデータを収集したら Ctrl+C を押します。これで perf.data ファイルが生成されるはずです。

- 出力ファイルの所有者をユーザーに変更します: `sudo chown $USER:$USER perf.data`

- ビルド情報を取得します: Gram を再度起動し、コマンドパレットで `gram: about` と入力して正確なコミットを取得します。

### 後日

- シンボル付きで Gram をビルドします:
  - 先ほど特定したコミットをチェックアウトし、`Cargo.toml` を変更します。
  - 次の差分を適用してから、リリースビルドを行います。

    ```diff
    [profile.release]
    -debug = "limited"
    +debug = "full"
    ```

- シンボルを perf データベースに追加します:
  `pref buildid-cache -v -a <リリース版 gram バイナリへのパス>`

- データベースからシンボルを解決します:
  `perf inject -i perf.data -o perf_with_symbols.data`

- flamegraph をインストールします: `cargo install cargo-flamegraph`

- フレームグラフを描画します: `flamegraph --perfdata perf_with_symbols.data`

## トラブルシューティング

### 依存関係が不安定な機能を使用しているという Cargo のエラー

`cargo clean` を実行してから `cargo build` を試してください。
