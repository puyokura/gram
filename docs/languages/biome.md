# Biome

Gram での [Biome](https://biomejs.dev/) サポートは、コミュニティが保守する
[Biome 拡張機能](https://github.com/biomejs/biome-zed) によって提供されます。
問題はこちらへ報告してください:
[https://github.com/biomejs/biome-zed/issues](https://github.com/biomejs/biome-zed/issues)

- 言語サーバー: [biomejs/biome](https://github.com/biomejs/biome)

## Biome がサポートする言語

Biome 拡張機能は次の言語をサポートしています:

- JavaScript
- TypeScript
- JSX
- TSX
- JSON
- JSONC
- Vue.js
- Astro
- Svelte
- CSS

## 構成

既定では、`biome.json` ファイルがワークスペースのルートにある必要があります。

```json
{
  "$schema": "https://biomejs.dev/schemas/1.8.3/schema.json",
}
```

`biome.json` のオプションの完全な一覧は
[Biome の設定](https://biomejs.dev/reference/configuration/)
ドキュメントを参照してください。

機能と設定オプションの完全な一覧は、
[Biome Gram 拡張機能の README](https://github.com/biomejs/biome-zed) を参照してください。
