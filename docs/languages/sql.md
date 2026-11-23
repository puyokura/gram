# SQL

SQL のシンタックスハイライトとフォーマットのサポートは、エディターに組み込まれています。

- [Tree-sitter 文法](https://github.com/DerekStride/tree-sitter-sql)

### フォーマット

SQL の自動フォーマットは、[`sql-formatter`](https://github.com/sql-formatter-org/sql-formatter) などの外部ツールを使ってエディターに追加できます。

1. `sql-formatter` をインストールします:

```sh
npm install -g sql-formatter
```

2. `sql-formatter` がパス上にあることを確認し、バージョンを表示します:

```sh
which sql-formatter
sql-formatter --version
```

3. `sql-formatter` で SQL を自動フォーマットするよう Gram を設定します:

```json
  "languages": {
    "SQL": {
      "formatter": {
        "external": {
          "command": "sql-formatter",
          "arguments": ["--language", "mysql"]
        }
      }
    }
  },
```

上記の `mysql` を、好みの [SQL 方言]（`duckdb`、`hive`、
`mariadb`、`postgresql`、`redshift`、`snowflake`、`sqlite`、`spark` など）に置き換えてください。

これは Gram のプロジェクト設定（`.gram/settings.json`）に追加するか、Gram のユーザー設定（`~/.config/gram/settings.json`）で設定できます。

### 高度なフォーマット

sql-formatter では、[sql-formatter の設定オプション](https://github.com/sql-formatter-org/sql-formatter#configuration-options)を指定することで、より精密な制御も可能です。設定するには、プロジェクトに `.sql-formatter.json` ファイルを作成します:

```json
{
  "language": "postgresql",
  "tabWidth": 2,
  "keywordCase": "upper",
  "linesBetweenQueries": 2,
}
```

`.sql-formatter.json` ファイルを使用する場合、言語をインラインで指定する必要がないため、より簡素な Gram の設定で済みます:

```json
  "languages": {
    "SQL": {
      "formatter": {
        "external": {
          "command": "sql-formatter"
        }
      }
    }
  },
```
