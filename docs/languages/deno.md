# Deno

Deno のサポートは
[Deno 拡張機能](https://github.com/zed-extensions/deno) から利用できます。

- 言語サーバー:
  [Deno Language Server](https://docs.deno.com/runtime/manual/advanced/language_server/overview/)

## Deno の構成

TypeScript と TSX ファイルで Deno 言語サーバーを使うには、既定の言語サーバーを
無効にして deno を有効にするため、`settings.jsonc` に次を追加するとよいでしょう:

```json
{
  "lsp": {
    "deno": {
      "settings": {
        "deno": {
          "enable": true,
        },
      },
    },
  },
  "languages": {
    "JavaScript": {
      "language_servers": [
        "deno",
        "!typescript-language-server",
        "!vtsls",
        "!eslint",
      ],
      "formatter": "language_server",
    },
    "TypeScript": {
      "language_servers": [
        "deno",
        "!typescript-language-server",
        "!vtsls",
        "!eslint",
      ],
      "formatter": "language_server",
    },
    "TSX": {
      "language_servers": [
        "deno",
        "!typescript-language-server",
        "!vtsls",
        "!eslint",
      ],
      "formatter": "language_server",
    },
  },
}
```

詳しくは Gram ドキュメントの
[対応言語の構成](../configuring-languages.md) を参照してください。

## 構成の補完

`deno.json` や `package.json` の補完を得るには、`settings.jsonc` に次を追加します:
（詳細は ./json.md）

```json
"lsp": {
    "json-language-server": {
      "settings": {
        "json": {
          "schemas": [
            {
              "fileMatch": [
                "deno.json",
                "deno.jsonc"
              ],
              "url": "https://raw.githubusercontent.com/denoland/deno/refs/heads/main/cli/schemas/config-file.v1.json"
            },
            {
              "fileMatch": [
                "package.json"
              ],
              "url": "https://www.schemastore.org/package"
            }
          ]
        }
      }
    }
  }
```

## DAP サポート

deno プログラムをデバッグするには、`.gram/debug.jsonc` に次を追加します:

```json [debug]
[
  {
    "adapter": "JavaScript",
    "label": "Deno",
    "request": "launch",
    "type": "pwa-node",
    "cwd": "$GRAM_WORKTREE_ROOT",
    "program": "$GRAM_FILE",
    "runtimeExecutable": "deno",
    "runtimeArgs": ["run", "--allow-all", "--inspect-wait"],
    "attachSimplePort": 9229
  }
]
```

## 実行サポート

UI からテストなどの deno タスクを実行するには、`.gram/tasks.jsonc` に次を追加します:

```json [tasks]
[
  {
    "label": "deno test",
    "command": "deno test -A --filter '/^$GRAM_CUSTOM_DENO_TEST_NAME$/' '$GRAM_FILE'",
    "tags": ["js-test"]
  }
]
```

## 関連項目:

- [TypeScript](./typescript.md)
- [JavaScript](./javascript.md)
