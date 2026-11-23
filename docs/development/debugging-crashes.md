# クラッシュのデバッグ

Gram がパニックまたはクラッシュしたときにミニダンプを生成するには、環境変数 `GRAM_GENERATE_MINIDUMPS=1` を設定します:

```sh
$ GRAM_GENERATE_MINIDUMPS=1 gram
```

この変数が設定されている場合、Gram はサイドカープロセスにメッセージを送り、そのプロセスがクラッシュしたエディターのメモリを検査して、[minidump](https://chromium.googlesource.com/breakpad/breakpad/+/master/docs/getting_started_with_breakpad.md#the-minidump-file-format) ファイルを `~/Library/Logs/Gram` または `$XDG_DATA_HOME/gram/logs` に作成します。このミニダンプは、全スレッドのスタックのバックトレースを生成するのに使用できます。

これらのクラッシュレポートには豊富な情報が含まれますが、スパンやシンボル情報を含まないため、読み解くのは困難です。それでもローカルで扱うには、使用している Gram リリースのソースと、ストリップされていないバイナリ（または別個のシンボルファイル）をダウンロードして、次のコマンドを実行します:

```sh
zstd -d ~/.local/share/gram/<uuid>.dmp -o minidump.dmp
minidump-stackwalk minidump.dmp
```

ログフォルダー内のミニダンプファイルと並んで、`<uuid>.json` があるはずです。このファイルには、パニックメッセージ、スパン、システム仕様などの追加メタデータが含まれます。

## デバッガーの使用

クラッシュを安定して再現できる場合は、デバッガーを使ってクラッシュ時のプログラムの状態を検査できます。これは多くの場合、クラッシュの原因を特定するうえで有用な洞察をもたらします。

Gram でのデバッガーの設定と使用方法、とくにクラッシュのデバッグについては[こちら](./debuggers.md#debugging-panics-and-crashes)を参照してください。
