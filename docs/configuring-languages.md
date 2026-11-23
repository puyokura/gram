# 対応言語の設定

言語サポート機能は、主に以下の 2 つの技術をベースに構築されています。

1. Tree-sitter: シンタックスハイライトやアウトラインパネルなどの構造ベースの機能を担当します。
2. Language Server Protocol (LSP): コード補完や診断などのセマンティック（意味論的）機能を提供します。

これらのコンポーネントが連携することで、高度な言語機能を実現しています。

## 目次

- 言語別の設定
- ファイルの関連付け
- 言語サーバーの操作
- フォーマットとリンティングの設定
- シンタックスハイライトとテーマのカスタマイズ
- 高度な言語機能

各言語固有の構成の詳細については [対応言語一覧](./languages.md) を参照してください。新しい言語のサポートを追加する方法については、[言語拡張機能の開発](./extensions/languages.md) ガイドを参照してください。

## 言語別の設定

個々の言語に対してグローバル設定を上書きすることが可能です。これらのカスタム設定は、`settings.jsonc` ファイルの `languages` キーの下に定義します。

言語別設定の例:

```json
"languages": {
  "Python": {
    "tab_size": 4,
    "formatter": "language_server",
    "format_on_save": "on"
  },
  "JavaScript": {
    "tab_size": 2,
    "formatter": {
      "external": {
        "command": "prettier",
        "arguments": ["--stdin-filepath", "{buffer_path}"]
      }
    }
  }
}
```

カスタマイズ可能な主な設定項目は以下のとおりです。

- [`tab_size`](./configuring-gram.md#tab-size): 各インデントレベルのスペース数
- [`formatter`](./configuring-gram.md#formatter): コードの整形に使用するツール
- [`format_on_save`](./configuring-gram.md#format-on-save): ファイル保存時に自動的にコードを整形するかどうか
- [`enable_language_server`](./configuring-gram.md#enable-language-server): 言語サーバーサポートの有効/無効の切り替え
- [`hard_tabs`](./configuring-gram.md#hard-tabs): インデントにスペースの代わりにタブを使用するかどうか
- [`preferred_line_length`](./configuring-gram.md#preferred-line-length): 推奨される最大行長
- [`soft_wrap`](./configuring-gram.md#soft-wrap): 長いコード行の折り返し方法
- [`show_completions_on_input`](./configuring-gram.md#show-completions-on-input): 入力中に補完候補を表示するかどうか
- [`show_completion_documentation`](./configuring-gram.md#show-completion-documentation): 補完メニューの項目にインラインおよび横並びのドキュメントを表示するかどうか
- [`colorize_brackets`](./configuring-gram.md#colorize-brackets): Tree-sitter の括弧クエリを使用してエディター内の括弧を検出・色分け表示（レインボーブラケット）するかどうか

## ファイルの関連付け

Gram は拡張子に基づいてファイルタイプを自動検出しますが、この関連付けをカスタマイズすることもできます。

独自のファイル関連付けを設定するには、`settings.jsonc` で [`file_types`](./configuring-gram.md#file-types) 設定を使用します。

```json
"file_types": {
  "C++": ["c"],
  "TOML": ["MyLockFile"],
  "Dockerfile": ["Dockerfile*"]
}
```

この設定により、Gram は以下のように動作します。

- `.c` ファイルを C ではなく C++ として扱う
- 「MyLockFile」という名前のファイルを TOML として認識する
- 「Dockerfile」で始まるすべてのファイルに Dockerfile 構文を適用する

より柔軟な一致判定のために Glob パターンを使用できるため、プロジェクト内の複雑な命名規則にも対応できます。

## 言語サーバーの操作

言語サーバーは、自動補完、定義への移動、リアルタイムのエラーチェックなどの機能を提供します。

### 言語サーバーとは

言語サーバーは Language Server Protocol (LSP) を実装しており、エディターと言語固有ツールとの間の通信を標準化します。これにより、エディター側で各言語の機能を個別に実装することなく、複数のプログラミング言語に対して高度な機能を提供できるようになります。

言語サーバーが提供する主な機能は以下のとおりです。

- コード補完
- エラーチェックおよび診断
- コードナビゲーション（定義への移動、すべての参照の検索）
- コードアクション（名前の変更、メソッドの抽出）
- ホバー情報
- ワークスペースシンボルの検索

### 言語サーバーの管理

Gram はユーザーにとって言語サーバーの管理が容易になるよう設計されています。

1. 推奨拡張機能: 既知のファイルタイプのファイルを開くと、関連する拡張機能のインストールを促すメッセージが表示される場合があります。インストールを正常に行うには、[rustup](https://rustup.rs) が正しく設定されている必要があります。

2. 保存場所:

   - macOS: `~/Library/Application Support/Gram/languages`
   - Linux: `$XDG_DATA_HOME/gram/languages`、
     `$FLATPAK_XDG_DATA_HOME/gram/languages`、または
     `$HOME/.local/share/gram/languages`

### 言語サーバーの選択

言語によっては、複数の言語サーバーの選択肢が存在する場合があります。同じ言語を対象とする言語サーバーを同梱した複数の拡張機能がインストールされている場合、機能の重複が生じる可能性があります。Gram では、どの言語サーバーをどのような順序で使用するか優先順位を設定できます。

優先順位は `language_servers` 設定で指定します。

```json
  "languages": {
    "PHP": {
      "language_servers": ["intelephense", "!phpactor", "!phptools", "..."]
    }
  }
```

この例では以下のようになります。

- `intelephense` がプライマリの言語サーバーとして設定されます
- `phpactor` は無効化されます（`!` プレフィックスに注目してください）
- `...` は、PHP 用に登録されている残りの言語サーバーに展開されます

### ツールチェーン

一部の言語サーバーでは、現在の「ツールチェーン」を設定する必要があります。ツールチェーンとは、プログラミング言語のコンパイラやインタプリタの特定バージョンのインストール環境であり、プロジェクトの完全な依存関係セットが含まれる場合もあります。

Gram におけるツールチェーンの一例として、Python の仮想環境（virtual environment）が挙げられます。

Gram のすべての言語がツールチェーンの検出と選択に対応しているわけではありませんが、対応している言語ではツールチェーンピッカー（{#action toolchain::Select} 経由）からツールチェーンを指定できます。ツールチェーンの詳細については、[`toolchains`](./toolchains.md) を参照してください。

### 言語サーバーの設定

言語サーバーのカスタム構成オプションは、`settings.jsonc` の `lsp` セクションで設定します。

```json
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "check": {
          "command": "clippy"
        }
      }
    }
  }
```

この例では、ファイル保存時により詳細なリンティングを行うために Clippy を使用するよう Rust Analyzer を構成しています。

#### ネストされたオブジェクト

言語サーバーのオプションを設定する際は、ドット区切りの文字列ではなく、ネストされたオブジェクトを使用することが重要です。これは、より複雑な構成を扱う場合に特に重要です。TypeScript 言語サーバーを使用した具体例を見てみましょう。

TypeScript に対して以下の設定を行いたいとします。

- 厳格な null チェック（strict null checks）を有効にする
- ターゲットの ECMAScript バージョンを ES2020 に設定する

`settings.jsonc` では、以下のように構造化して記述します。

```json
"lsp": {
  "typescript-language-server": {
    "initialization_options": {
      // 以下はサポートされていません（VSCode のドット記法スタイル）:
      // "preferences.strictNullChecks": true,
      // "preferences.target": "ES2020"
      //
      // 以下が正しい記法です（ネスト表記）:
      "preferences": {
        "strictNullChecks": true,
        "target": "ES2020"
      },
    }
  }
}
```

#### 設定可能なオプション

各言語サーバーの実装方法に応じて、LSP で規定された初期化オプションに依存する場合があります。

- [initializationOptions](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#version_3_17_0)

これらのオプションは言語サーバーの起動時に送信され、変更を適用するにはサーバーの再起動が必要です。

たとえば、rust-analyzer や clangd はこのように設定されます。

```json
  "lsp": {
    "rust-analyzer": {
      "initialization_options": {
        "checkOnSave": false
      }
    }
  }
```

- [Configuration Request](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#workspace_configuration)

ほとんどの言語サーバーは、再起動なしで更新可能な設定を使用しています。

```json
"lsp": {
  "tailwindcss-language-server": {
    "settings": {
      "tailwindCSS": {
        "emmetCompletions": true,
      },
    }
  }
}
```

一部の言語サーバーでは、言語サーバーバイナリの起動方法を設定できます。言語サーバーは自動的にダウンロードされるか、パス内で見つかった場合に起動されます。

TODO: これはオプトインであるべきです。

明示的に代替バイナリを指定したい場合は、設定で指定できます。

```json
  "lsp": {
    "rust-analyzer": {
      "binary": {
        // インターネットからバイナリを取得するか、ローカルでの検索を試みるか。
        "ignore_system_version": false,
        "path": "/path/to/langserver/bin",
        "arguments": ["--option", "value"],
        "env": {
          "FOO": "BAR"
        }
      }
    }
  }
```

### 言語サーバーの有効化と無効化

言語サーバーサポートは、グローバルまたは言語ごとに切り替えることができます。

```json
  "languages": {
    "Markdown": {
      "enable_language_server": false
    }
  }
```

これにより Markdown ファイル用の言語サーバーが無効化され、大規模なドキュメントプロジェクトでのパフォーマンス向上に役立ちます。これは `~/.config/gram/settings.jsonc` でグローバルに設定することも、プロジェクトディレクトリ内の `.gram/settings.jsonc` で設定することも可能です。

## フォーマットとリンティング

### フォーマッターの設定

Gram は組み込みフォーマッターと外部フォーマッターの両方に対応しています。詳細については [`formatter`](./configuring-gram.md#formatter) ドキュメントを参照してください。フォーマッターは `settings.jsonc` でグローバルまたは言語ごとに設定します。

```json
"languages": {
  "JavaScript": {
    "formatter": {
      "external": {
        "command": "prettier",
        "arguments": ["--stdin-filepath", "{buffer_path}"]
      }
    },
    "format_on_save": "on"
  },
  "Rust": {
    "formatter": "language_server",
    "format_on_save": "on"
  }
}
```

この例では、JavaScript には Prettier を、Rust には言語サーバーのフォーマッターを使用し、いずれも保存時にフォーマットするよう設定しています。

特定の言語でフォーマットを無効化するには、次のように設定します。

```json
"languages": {
  "Markdown": {
    "format_on_save": "off"
  }
}
```

### リンターのセットアップ

リンティングは通常、言語サーバーによって処理されます。

多くの言語サーバーでは、リンティングルールの構成が可能です。

```json
"lsp": {
  "eslint": {
    "settings": {
      "codeActionOnSave": {
        "rules": ["import/order"]
      }
    }
  }
}
```

この設定により、JavaScript ファイルの保存時に ESLint がインポート文を整理（organize imports）するようになります。

保存時にリンターによる自動修正を実行するには、以下のように設定します。

```json
"languages": {
  "JavaScript": {
    "formatter": {
      "code_action": "source.fixAll.eslint"
    }
  }
}
```

### フォーマットとリンティングの統合

以下は、JavaScript ファイルの保存時に ESLint によるリンティングと Prettier によるフォーマットを自動的に実行する設定例です。

```json
"languages": {
  "JavaScript": {
    "formatter": [
      {
        "code_action": "source.fixAll.eslint"
      },
      {
        "external": {
          "command": "prettier",
          "arguments": ["--stdin-filepath", "{buffer_path}"]
        }
      }
    ],
    "format_on_save": "on"
  }
}
```

### トラブルシューティング

フォーマットやリンティングで問題が発生した場合:

1. エディターのログファイルでエラーメッセージを確認します（コマンドパレットから `gram: open log` を実行）。
2. 外部ツール（フォーマッター、リンター）が正しくインストールされ、PATH に通っていることを確認します。
3. Gram の設定と各言語固有の設定ファイル（例: `.eslintrc`, `.prettierrc`）の両方で設定内容を確認します。

## シンタックスハイライトとテーマ

### シンタックスハイライトのカスタマイズ

Gram はシンタックスハイライトに Tree-sitter 文法を使用しています。デフォルトのハイライトは `theme_overrides` 設定で上書きできます。

以下の例では、コメントを斜体（イタリック）にし、文字列の色を変更します。

```json
"theme_overrides": {
  "One Dark": {
    "syntax": {
      "comment": {
        "font_style": "italic"
      },
      "string": {
        "color": "#00AA00"
      }
    }
  }
}
```

### テーマの選択とカスタマイズ

エディターのテーマを変更するには:

1. テーマセレクターを使用する（{#kb theme_selector::Toggle}）
2. または `settings.jsonc` で指定する:

```json
"theme": {
  "mode": "dark",
  "dark": "One Dark",
  "light": "GitHub Light"
}
```

`~/.config/gram/themes/` に JSON ファイルを作成することで、カスタムテーマを作成できます。Gram はこのディレクトリにあるすべてのテーマを自動的に検出して利用可能にします。

### テーマ拡張機能の使用

Gram はテーマ拡張機能をサポートしています。インストールしたいテーマの URL を確認し、拡張機能パネル（{#kb gram::Extensions}）からインストールします。

独自のテーマ拡張機能を作成する方法については、[テーマ拡張機能の開発](./extensions/themes.md) ガイドを参照してください。

## 言語サーバー機能の活用

### インレイヒント（Inlay Hints）

インレイヒントは、パラメータ名や推論された型などの追加情報をコード内にインラインで表示します。`settings.jsonc` でインレイヒントを設定します。

```json
"inlay_hints": {
  "enabled": true,
  "show_type_hints": true,
  "show_parameter_hints": true,
  "show_other_hints": true
}
```

言語別のインレイヒント設定については、各言語のドキュメントを参照してください。

### コードアクション

コードアクションは、クイック修正やリファクタリングの選択肢を提供します。`editor: Toggle Code Actions` コマンドを実行するか、アクションが利用可能なときにカーソルの横に表示される電球アイコンをクリックしてアクセスします。

### 定義および参照への移動

コードベース内を移動するには、以下のコマンドを使用します。

- `editor: Go to Definition` (<kbd>f12|f12</kbd>)
- `editor: Go to Type Definition` (<kbd>cmd-f12|ctrl-f12</kbd>)
- `editor: Find All References` (<kbd>shift-f12|shift-f12</kbd>)

### シンボルの名前変更（リネーム）

プロジェクト全体でシンボルの名前を変更するには:

1. シンボル上にカーソルを置きます
2. `editor: Rename Symbol` コマンドを実行します (<kbd>f2|f2</kbd>)
3. 新しい名前を入力して Enter キーを押します

これらの機能は、各言語の言語サーバーの機能に依存します。

複数のファイルにまたがるシンボル名を変更する場合、Gram はマルチバッファーでプレビューを開きます。変更を適用する前に、ここですべての変更点を確認できます。リネームを確定するには、マルチバッファーを保存するだけです。リネームを取りやめたい場合は、変更を元に戻すか、保存せずにマルチバッファーを閉じます。

### ホバー情報

`editor: Hover` コマンドを使用すると、カーソル位置のシンボルに関する情報が表示されます。多くの場合、型情報、ドキュメント、関連リソースへのリンクが含まれます。

### ワークスペースシンボルの検索

`workspace: Open Symbol` コマンドを使用すると、プロジェクト全体からシンボル（関数、クラス、変数）を検索できます。大規模なコードベース内を素早くナビゲートするのに役立ちます。

### コード補完

Gram は入力中にコード補完の候補を表示します。`editor: Show Completions` コマンドで手動で補完をトリガーすることも可能です。<kbd>tab|tab</kbd> または <kbd>enter|enter</kbd> を使用して候補を確定します。

### 診断

言語サーバーは、コーディング中にリアルタイムの診断情報（エラー、警告、ヒント）を提供します。`diagnostics: Toggle` コマンドを使用して、プロジェクトのすべての診断情報を確認できます。
