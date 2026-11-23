# Windows 向け Gram のビルド

> 次のコマンドは任意のシェルで実行できます。

## リポジトリ

Gram の[リポジトリ](https://codeberg.org/GramEditor/gram)をクローンします。

> [!NOTE]
>
> Windows ビルドがまだ機能するかどうか、また動作させるために何が必要かは不明です。Windows ビルドは署名も必要になるため、証明書が必要です。

おそらく次のようなコマンドでしょうか？

```powershell
.\script\bundle-windows.ps1
```

## 依存関係

- [rustup](https://www.rust-lang.org/tools/install) をインストールします

- 次のオプションコンポーネントを付けて [Visual Studio](https://visualstudio.microsoft.com/downloads) をインストールします: `MSVC v*** - VS YYYY C++ x64/x86 build tools` と `MSVC v*** - VS YYYY C++ x64/x86 Spectre-mitigated libs (latest)`（`v***` は VS のバージョン、`YYYY` は VS のリリース年です。アーキテクチャに注意し、必要に応じて自分の環境に合わせて変更してください。）
- あるいは、MSVC コンパイラーツールだけを含むより軽量なインストーラーを希望する場合は、[ビルドツール](https://visualstudio.microsoft.com/visual-cpp-build-tools)（+ 上記のライブラリー）と「C++ によるデスクトップ開発」ワークロードをインストールします。ただし、このインストールは rustup によって自動的に認識されないことに注意してください。このインストールがスタートメニューまたは Windows Terminal に配置する「開発者用」シェル（cmd/powershell）を最初に起動して環境変数を初期化してから、コンパイルする必要があります。
- システムに応じて Windows 11 または 10 SDK をインストールしますが、少なくとも `Windows 10 SDK version 2104 (10.0.20348.0)` がマシンにインストールされていることを確認してください。[Windows SDK Archive](https://developer.microsoft.com/windows/downloads/windows-sdk) からダウンロードできます。
- [CMake](https://cmake.org/download) をインストールします（[依存クレート](https://docs.rs/wasmtime-c-api-impl/latest/wasmtime_c_api)が必要とするもの）。Visual Studio Installer からインストールして、その `bin` フォルダーを手動で `PATH` に追加する方法もあります。例: `C:\Program Files\Microsoft Visual Studio\2022\Community\Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin`。

Gram をコンパイルできない場合は、Visual Studio をインストールした環境では少なくとも次のコンポーネントが入っていることを確認してください:

```json
{
  "version": "1.0",
  "components": [
    "Microsoft.VisualStudio.Component.CoreEditor",
    "Microsoft.VisualStudio.Workload.CoreEditor",
    "Microsoft.VisualStudio.Component.VC.Tools.x86.x64",
    "Microsoft.VisualStudio.ComponentGroup.WebToolsExtensions.CMake",
    "Microsoft.VisualStudio.Component.VC.CMake.Project",
    "Microsoft.VisualStudio.Component.Windows11SDK.26100",
    "Microsoft.VisualStudio.Component.VC.Runtimes.x86.x64.Spectre",
  ],
  "extensions": [],
}
```

ビルドツールのみの場合は、次のコンポーネントです:

```json
{
  "version": "1.0",
  "components": [
    "Microsoft.VisualStudio.Component.Roslyn.Compiler",
    "Microsoft.Component.MSBuild",
    "Microsoft.VisualStudio.Component.CoreBuildTools",
    "Microsoft.VisualStudio.Workload.MSBuildTools",
    "Microsoft.VisualStudio.Component.Windows10SDK",
    "Microsoft.VisualStudio.Component.VC.CoreBuildTools",
    "Microsoft.VisualStudio.Component.VC.Tools.x86.x64",
    "Microsoft.VisualStudio.Component.VC.Redist.14.Latest",
    "Microsoft.VisualStudio.Component.Windows11SDK.26100",
    "Microsoft.VisualStudio.Component.VC.CMake.Project",
    "Microsoft.VisualStudio.Component.TextTemplating",
    "Microsoft.VisualStudio.Component.VC.CoreIde",
    "Microsoft.VisualStudio.ComponentGroup.NativeDesktop.Core",
    "Microsoft.VisualStudio.Workload.VCTools",
    "Microsoft.VisualStudio.Component.VC.Runtimes.x86.x64.Spectre",
  ],
  "extensions": [],
}
```

この一覧は次の手順で取得できます:

- Visual Studio Installer を開きます
- `Installed` タブの `More` をクリックします
- `Export configuration` をクリックします

### 注意事項

`data` フォルダーの `pg_hba.conf` ファイルを変更し、`host` メソッドで `scram-sha-256` の代わりに `trust` を使用する必要があります。そうしないと、接続は `password authentication failed` エラーで失敗します。`pg_hba.conf` ファイルは通常 `C:\Program Files\PostgreSQL\17\data\pg_hba.conf` にあります。変更後、ファイルは次のようになります:

```ini
# IPv4 local connections:
host    all             all             127.0.0.1/32            trust
# IPv6 local connections:
host    all             all             ::1/128                 trust
```

また、ラテン文字以外の Windows を使用している場合は、`data` フォルダーの `postgresql.conf` ファイルにある `lc_messages` パラメーターを `English_United States.1252`（または利用可能な UTF8 互換のエンコーディング）に変更する必要があります。そうしないと、データベースがパニックします。`postgresql.conf` ファイルは次のようになります:

```ini
# lc_messages = 'Chinese (Simplified)_China.936' # locale for system error message strings
lc_messages = 'English_United States.1252'
```

その後、`postgresql` サービスを再起動してください。`win` キー + `R` を押して `Run` ウィンドウを開きます。`services.msc` と入力して `OK` ボタンを押すと、サービス マネージャーが開きます。次に `postgresql-x64-XX` サービスを見つけ、右クリックして `Restart` を選択します。

## ソースからのビルド

依存関係をインストールしたら、[Cargo](https://doc.rust-lang.org/cargo) を使って Gram をビルドできます。

デバッグビルド:

```powershell
cargo run
```

リリースビルド:

```powershell
cargo run --release
```

テストを実行する場合:

```powershell
cargo test --workspace
```

## msys2 からのインストール

Gram は、Mingw-w64 向けにビルドされた非公式の MSYS2 Gram パッケージをサポートしておらず、代わりに [mingw-w64-zed](https://packages.msys2.org/base/mingw-w64-zed) を使用します。

まず [MSYS2 のドキュメント](https://www.msys2.org/docs/ides-editors/#zed)を参照してください。

## トラブルシューティング

### `RUSTFLAGS` 環境変数を設定するとビルドが壊れる

`RUSTFLAGS` 環境変数を設定すると、Gram を正しくビルドするために必要な `.cargo/config.toml` の `rustflags` 設定が上書きされます。

これらの設定は時々変わるため、受け取るビルドエラーはリンカーエラーから他の奇妙なエラーまでさまざまです。

追加の rust フラグを加えたい場合は、`.cargo/config.toml` で次のいずれかを行えます:

build セクションにフラグを追加する

```toml
[build]
rustflags = ["-C", "symbol-mangling-version=v0", "--cfg", "tokio_unstable"]
```

windows ターゲットセクションにフラグを追加する

```toml
[target.'cfg(target_os = "windows")']
rustflags = [
    "--cfg",
    "windows_slim_errors",
    "-C",
    "target-feature=+crt-static",
]
```

または、Gram リポジトリと同じフォルダーに新しい `.cargo/config.toml` を作成します（下記参照）。元の `.cargo/config.toml` を編集する必要がないため、CI ビルドを行う場合にとくに便利です。

```tree
upper_dir
├── .cargo          // <-- Make this folder
│   └── config.toml // <-- Make this file
└── gram
    ├── .cargo
    │   └── config.toml
    └── crates
        └── ...
```

新しい（上記の）`.cargo/config.toml` で、rustflags に `--cfg gles` を追加したい場合、次のようになります

```toml
[target.'cfg(all())']
rustflags = ["--cfg", "gles"]
```

### 依存関係が不安定な機能を使用しているという Cargo のエラー

`cargo clean` を実行してから `cargo build` を試してください。

### `STATUS_ACCESS_VIOLATION`

このエラーは、"rust-lld.exe" リンカーを使用している場合に発生することがあります。別のリンカーを試してみてください。

グローバル設定を使用している場合は、Gram リポジトリをネストしたフォルダーに移動し、親フォルダーにカスタムリンカー設定を含む `.cargo/config.toml` を追加することを検討してください。

詳細はこの issue を参照してください
[#12041](https://github.com/zed-industries/zed/issues/12041)

### 無効な RC パスが選択される

ラップトップに適用されているセキュリティルールによっては、Gram のコンパイル中に次のエラーが発生することがあります:

```powershell
error: failed to run custom build command for `gram(C:\Users\USER\src\gram\crates\gram)`

Caused by:
  process didn't exit successfully: `C:\Users\USER\src\gram\target\debug\build\gram-b24f1e9300107efc\build-script-build` (exit code: 1)
  --- stdout
  cargo:rerun-if-changed=../../.git/logs/HEAD
  cargo:rustc-env=GRAM_COMMIT_SHA=25e2e9c6727ba9b77415588cfa11fd969612adb7
  cargo:rustc-link-arg=/stack:8388608
  cargo:rerun-if-changed=resources/windows/app-icon.ico
  package.metadata.winresource does not exist
  Selected RC path: 'bin\x64\rc.exe'

  --- stderr
  The system cannot find the path specified. (os error 3)
warning: build failed, waiting for other jobs to finish...
```

この問題を修正するには、環境変数 `GRAM_RC_TOOLKIT_PATH` を RC ツールキットのパスに手動で設定します。通常は次のように設定できます: `C:\Program Files (x86)\Windows Kits\10\bin\<SDK_version>\x64`。

詳細はこの [issue](https://github.com/zed-industries/zed/issues/18393) を参照してください。

### グラフィックスの問題

#### Gram の起動に失敗する

現在、Gram は Windows 上でグラフィックス API として Vulkan を使用しています。しかし Vulkan は Windows で常に最も信頼できるわけではないため、Gram の起動に失敗する場合は Vulkan 関連の問題である可能性が高いです。

Gram のログは `C:\Users\YOU\AppData\Local\Gram\logs\Gram.log` で確認できます。

次のようなメッセージが表示される場合:

- `Gram failed to open a window: NoSupportedDeviceFound`
- `ERROR_INITIALIZATION_FAILED`
- `GPU Crashed`
- `ERROR_SURFACE_LOST_KHR`

Vulkan がシステム上で正しく動作していない可能性があります。多くの場合、GPU ドライバーを更新すると解決します。

ログに Vulkan 関連の記述がなく、Bandicam をインストールしている場合は、アンインストールしてみてください。Gram は現在 Bandicam と互換性がありません。
