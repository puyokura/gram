# JavaScript

JavaScript のサポートは Gram にネイティブで組み込まれています。

- Tree-sitter:
  [tree-sitter/tree-sitter-javascript](https://github.com/tree-sitter/tree-sitter-javascript)
- 言語サーバー: [yioneko/vtsls](https://github.com/yioneko/vtsls)
- 代替の言語サーバー:
  [typescript-language-server/typescript-language-server](https://github.com/typescript-language-server/typescript-language-server)
- デバッグアダプター: [vscode-js-debug](https://github.com/microsoft/vscode-js-debug)

## コードフォーマット

JavaScript では、TypeScript 組み込みのコードフォーマットを使って保存時のフォーマットが
既定で有効です。ただし、多くの JavaScript プロジェクトでは [Prettier](https://prettier.io/)
などの他のコマンドラインコードフォーマットツールを使っています。設定で JavaScript 用の
_外部_ コードフォーマッターを指定すれば、これらのツールを利用できます。詳しくは
[構成のドキュメント](../configuring-gram.md) を参照してください。

たとえば、Prettier がインストールされ `PATH` に入っている場合は、`settings.jsonc` に
次を追加すると Prettier で JavaScript ファイルをフォーマットできます:

```json
{
  "languages": {
    "JavaScript": {
      "formatter": {
        "external": {
          "command": "prettier",
          "arguments": ["--stdin-filepath", "{buffer_path}"],
        },
      },
    },
  },
}
```

## JSX

Gram は JSX のシンタックスハイライトを標準でサポートしています。

JSX の文字列では、[`tailwindcss-language-server`](./tailwindcss.md) が Tailwind CSS の
クラスの自動補完を提供します。

## JSDoc

Gram は、JSDoc の構文に一致する JavaScript と TypeScript のコメントで JSDoc 構文を
サポートしています。Gram は JSDoc の解析とハイライトに
[tree-sitter/tree-sitter-jsdoc](https://github.com/tree-sitter/tree-sitter-jsdoc) を
使用します。

## ESLint

フォーマット時に ESLint のコードアクションを実行するように設定すると、`eslint --fix` を
使ってコードをフォーマットできます:

```json
{
  "languages": {
    "JavaScript": {
      "code_actions_on_format": {
        "source.fixAll.eslint": true,
      },
    },
  },
}
```

`fixAll` を使うときに 1 つの ESLint ルールだけを実行することもできます:

```json
{
  "languages": {
    "JavaScript": {
      "code_actions_on_format": {
        "source.fixAll.eslint": true,
      },
    },
  },
  "lsp": {
    "eslint": {
      "settings": {
        "codeActionOnSave": {
          "rules": ["import/order"],
        },
      },
    },
  },
}
```

> **注:** 設定した他のフォーマッターは、ESLint の後に引き続き実行されます。そのため、
> 言語サーバーや Prettier の構成が ESLint のルールに従ってフォーマットしない場合、
> それらが ESLint の修正内容を上書きし、エラーが残ることになります。

保存時に ESLint だけを実行したい場合は、コードアクションをフォーマッターとして設定
できます:

```json
{
  "languages": {
    "JavaScript": {
      "formatter": [],
      "code_actions_on_format": {
        "source.fixAll.eslint": true,
      },
    },
  },
}
```

### ESLint の `nodePath` を構成する

ESLint の `nodePath` 設定を構成できます:

```json
{
  "lsp": {
    "eslint": {
      "settings": {
        "nodePath": ".yarn/sdks",
      },
    },
  },
}
```

### ESLint の `problems` を構成する

ESLint の `problems` 設定を構成できます。

たとえば、`problems.shortenToSingleLine` は次のように設定します:

```json
{
  "lsp": {
    "eslint": {
      "settings": {
        "problems": {
          "shortenToSingleLine": true,
        },
      },
    },
  },
}
```

### ESLint の `rulesCustomizations` を構成する

ESLint の `rulesCustomizations` 設定を構成できます:

```json
{
  "lsp": {
    "eslint": {
      "settings": {
        "rulesCustomizations": [
          // すべての eslint エラー/警告を警告として表示する
          { "rule": "*", "severity": "warn" },
        ],
      },
    },
  },
}
```

### ESLint の `workingDirectory` を構成する

ESLint の `workingDirectory` 設定を構成できます:

```json
{
  "lsp": {
    "eslint": {
      "settings": {
        "workingDirectory": {
          "mode": "auto",
        },
      },
    },
  },
}
```

## デバッグ

Gram は `vscode-js-debug` を使って、JavaScript コードのデバッグを標準でサポートして
います。追加の構成を書かなくても、次のものをデバッグできます:

- `package.json` のタスク
- 各種の人気フレームワーク（Jest、Mocha、Vitest、Jasmine、Bun、Node）で書かれたテスト

{#action debugger::Start}（{#kb debugger::Start}）を実行すると、あらかじめ定義された
デバッグタスクがコンテキストに応じて一覧表示されます。

> **注:** Bun のテストは、`package.json` に `@types/bun` があると自動的に検出されます。
>
> **注:** Node のテストは、`package.json` に `@types/node` があると自動的に検出されます
> （Node.js 20 以降が必要です）。

他の言語と同様に、`.vscode/launch.json` の構成も Gram でのデバッグに利用できます。

ここまでのいずれにも当てはまらない用途では、`.gram/debug.jsonc` にデバッグ構成を追加
すれば完全に制御できます。構成例は以下に示します。

### JavaScript のデバッグタスクを構成する

JavaScript のデバッグは、Node.js とブラウザという 2 つの異なる環境があるため、他の言語
より複雑です。`vscode-js-debug` は `type` フィールドを公開しており、`node` または
`chrome` を指定して環境を選べます。

- [vscode-js-debug の構成ドキュメント](https://github.com/microsoft/vscode-js-debug/blob/main/OPTIONS.md)

### Node で現在のファイルをデバッグする

```json [debug]
[
  {
    "adapter": "JavaScript",
    "label": "Debug JS file",
    "type": "node",
    "request": "launch",
    "program": "$GRAM_FILE",
    "skipFiles": ["<node_internals>/**"]
  }
]
```

### Chrome で Web アプリを起動する

```json [debug]
[
  {
    "adapter": "JavaScript",
    "label": "Debug app in Chrome",
    "type": "chrome",
    "request": "launch",
    "file": "$GRAM_WORKTREE_ROOT/index.html",
    "webRoot": "$GRAM_WORKTREE_ROOT",
    "console": "integratedTerminal",
    "skipFiles": ["<node_internals>/**"]
  }
]
```

## 関連項目

- [Yarn のドキュメント](./yarn.md): プロジェクトで Yarn を使うための構成手順。
- [TypeScript のドキュメント](./typescript.md)
