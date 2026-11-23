# Kotlin

Gram での Kotlin 言語サポートは、コミュニティが保守する
[Kotlin 拡張機能](https://github.com/zed-extensions/kotlin) によって提供されます。
問題は
[https://github.com/zed-extensions/kotlin/issues](https://github.com/zed-extensions/kotlin/issues)
に報告してください。

- Tree-sitter:
  [fwcd/tree-sitter-kotlin](https://github.com/fwcd/tree-sitter-kotlin)
- 言語サーバー:
  [fwcd/kotlin-language-server](https://github.com/fwcd/kotlin-language-server)

## 構成

ワークスペースの構成オプションは、`settings.jsonc` の lsp 設定を通じて言語サーバーに渡せます。

lsp の `settings` の完全な一覧は
[こちら](https://github.com/fwcd/kotlin-language-server/blob/main/server/src/main/kotlin/org/javacs/kt/Configuration.kt)
の `class Configuration` に、initialization_options は
`class InitializationOptions` にあります。

### JVM ターゲット

次の例は、JVM ターゲットを `default`（1.8）から `17` に変更します:

```json
{
  "lsp": {
    "kotlin-language-server": {
      "settings": {
        "compiler": {
          "jvm": {
            "target": "17",
          },
        },
      },
    },
  },
}
```

### JAVA_HOME

特定の Java インストールを使用するには、`JAVA_HOME` 環境変数を次のように指定します:

```json
{
  "lsp": {
    "kotlin-language-server": {
      "binary": {
        "env": {
          "JAVA_HOME": "/Users/whatever/Applications/Work/Android Studio.app/Contents/jbr/Contents/Home",
        },
      },
    },
  },
}
```
