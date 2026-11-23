# Java

Gram での Java 言語サポートは、次のものによって提供されます:

- Gram Java: [zed-extensions/java](https://github.com/zed-extensions/java)
- Tree-sitter:
  [tree-sitter/tree-sitter-java](https://github.com/tree-sitter/tree-sitter-java)
- 言語サーバー:
  [eclipse-jdtls/eclipse.jdt.ls](https://github.com/eclipse-jdtls/eclipse.jdt.ls)

## OpenJDK のインストール

Java ランタイム（OpenJDK）をインストールする必要があります。

- macOS: `brew install openjdk`
- Ubuntu:
  `sudo add-apt-repository ppa:openjdk-23 && sudo apt-get install openjdk-23`
- Windows: `choco install openjdk`
- Arch Linux: `sudo pacman -S jre-openjdk-headless`

または [OpenJDK 23](https://jdk.java.net/23/) を手動でダウンロードしてインストールします。

## 拡張機能のインストール

{#action gram::Extensions}（{#kb gram::Extensions}）を開き、`java` を検索すると
インストールできます。

## クイックスタートと構成

ほとんどのユーザーにとって、Java のサポートは追加設定なしで動作します。

- 一般的には、Gram プロジェクトのルートを Java プロジェクトのルートフォルダ
  （通常は `pom.xml` や `build.gradle` がある場所）にしてプロジェクトを開くことを
  おすすめします。

- 既定では、拡張機能が最新の公式 JDTLS をダウンロードして実行します。ただし、そのためには
  Java 21 が `$JAVA_HOME` 環境変数で、または `$PATH` 上の `java(.exe)` 実行ファイルとして
  利用できる必要があります。プロジェクトがより低い Java バージョンを必要とする場合は、
  `java_home` 構成オプションで JDTLS の実行に使う JDK を別に指定できます。

- `$PATH` 環境変数に `jdtls`（Windows では `jdtls.bat`）という名前の実行ファイルを追加
  すると、**JDTLS 用のカスタム起動スクリプト**を利用できます。これがある場合、拡張機能は
  管理対象インスタンスのダウンロードと起動を省略し、環境にあるものを使用します。

- [Lombok](https://projectlombok.org/) をサポートするには、JDTLS の起動時に lombok-jar を
  ダウンロードし、Java エージェントとして登録する必要があります。既定では拡張機能が自動で
  処理しますが、不要な場合は `lombok_support` 構成オプションを `false` に設定できます。

上記の構成を含む一般的な `settings.jsonc` は次のとおりです:

```json
{
  "lsp": {
    "jdtls": {
      "settings": {
        "java_home": "/path/to/your/JDK21+",
        "lombok_support": true,
      },
    },
  },
}
```

## デバッグ

デバッグサポートは、[Java Debug のフォーク](https://github.com/zed-industries/java-debug)
によって有効になり、拡張機能が自動でダウンロードして起動します。Gram でのデバッグの
一般的な仕組みは、[デバッガーのドキュメント](../debugger.md#getting-started) を参照して
ください。

Java でデバッグを始めるには、デバッグメニューの `edit debug.jsonc` ボタンをクリックし、
ファイルの内容を次に置き換えます:

```json
[
  {
    "adapter": "Java",
    "request": "launch",
    "label": "Launch Debugger",
    // プロジェクトに複数のエントリーポイントがある場合は、使用するものを指定します:
    // "mainClass": "com.myorganization.myproject.MyMainClass",
    //
    // これにより、実質的にプログラムのエントリーポイントにブレークポイントを設定します:
    "stopOnEntry": true,
    // デバッグプロセスの作業ディレクトリ
    "cwd": "$GRAM_WORKTREE_ROOT",
  },
]
```

これで、デバッグメニューの "Launch Debugger" シナリオから新しいデバッグセッションを
開始できるようになります。

## Windows での起動スクリプト（タスク）

この拡張機能は、テストやエントリーポイントの横に表示される小さな再生ボタンから、Gram 内で
アプリケーションやテストを実行するためのタスクを提供します。ただし、Gram の拡張機能
インターフェースの現在の制限により、Windows と Unix 系システムの両方で Maven と Gradle に
対応するスクリプトは提供できず、既定では起動スクリプトは Mac と Linux でのみ動作します。

Windows でも動作させるための比較的簡単な修正方法があります。独自のタスクスクリプトを用意
してください。方法は[こちらの Issue](https://github.com/zed-extensions/java/issues/94) を、
詳しくは [Gram ドキュメントのタスクの節](../tasks.md) を参照してください。

## 高度な構成と JDTLS の初期化オプション

JDTLS は、`initialize` LSP リクエストで渡せる多くの構成オプションを提供します。拡張機能は、
設定の `lsp.jdtls.settings.initialization_options` にある JSON オブジェクトを JDTLS に
渡します。利用できるオプションと値は
[JDTLS の構成 Wiki ページ](https://github.com/eclipse-jdtls/eclipse.jdt.ls/wiki/Running-the-JAVA-LS-server-from-the-command-line#initialize-request)
を参照してください。以下は、上記 Wiki ページの構成例を JDTLS に渡す `settings.jsonc` の
例です:

```json
{
  "lsp": {
    "jdtls": {
      "settings": {
        // これは JDTLS に initializationOptions として送信されます:
        "initialization_options": {
          "bundles": [],
          // Gram プロジェクトのルートフォルダが Java プロジェクトのルートと異なる場合に使用します:
          "workspaceFolders": ["file:///home/snjeza/Project"],
          "settings": {
            "java": {
              "home": "/usr/local/jdk-9.0.1",
              "errors": {
                "incompleteClasspath": {
                  "severity": "warning",
                },
              },
              "configuration": {
                "updateBuildConfiguration": "interactive",
                "maven": {
                  "userSettings": null,
                },
              },
              "import": {
                "gradle": {
                  "enabled": true,
                },
                "maven": {
                  "enabled": true,
                },
                "exclusions": [
                  "**/node_modules/**",
                  "**/.metadata/**",
                  "**/archetype-resources/**",
                  "**/META-INF/maven/**",
                  "/**/test/**",
                ],
              },
              "referencesCodeLens": {
                "enabled": false,
              },
              "signatureHelp": {
                "enabled": false,
              },
              "implementationCodeLens": "all",
              "format": {
                "enabled": true,
              },
              "saveActions": {
                "organizeImports": false,
              },
              "contentProvider": {
                "preferred": null,
              },
              "autobuild": {
                "enabled": false,
              },
              "completion": {
                "favoriteStaticMembers": [
                  "org.junit.Assert.*",
                  "org.junit.Assume.*",
                  "org.junit.jupiter.api.Assertions.*",
                  "org.junit.jupiter.api.Assumptions.*",
                  "org.junit.jupiter.api.DynamicContainer.*",
                  "org.junit.jupiter.api.DynamicTest.*",
                ],
                "importOrder": ["java", "javax", "com", "org"],
              },
            },
          },
        },
      },
    },
  },
}
```

## 関連項目

[Gram Java リポジトリ](https://github.com/zed-extensions/java)
[Eclipse JDTLS リポジトリ](https://github.com/eclipse-jdtls/eclipse.jdt.ls)
