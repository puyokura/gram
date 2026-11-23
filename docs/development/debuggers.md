# デバッガーの使用

> **免責事項**: これは [Gram のデバッガーを設定する](../debugger.md)ためのドキュメントではありません。Gram 自体を開発する際にデバッガーを使う方法について説明することを目的としています。

## Gram 内蔵デバッガーの使用

Gram プロジェクトを開いている状態で、`New Process Modal` を開き、`Debug` タブを選択できます。そこには Gram をデバッグするためのデバッグ構成が表示され、GDB 用と LLDB 用が1つずつあります。使用する構成を選択すると、Gram がバイナリをビルドして起動します。

注: GDB は ARM 搭載の MacBook ではサポートされていません。

## リリースビルドプロファイルに関する考慮事項

既定では、リリースプロファイル（製品ビルド、つまり stable で使われるプロファイル）を使用するビルドには、限定的なデバッグ情報のみが含まれます。

これは、ルートの `Cargo.toml` にある `profile.(release).debug` フィールドを `"limited"` に設定することで行われます。

`debug` フィールドの公式ドキュメントは[こちら](https://doc.rust-lang.org/cargo/reference/profiles.html#debug)にあります。要点は、`"limited"` を指定すると型と変数レベルのデバッグ情報が取り除かれるということです。

リリースビルドでこの設定を行っているのは、バイナリサイズを削減するためです。型と変数レベルのデバッグ情報は不要であり、生成されるスタックトレースの使いやすさにも影響しません。

ただし、型と変数レベルのデバッグ情報は、良いスタックトレースには不要であっても、デバッガーを使う体験には非常に重要です。型と変数レベルのデバッグ情報がないと、デバッガーはローカル変数を解決できず、値を検査したり、プリティプリンターで整形表示したりできません。

そのため、リリースビルドをデバッグするときにデバッガーを最大限活用するには、完全なデバッグ情報付きで Gram バイナリを新たにコンパイルする必要があります。

最も簡単な方法は、次のように `cargo run` や `cargo build` を実行する際に `--config` フラグを使って、ルートの `Cargo.toml` の `debug` フィールドを上書きすることです:

```sh
cargo run --config 'profile.release.debug="full"'
cargo build --config 'profile.release.debug="full"'
```

> `cargo` を呼び出すたびに `--config` フラグを渡すのを避けたい場合は、[ルートの `Cargo.toml`](https://github.com/zed-industries/zed/blob/main/Cargo.toml) の該当セクションを
>
> ```toml
> [profile.release]
> debug = "limited"
> ```
>
> から
>
> ```toml
> [profile.release]
> debug = "full"
> ```
>
> に変更してもかまいません。
>
> これにより、`cargo run --release` や `cargo build --release` を呼び出すたびに、完全なデバッグ情報付きでコンパイルされます。
>
> **警告:** これらの変更をコミットしないよう注意してください！

## シェルデバッガー GDB/LLDB で Gram を実行する

### 背景

Gram の開発用に Rust を入手する推奨の方法は、[rustup](https://rustup.rs) 経由です。詳細は[各プラットフォームでの始め方](../development.md)のドキュメントを参照してください。

rustup で Rust をインストールすると、Rust でコンパイルされたバイナリのデバッグを支援する追加のスクリプトがいくつかインストールされ、パスに追加されます: `rust-gdb` と `rust-lldb`。これらのスクリプトと、それが役に立つ理由については[brief respite](https://michaelwoerister.github.io/2015/03/27/rust-xxdb.html)で詳しく説明されています。

要約すると、これらは標準の `gdb` と `lldb` コマンドをラップする単純なシェルスクリプトで、プリティプリンターや型情報といった Rust 固有の追加機能を有効にするためのコマンドやフラグを注入します。

`rust-gdb` や `rust-lldb` を使うには、`gdb` または `lldb` がシステムにインストールされている必要があります。インストールされていない場合は、プラットフォームに適した方法でインストールしてください。

上記の記事から引用します:

> "サポートされるデバッガーの最小バージョンは GDB 7.7 と LLDB 310 です。ただし、一般的な原則は「新しいほど良い」です。"

可能であれば、最新バージョンの `gdb` または `lldb` をインストールすることをおすすめします。

> **注**: Windows では、Windows 向けの `gdb` サポートが安定していないため、`rust-gdb` は既定ではインストールされません。代わりに `rust-lldb` と合わせて `lldb` を使用してください。

- [gdb](https://www.gnu.org/software/gdb/)
- [lldb](https://lldb.llvm.org/)

### 使い方

上記の手順に従い、Gram をコンパイルする際に完全なデバッグ情報を含めます。その後、`cargo build` でビルドした Gram バイナリに対して `rust-gdb` または `rust-lldb` を実行します:

```
# for gdb:
rust-gdb target/debug/gram

# for lldb:
rust-lldb target/debug/gram
```

あるいは、実行中の Gram インスタンス（`cargo run` で起動したインスタンスなど）にアタッチします:

```
rust-gdb -p <pid>
rust-lldb -p <pid>
```

ここで `<pid>` は、アタッチしたい Gram インスタンスのプロセス ID です。

実行中の Gram インスタンスのプロセス ID を取得するには、Windows の `Task Manager` や macOS の `Activity Monitor` といった OS のプロセス管理ツールを使用するか、ターミナルから次のコマンドを実行します:

```
# Mac or Linux
ps aux | grep gram

# Windows (PowerShell)
Get-Process | Select-Object Id, ProcessName
```

#### パニックとクラッシュのデバッグ

デバッガーは、Gram を含むあらゆるプログラムで、パニックやクラッシュの原因をデバッグするための優れたツールになります。

既定では、`gdb` や `lldb` がアタッチしているプロセスがパニックなどの例外に達すると、デバッガーは自動的にパニックの地点で停止し、プログラムの状態を検査できるようになります。

多くの場合、デバッガーが停止する地点は Rust 標準ライブラリのパニック処理や例外処理のコードの奥深くにあるため、スタックトレースを遡って実際の原因を見つける必要があります。

これは、`lldb` では `backtrace` コマンドと `frame select` コマンドを組み合わせて行えます。`gdb` にも同様のコマンドがあります。

プログラムが停止すると、例外に達する前のように実行を続けることはできません。ただし、さまざまなスタックフレームに移動し、各フレーム内の変数や式の値を検査できます。これはクラッシュの根本原因を特定するうえで非常に有用です。

詳しくは[クラッシュのデバッグ](./debugging-crashes.md)を参照してください。
