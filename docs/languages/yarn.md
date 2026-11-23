# Yarn

[Yarn](https://yarnpkg.com/) は、JavaScript などの言語における依存関係の管理とワークフローの効率を改善する、多用途のパッケージマネージャーです。決定的な依存関係ツリーを保証し、オフラインサポートを提供し、信頼できるビルドのためのセキュリティを強化します。

## セットアップ

1. `yarn dlx @yarnpkg/sdks base` を実行して `.yarn/sdks` フォルダーを生成します。
2. 言語サーバー（例: VTSLS）が `.yarn/sdks/typescript/lib` フォルダーの TypeScript SDK を使うよう、[LSP の初期化オプション](../configuring-gram.md#lsp)で設定します。実際の設定方法は言語サーバーによって異なります。たとえば VTSLS では
   [`typescript.tsdk`](https://github.com/yioneko/vtsls/blob/6adfb5d3889ad4b82c5e238446b27ae3ee1e3767/packages/service/configuration.schema.json#L5)
   を設定します。
3. これで完了です！定義へ移動、コード補完、ホバー時のドキュメント表示といった言語サーバーの機能が動作するはずです。
