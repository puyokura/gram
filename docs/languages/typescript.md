# TypeScript

TypeScript と TSX のサポートは、Gram でネイティブに利用できます。

- Tree-sitter:
  [tree-sitter/tree-sitter-typescript](https://github.com/tree-sitter/tree-sitter-typescript)
- 言語サーバー: [yioneko/vtsls](https://github.com/yioneko/vtsls)
- 代替の言語サーバー:
  [typescript-language-server/typescript-language-server](https://github.com/typescript-language-server/typescript-language-server)
- デバッグアダプター: [vscode-js-debug](https://github.com/microsoft/vscode-js-debug)

<!--
TBD: Document the difference between Language servers
-->

## 言語サーバー

既定では、Gram は TypeScript、TSX、JavaScript の各ファイルに [vtsls](https://github.com/yioneko/vtsls) を使用します。
[typescript-language-server](https://github.com/typescript-language-server/typescript-language-server)
は、設定ファイルで言語ごとに使用するよう設定できます:

```json
{
  "languages": {
    "TypeScript": {
      "language_servers": ["typescript-language-server", "!vtsls", "..."],
    },
    "TSX": {
      "language_servers": ["typescript-language-server", "!vtsls", "..."],
    },
    "JavaScript": {
      "language_servers": ["typescript-language-server", "!vtsls", "..."],
    },
  },
}
```

TypeScript ファイルでは既定で Prettier も使用されます。これを無効にするには:

```json
{
  "languages": {
    "TypeScript": {
      "prettier": { "allowed": false },
    },
    //...
  },
}
```

## 大規模なプロジェクト

`vtsls` は非常に大きなプロジェクトでメモリ不足になることがあります。既定値の 3072 に対して上限を 8092（8 GiB）に設定していますが、これでも足りない場合があります:

```json
{
  "lsp": {
    "vtsls": {
      "settings": {
        // For TypeScript:
        "typescript": { "tsserver": { "maxTsServerMemory": 16184 } },
        // For JavaScript:
        "javascript": { "tsserver": { "maxTsServerMemory": 16184 } },
      },
    },
  },
}
```

## インレイヒント

Gram は、言語サーバーがインレイヒントを返すよう、次の初期化オプションを設定します（つまり、Gram の設定でインレイヒントが有効になっている場合です）。

`typescript-language-server` を使用する場合、Gram の `settings.jsonc` でこれらの設定を上書きできます:

```json
{
  "lsp": {
    "typescript-language-server": {
      "initialization_options": {
        "preferences": {
          "includeInlayParameterNameHints": "all",
          "includeInlayParameterNameHintsWhenArgumentMatchesName": true,
          "includeInlayFunctionParameterTypeHints": true,
          "includeInlayVariableTypeHints": true,
          "includeInlayVariableTypeHintsWhenTypeMatchesName": true,
          "includeInlayPropertyDeclarationTypeHints": true,
          "includeInlayFunctionLikeReturnTypeHints": true,
          "includeInlayEnumMemberValueHints": true,
        },
      },
    },
  },
}
```

詳しくは
[typescript-language-server のインレイヒントのドキュメント](https://github.com/typescript-language-server/typescript-language-server?tab=readme-ov-file#inlay-hints-textdocumentinlayhint)
を参照してください。

`vtsls` を使用する場合:

```json
{
  "lsp": {
    "vtsls": {
      "settings": {
        // For JavaScript:
        "javascript": {
          "inlayHints": {
            "parameterNames": {
              "enabled": "all",
              "suppressWhenArgumentMatchesName": false,
            },
            "parameterTypes": {
              "enabled": true,
            },
            "variableTypes": {
              "enabled": true,
              "suppressWhenTypeMatchesName": true,
            },
            "propertyDeclarationTypes": {
              "enabled": true,
            },
            "functionLikeReturnTypes": {
              "enabled": true,
            },
            "enumMemberValues": {
              "enabled": true,
            },
          },
        },
        // For TypeScript:
        "typescript": {
          "inlayHints": {
            "parameterNames": {
              "enabled": "all",
              "suppressWhenArgumentMatchesName": false,
            },
            "parameterTypes": {
              "enabled": true,
            },
            "variableTypes": {
              "enabled": true,
              "suppressWhenTypeMatchesName": true,
            },
            "propertyDeclarationTypes": {
              "enabled": true,
            },
            "functionLikeReturnTypes": {
              "enabled": true,
            },
            "enumMemberValues": {
              "enabled": true,
            },
          },
        },
      },
    },
  },
}
```

## デバッグ

Gram は `vscode-js-debug` を使って、TypeScript コードのデバッグを標準でサポートしています。追加の設定を書かなくても、次のものをデバッグできます:

- `package.json` のタスク
- いくつかの人気フレームワーク（Jest、Mocha、Vitest、Jasmine、Bun、Node）で書かれたテスト

{#action debugger::Start}（{#kb debugger::Start}）を実行すると、これらの定義済みデバッグタスクの文脈依存の一覧が表示されます。

> **注:** Bun のテストは、`package.json` に `@types/bun` がある場合に自動検出されます。
>
> **注:** Node のテストは、`package.json` に `@types/node` がある場合に自動検出されます（Node.js 20 以降が必要）。

すべての言語と同様、`.vscode/launch.json` の設定も Gram でのデバッグに使用できます。

これらのいずれにも当てはまらないユースケースでは、`.gram/debug.jsonc` にデバッグ構成を追加することで完全に制御できます。構成例は以下を参照してください。

### JavaScript デバッグタスクの設定

JavaScript のデバッグは、Node.js とブラウザーの2つの異なる環境があるため、他の言語より複雑です。`vscode-js-debug` は `type` フィールドを公開しており、これを使って環境（`node` または `chrome`）を指定できます。

- [vscode-js-debug の設定ドキュメント](https://github.com/microsoft/vscode-js-debug/blob/main/OPTIONS.md)

### Web ブラウザーで動作するサーバーにデバッガーをアタッチする（`npx serve`）

外部で実行している Web サーバー（例: `npx serve` や `npx live-server`）がある場合、それにアタッチしてブラウザーで開けます。

```json [debug]
[
  {
    "label": "Launch Chrome (TypeScript)",
    "adapter": "JavaScript",
    "type": "chrome",
    "request": "launch",
    "url": "http://localhost:5500",
    "program": "$GRAM_FILE",
    "webRoot": "${GRAM_WORKTREE_ROOT}",
    "build": {
      "command": "npx",
      "args": ["tsc"]
    },
    "skipFiles": ["<node_internals>/**"]
  }
]
```

## 関連項目

- [Gram の Yarn ドキュメント](./yarn.md): プロジェクトで Yarn を使うための設定の手順。
- [Gram の Deno ドキュメント](./deno.md)
