# Tailwind CSS

Gram は Tailwind CSS のオートコンプリート、リンティング、ホバープレビューを標準でサポートしています。

- 言語サーバー:
  [tailwindlabs/tailwindcss-intellisense](https://github.com/tailwindlabs/tailwindcss-intellisense)

## 設定

Tailwind CSS 言語サーバーを設定するには、
[拡張機能の設定](https://github.com/tailwindlabs/tailwindcss-intellisense?tab=readme-ov-file#extension-settings)
を参照し、`settings.json` の `lsp` セクションに追加します:

```json
{
  "lsp": {
    "tailwindcss-language-server": {
      "settings": {
        "classFunctions": ["cva", "cx"],
        "experimental": {
          "classRegex": ["[cls|className]\\s\\:\\=\\s\"([^\"]*)"],
        },
      },
    },
  },
}
```

Gram で Tailwind CSS と組み合わせて使用できる言語:

- [Astro](./astro.md)
- [CSS](./css.md)
- [ERB](./ruby.md)
- [Gleam](./gleam.md)
- [HEEx](./elixir.md#heex)
- [HTML](./html.md)
- [TypeScript](./typescript.md)
- [JavaScript](./javascript.md)
- [PHP](./php.md)
- [Svelte](./svelte.md)
- [Vue](./vue.md)

### Prettier プラグイン

Gram は Prettier を標準でサポートしています。つまり、
[Tailwind CSS Prettier プラグイン](https://github.com/tailwindlabs/prettier-plugin-tailwindcss)
がインストールされていれば、Prettier の設定に追加するだけで自動的に動作します:

```json
// .prettierrc
{
  "plugins": ["prettier-plugin-tailwindcss"],
}
```
