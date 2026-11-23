# OCaml

OCaml のサポートは
[OCaml 拡張機能](https://github.com/zed-extensions/ocaml) を通じて利用できます。

- Tree-sitter:
  [tree-sitter/tree-sitter-ocaml](https://github.com/tree-sitter/tree-sitter-ocaml)
- 言語サーバー: [ocaml/ocaml-lsp](https://github.com/ocaml/ocaml-lsp)

## セットアップ手順

開発環境がすでにセットアップされている場合は、
[Gram の起動](#launching-gram) へスキップしてください。

### Opam の使用

Opam は OCaml の公式パッケージマネージャーであり、OCaml を始める際に強く推奨されます。
Opam の使用を開始するには、[こちら](https://ocaml.org/install) に記載されている手順に従ってください。

Opam をインストールし、手順に従って開発環境で switch をセットアップしたら次に進みます。

<a id="launching-gram"></a>
### Gram の起動

LSP 用の `ocamllsp` コマンドラインツールがインストールされていることを確認します:

```sh
ocamllsp --help
```

コマンドが見つからない場合やヘルプテキストが表示されない場合は、`ocamllsp` のインストール手順を再確認し、正しくインストールされていることを確認してください。

注意: OCaml パッケージマネージャーの仕組み上、Gram はターミナルから実行する必要があるため、必ず Gram CLI をインストールしてください。

ターミナルから OCaml プロジェクトに移動し、次を実行します:

```sh
gram .
```

これで OCaml サポートが有効な状態で Gram が実行されます。
