# Scala

Gram での Scala 言語サポートは、コミュニティが保守する
[Scala 拡張機能](https://github.com/scalameta/metals-zed)によって提供されます。問題はこちらへ報告してください:
[https://github.com/scalameta/metals-zed/issues](https://github.com/scalameta/metals-zed/issues)

- Tree-sitter:
  [tree-sitter/tree-sitter-scala](https://github.com/tree-sitter/tree-sitter-scala)
- 言語サーバー: [scalameta/metals](https://github.com/scalameta/metals)

## セットアップ

- `cs setup`（Coursier）で Scala をインストールします: https://www.scala-lang.org/download/
  - `brew install coursier/formulas/coursier && cs setup`
- REPL（Almond）のセットアップ手順 https://almond.sh/docs/quick-start-install
  - `brew install --cask temurin`（Eclipse Foundation 公式の OpenJDK バイナリ）
  - `brew install coursier/formulas/coursier && cs setup`
  - `coursier launch --use-bootstrap almond -- --install`

## 設定

Metals 言語サーバーの動作は、次のファイルで制御できます:

- `.scalafix.conf` ファイル - [Scalafix の設定](https://scalacenter.github.io/scalafix/docs/users/configuration.html)を参照
- `.scalafmt.conf` ファイル - [Scalafmt の設定](https://scalameta.org/scalafmt/docs/configuration.html)を参照

これらのファイルはプロジェクトのルートに置くか、Metals の設定で場所を指定できます。詳しくは [Metals のユーザー設定](https://scalameta.org/metals/docs/editors/user-configuration)を参照してください。

<!--
TBD: Provide LSP configuration example for metals in Gram settings.json. metals.{javaHome,excludedPackages,customProjectRoot} etc.
-->
