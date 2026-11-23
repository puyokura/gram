# Nix

Nix のサポートはエディターに組み込まれています。

## 言語サーバーの選択

`nil` と `nixd` という 2 つの異なる言語サーバーがサポートされています。

`nil` のみを有効にする場合:

```json
  "languages": {
    "Nix": {
      "language_servers": ["nil", "!nixd", "..."]
    },
  }
```

`nixd` のみを有効にする場合:

```json
  "languages": {
    "Nix": {
      "language_servers": ["!nil", "nixd", "..."]
    },
  }
```

## 設定

言語サーバーのオプションは `settings.jsonc` ファイルで設定できます。詳細については
[言語の設定](../configuring-languages.md) ドキュメントを参照してください。

### nixd

詳細については、
[nixd のドキュメント](https://github.com/nix-community/nixd/blob/main/nixd/docs/configuration.md)
を参照してください。

```json
{
  "lsp": {
    "nixd": {
      "settings": {
        "diagnostic": {
          "suppress": [ "sema-extra-with" ]
        }
      }
    }
  }
}
```

### nil

詳細については、
[nil のドキュメント](https://github.com/oxalica/nil/blob/main/docs/configuration.md)
を参照してください。

```json
{
  "lsp": {
    "nil": {
      "settings": {
         "diagnostics": {
          "ignored": [ "unused_binding" ]
        }
      }
    }
  }
}
```
