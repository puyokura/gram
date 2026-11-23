# Proto

Proto/proto3（Protocol Buffers 定義言語）のサポートは
[Proto 拡張機能](https://codeberg.org/GramEditor/proto-extension) を通じて利用できます。

- Tree-sitter:
  [coder3101/tree-sitter-proto](https://github.com/coder3101/tree-sitter-proto)
- 言語サーバー:
  [protobuf-language-server](https://github.com/lasorda/protobuf-language-server)

## セットアップ

Protobuf には 2 つの言語サーバー実装が利用可能です。いずれかを選択し、以下のインストール手順に従ってください。

### protobuf-language-server のインストール

protobuf-language-server をインストールし、PATH に含まれていることを確認します:

```
go install github.com/lasorda/protobuf-language-server@latest
which protobuf-language-server
```

### ProtoLS のインストール

protols をインストールし、PATH に含まれていることを確認します:

```
cargo install protols
which protols
```

## 設定

```json
"lsp": {
  "protobuf-language-server": {
    "binary": {
      "path": "protols"
    }
  }
}
```

## フォーマット

ProtoLS は、`clang-format` がインストールされている場合にフォーマットをサポートします。

```sh
# MacOS:
brew install clang-format
# Ubuntu
sudo apt-get install clang-format
# Fedora
sudo dnf install clang-tools-extra
```

フォーマット設定をカスタマイズするには、`.clang-format` ファイルを作成します（例）:

```clang-format
IndentWidth: 4
ColumnLimit: 120
```

あるいは、設定でフォーマッターとして指定することにより、Gram から直接 `clang-format` を呼び出すこともできます:

```json
  "languages": {
    "Proto": {
      "format_on_save": "on",
      "tab_size": 4,
      "formatter": {
        "external": {
          "command": "clang-format",
          "arguments": ["-style={IndentWidth: 4, ColumnLimit: 0}"]
        }
      }
    },
  }
```
