# Rego

Gram での Rego 言語サポートは、コミュニティが管理する
[Rego 拡張機能](https://github.com/StyraInc/zed-rego) によって提供されます。

- Tree-sitter:
  [FallenAngel97/tree-sitter-rego](https://github.com/FallenAngel97/tree-sitter-rego)
- 言語サーバー: [StyraInc/regal](https://github.com/StyraInc/regal)

## インストール

この拡張機能は主に [Regal](https://docs.styra.com/regal/language-server) 言語サーバーに基づいており、拡張機能を利用するには Regal をインストールする必要があります。詳細については
[利用開始手順](https://docs.styra.com/regal#getting-started) を参照してください。

## 設定

拡張機能の動作は `.regal/config.yaml` ファイルで設定します。以下は、`todo-comment` ルールを無効化し、`line-length` ルールをカスタマイズし、`opa-fmt` ルールでテストファイルを無視する設定例です:

```yaml
rules:
  style:
    todo-comment:
      # don't report on todo comments
      level: ignore
    line-length:
      # custom rule configuration
      max-line-length: 100
      # warn on too long lines, but don't fail
      level: warning
    opa-fmt:
      # not needed as error is the default, but
      # being explicit won't hurt
      level: error
      # files can be ignored for any individual rule
      # in this example, test files are ignored
      ignore:
        files:
          - "*_test.rego"
```

詳細については Regal の
[設定ドキュメント](https://docs.styra.com/regal#configuration)
を参照してください。
