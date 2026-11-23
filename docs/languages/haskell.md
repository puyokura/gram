# Haskell

Haskell のサポートは
[Haskell 拡張機能](https://github.com/zed-extensions/haskell) で利用できます。

- Tree-sitter:
  [tree-sitter-haskell](https://github.com/tree-sitter/tree-sitter-haskell)
- 言語サーバー:
  [haskell-language-server](https://github.com/haskell/haskell-language-server)

## HLS のインストール

[haskell-language-server のインストール](https://haskell-language-server.readthedocs.io/en/latest/installation.html)
（HLS）では、[ghcup](https://www.haskell.org/ghcup/install/) 経由の方法をおすすめします
（`curl --proto '=https' --tlsv1.2 -sSf https://get-ghcup.haskell.org | sh `）:

```sh
ghcup install hls
which haskell-language-server-wrapper
```

## HLS の構成

haskell-language-server（hls）を構成する必要がある場合は、Gram の `settings.jsonc`
に構成オプションを追加できます:

```json
{
  "lsp": {
    "hls": {
      "initialization_options": {
        "haskell": {
          "formattingProvider": "fourmolu",
        },
      },
    },
  },
}
```

その他のオプションについては、公式の
[haskell-language-server の構成](https://haskell-language-server.readthedocs.io/en/latest/configuration.html)
ドキュメントを参照してください。

特定の hls バイナリを使いたい場合や、代わりに
[static-ls](https://github.com/josephsumabat/static-ls) をそのまま置き換えて使いたい
場合は、バイナリのパスと引数を指定できます:

```json
{
  "lsp": {
    "hls": {
      "binary": {
        "path": "static-ls",
        "arguments": ["--experimentalFeatures"],
      },
    },
  },
}
```
