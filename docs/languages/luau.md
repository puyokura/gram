# Luau

[Luau](https://luau.org/) は、Lua から派生した高速で小さく、安全で段階的に型付けされる
埋め込み可能なスクリプト言語です。Luau は Roblox によって開発され、MIT ライセンスで
公開されています。

Gram での Luau 言語サポートは、コミュニティが保守する
[Luau 拡張機能](https://github.com/4teapo/zed-luau) によって提供されます。問題は
[https://github.com/4teapo/zed-luau/issues](https://github.com/4teapo/zed-luau/issues)
に報告してください。

- Tree-sitter:
  [4teapo/tree-sitter-luau](https://github.com/4teapo/tree-sitter-luau)
- 言語サーバー:
  [JohnnyMorganz/luau-lsp](https://github.com/JohnnyMorganz/luau-lsp)

## 構成

設定手順は
[Luau Gram 拡張機能の README](https://github.com/4teapo/zed-luau) にあります。

## フォーマット

コードの自動フォーマットを利用するには、Lua のコードフォーマッターである
[JohnnyMorganz/StyLua](https://github.com/JohnnyMorganz/StyLua) を使用できます。

インストール方法:

```sh
# macOS は Homebrew 経由
brew install stylua
# または Cargo 経由
cargo install stylua --features lua52,lua53,lua54,luau
```

次に、Gram の `settings.json` に以下を追加します:

```json
  "languages": {
    "Luau": {
      "formatter": {
        "external": {
          "command": "stylua",
          "arguments": ["-"]
        }
      }
    }
  }
```
