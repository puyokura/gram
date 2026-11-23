# 肥大化

現在の主な課題は、バイナリのサイズを減らし、コードを読みやすくすることです。
これまでは主に、依存関係とコードを可能な限り削除する形で取り組んできました。
ここでは、その作業に役立ったツールと手法についてのメモを残します。

まず有用な情報源は
[Rust performance book](https://nnethercote.github.io/perf-book/title-page.html)
です。

次のリポジトリには、主に cargo のオプションに関する役立つヒントがありました:
<https://github.com/johnthagen/min-sized-rust>

## 未使用の依存関係

未使用の依存関係を見つけるツールは2つあります:

- `cargo-machete`: <https://github.com/bnjbvr/cargo-machete>
- `cargo-shear`: <https://github.com/Boshen/cargo-shear>

両方とも実行しています。`cargo-shear` のほうが精度が高く、より多くの未使用依存関係を見つけられるうえ、誤検知も少ないようです。

## async のコスト

まだ本格的に調べていませんが、Rust の async コードは、関数本体をステートマシンに書き換えるため、コード量が大きく膨らむコストがあります。詳細は次の記事を参照してください:
<https://tweedegolf.nl/en/blog/237/async-rust-never-left-the-mvp-state>

このコードベースはどこでも async を多用しているため、async である必要のないコードを直線的なコードに書き換え、長く走るタスクをバックグラウンドスレッドで実行するようにすれば、多くの改善が見込めるはずです。

## ジェネリックコードの増幅

> [!NOTE]
>
> 別名: 単相化（モノモルフィゼーション）による肥大化。

ジェネリック引数を取る関数や構造体は、使用されている各バリアントごとに別々のコピーとしてコンパイルされます。大きな関数や、コピーを大量に生成している関数を見つけるのに役立つツールをいくつか見つけました:

- `cargo-bloat`: <https://github.com/RazrFalcon/cargo-bloat>
- `cargo-llvm-lines`: <https://github.com/dtolnay/cargo-llvm-lines>

`cargo llvm-lines` は、関数ごとに生成されるコード量を示します:

```bash
CARGO_PROFILE_RELEASE_LTO=fat cargo llvm-lines --release --sort copies -p gram --bin gram > llvm-lines.txt
```

完全な LTO を有効にすると、すべての単相化が同じクレートに現れます。`--sort copies` を付けると、バリアント数が多い関数から順に一覧できます。

これは大きな問題であることが分かりました。とくに設定関連のコードが膨大な量のコードを生成しており、ジェネリック関数の外にコードを移す工夫でサイズをかなり削減できましたが、設定の実装全体はおそらくゼロから書き直すべきです。
