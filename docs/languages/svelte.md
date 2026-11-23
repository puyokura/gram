# Svelte

Svelte のサポートは、[Svelte 拡張機能](https://github.com/zed-extensions/svelte)を通じて利用できます。

- Tree-sitter:
  [tree-sitter-grammars/tree-sitter-svelte](https://github.com/tree-sitter-grammars/tree-sitter-svelte)
- 言語サーバー:
  [sveltejs/language-tools](https://github.com/sveltejs/language-tools)

## 追加のテーマスタイル設定

ディレクティブやモディファイアーなど、属性内の特定のスタイルの表示方法を変更できます:

```json
"syntax": {
  // Styling for directives (e.g., `class:foo` or `on:click`) (the `on` or `class` part of the attribute).
  "attribute.function": {
    "color": "#ff0000"
  },
  // Styling for modifiers at the end of attributes, e.g. `on:<click|preventDefault|stopPropagation>`
  "attribute.special": {
    "color": "#00ff00"
  }
}
```

## インレイヒント

Gram でインレイヒントが有効になっているとき、言語サーバーがインレイヒントを返すよう、Gram は次の初期化オプションを設定します:

```json
"inlayHints": {
  "parameterNames": {
    "enabled": "all",
    "suppressWhenArgumentMatchesName": false
  },
  "parameterTypes": {
    "enabled": true
  },
  "variableTypes": {
    "enabled": true,
    "suppressWhenTypeMatchesName": false
  },
  "propertyDeclarationTypes": {
    "enabled": true
  },
  "functionLikeReturnTypes": {
    "enabled": true
  },
  "enumMemberValues": {
    "enabled": true
  }
}
```

これらの設定を上書きするには、次のようにします:

```json
"lsp": {
  "svelte-language-server": {
    "initialization_options": {
      "configuration": {
        "typescript": {
          // ......
        },
        "javascript": {
          // ......
        }
      }
    }
  }
}
```

詳しくは
[TypeScript 言語サーバーの `package.json`](https://github.com/microsoft/vscode/blob/main/extensions/typescript-language-features/package.json)
を参照してください。
