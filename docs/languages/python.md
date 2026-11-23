# Gram での Python のセットアップ方法

Python のサポートは Gram にネイティブで組み込まれています。

- Tree-sitter:
  [tree-sitter-python](https://github.com/zed-industries/tree-sitter-python)
- 言語サーバー:
  - [DetachHead/basedpyright](https://github.com/DetachHead/basedpyright)
  - [astral-sh/ruff](https://github.com/astral-sh/ruff)
  - [astral-sh/ty](https://github.com/astral-sh/ty)
  - [microsoft/pyright](https://github.com/microsoft/pyright)
  - [python-lsp/python-lsp-server](https://github.com/python-lsp/python-lsp-server)
    (PyLSP)
- デバッグアダプター: [debugpy](https://github.com/microsoft/debugpy)

## Python のインストール

始める前に、Gram と Python の両方をインストールしておく必要があります。

### ステップ 1: Python のインストール

Gram には Python ランタイムが同梱されていないため、ご自身でインストールする必要があります。以下のいずれかの方法を選択してください:

- uv（推奨）

```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

詳細については、
[Astral のインストールガイド](https://docs.astral.sh/uv/getting-started/installation/)
を参照してください。

- Homebrew:

```bash
brew install python
```

- Python.org インストーラー:
  [python.org/downloads](https://python.org/downloads) から最新バージョンをダウンロードします。

### ステップ 2: Python インストールの確認

シェルで Python がインストールされ利用可能であることを確認します:

```bash
python3 --version
```

`Python 3.x.x` のような出力が表示されるはずです。

## Gram で最初の Python プロジェクトを開く

Gram と Python をインストールしたら、作業を開始するために Python コードを含むフォルダーを開きます。

### ステップ 1: Python プロジェクトで Gram を起動する

Gram を開きます。メニューバーから「ファイル > フォルダーを開く」を選択するか、ターミナルから起動します:

```bash
gram path/to/your/project
```

Gram はネイティブの tree-sitter-python パーサーを使用して `.py` ファイルを自動的に認識するため、プラグインや手動設定は不要です。

### ステップ 2: 統合ターミナルの使用（任意）

Gram には、下部パネルからアクセスできる統合ターミナルが含まれています。プロジェクトが
[仮想環境](#virtual-environments) を使用していることを Gram が検出した場合、新しく作成されたターミナルで自動的にアクティブ化されます。この動作は
[`detect_venv`](../configuring-gram.md#terminal-detect_venv) 設定で構成できます。

## Gram での Python 言語サーバーの設定

Gram は複数の Python 言語サーバーを初期状態で提供しています。既定では、
[basedpyright](https://github.com/DetachHead/basedpyright) がプライマリ言語サーバーとして使用され、フォーマットとリントには [Ruff](https://github.com/astral-sh/ruff) が使用されます。

その他の組み込み言語サーバーは次のとおりです:

- [Ty](https://docs.astral.sh/ty/): 速度を重視して構築された Astral 製の先進的な言語サーバー。
- [Pyright](https://github.com/microsoft/pyright): basedpyright の基礎。
- [PyLSP](https://github.com/python-lsp/python-lsp-server): `pycodestyle`、`autopep8`、`yapf` などのツールと統合するプラグインベースの言語サーバー。

これらは既定では無効になっていますが、設定で有効にすることができます。例:

```json
{
  "languages": {
    "Python": {
      "language_servers": [
        // basedpyright を無効化して Ty を有効化し、
        // それ以外は既定の設定を使用します。
        "ty",
        "!basedpyright",
        "...",
      ],
    },
  },
}
```

言語サーバーを有効または無効にする方法の詳細については、
[言語サーバーの操作](../configuring-languages.md#working-with-language-servers)
を参照してください。

### Basedpyright

[basedpyright](https://docs.basedpyright.com/latest/) は、Gram v0.204.0 以降の Gram におけるプライマリ Python 言語サーバーです。定義へ移動やすべての参照の検索などのナビゲーション機能や型チェックなど、コアとなる言語サーバー機能を提供します。Pyright と比較して、追加の言語サーバー機能（インレイヒントなど）やチェックルールのサポートが追加されています。

basedpyright 単体では既定で `recommended` の
[型チェックモード](https://docs.basedpyright.com/latest/benefits-over-pyright/better-defaults/#typecheckingmode)
になりますが、Gram では既定でそれより緩やかな `standard` モード（Pyright の動作と一致）を使用するよう設定されています。`pyrightconfig.json` または `pyproject.toml` の `typeCheckingMode` 設定を使用してプロジェクト固有の型チェックモードを設定でき、Gram の既定値が上書きされます。basedpyright の設定方法の詳細については以下をお読みください。

#### Basedpyright の設定

basedpyright は、次の 2 種類のソースから設定オプションを読み込みます:

- 言語サーバー設定（「ワークスペース設定」）: エディターごとに設定する必要があり（Gram では `settings.json` を使用）、そのエディターで開かれたすべてのプロジェクトに適用されます
- 設定ファイル（`pyrightconfig.json`、`pyproject.toml`）: エディターに依存しませんが、配置されたプロジェクト固有の設定となります

大まかな目安として、エディターから basedpyright を使用する場合にのみ関係するオプションは言語サーバー設定で指定し、
[コマンドラインツールとして](https://docs.basedpyright.com/latest/configuration/command-line/)
実行する場合でも関係するオプションは設定ファイルで指定する必要があります。インレイヒントに関する設定は前者の例であり、
[診断カテゴリ](https://docs.basedpyright.com/latest/configuration/config-files/#diagnostic-categories)
の設定は後者の例です。

両方の設定例を以下に示します。利用可能なオプションの完全な一覧については、
[言語サーバー設定](https://docs.basedpyright.com/latest/configuration/language-server-settings/)
および
[設定ファイル](https://docs.basedpyright.com/latest/configuration/config-files/)
に関する basedpyright のドキュメントを参照してください。

##### 言語サーバー設定

Gram における basedpyright の言語サーバー設定は、`settings.json` の `lsp` セクションで指定できます。

たとえば次のように設定できます:

- 開いているファイルのみという既定の動作ではなく、ワークスペース内のすべてのファイルを診断する
- 関数の引数に対するインレイヒントを無効にする

次の設定を使用します:

```json
{
  "lsp": {
    "basedpyright": {
      "settings": {
        "basedpyright.analysis": {
          "diagnosticMode": "workspace",
          "inlayHints": {
            "callArgumentNames": false,
          },
        },
      },
    },
  },
}
```

##### 設定ファイル

basedpyright は、`pyrightconfig.json` 設定ファイル、および `pyproject.toml` マニフェストの `[tool.basedpyright]` と `[tool.pyright]` セクションからプロジェクト固有の設定を読み込みます。両方に設定が存在する場合、`pyrightconfig.json` が `pyproject.toml` よりも優先されます。

以下は、basedpyright に `strict` 型チェックモードを使用させ、`__pycache__` ディレクトリ内のファイルに対して診断を発行しないように設定する `pyrightconfig.json` ファイルの例です:

```json
{
  "typeCheckingMode": "strict",
  "ignore": ["**/__pycache__"],
}
```

### PyLSP

一般に PyLSP として知られる [python-lsp-server](https://github.com/python-lsp/python-lsp-server/) は、既定で多数の外部ツール（autopep8、mccabe、pycodestyle、yapf）と統合されており、その他のツール（flake8、pylint）は任意で明示的に有効化・設定する必要があります。

詳細については、
[Python Language Server Configuration](https://github.com/python-lsp/python-lsp-server/blob/develop/CONFIGURATION.md)
を参照してください。

<a id="virtual-environments"></a>
## 仮想環境

[仮想環境](https://docs.python.org/3/library/venv.html) は、同一マシン上の他のプロジェクトから隔離された形で、特定のプロジェクトに対して Python バージョンと依存関係のセットを固定するための有用なツールです。Gram には、言語に依存しない
[ツールチェーン](../toolchains.md)
の概念に基づいて、仮想環境の検出、設定、アクティブ化のサポートが組み込まれています。

なお、グローバルな Python インストール環境が存在する場合、それも Gram の用途上はツールチェーンとしてカウントされます。

### 仮想環境の作成

プロジェクトにまだ仮想環境がセットアップされていない場合は、次のように作成できます:

```bash
python3 -m venv .venv
```

あるいは `uv` を使用している場合、初回実行時に `uv sync` を実行すると仮想環境が作成されます。

### Gram による Python ツールチェーンの使用方法

Gram は、プロジェクトに対して選択された Python ツールチェーンを次のように使用します:

- 組み込み言語サーバーには、ツールチェーンの Python インタープリターへのパス、および該当する場合は仮想環境へのパスが自動的に設定されます。これは依存関係を解決するために重要です。（現在、拡張機能によって提供される言語サーバーはこのように自動設定することはできません。）
- Python タスク（pytest テストなど）は、ツールチェーンの Python インタープリターを使用して実行されます。
- ツールチェーンが仮想環境である場合、Gram の統合ターミナルで新しいシェルを起動すると環境のアクティブ化スクリプトが自動的に実行され、選択した Python インタープリターと依存関係セットに便利にアクセスできます。
- アクティブな仮想環境内に組み込み言語サーバーがインストールされている場合、Gram がプライベートに自動インストールしたバイナリではなく、そのバイナリが使用されます。これは debugpy にも適用されます。

### ツールチェーンの選択

ほとんどのプロジェクトでは、Gram が適切な Python ツールチェーンを自動的に選択します。複数の仮想環境を持つ複雑なプロジェクトでは、この選択をオーバーライドする必要がある場合があります。Gram が検出した一覧からツールチェーンを選択するには
[ツールチェーンセレクター](../toolchains.md#selecting-toolchains)
を使用し、一覧にない場合は
[ツールチェーンのパスを手動で指定](../toolchains.md#adding-toolchains-manually)
できます。

## コードフォーマットとリント

Gram は、Python コード向けに [Ruff](https://docs.astral.sh/ruff/) フォーマッターとリンターを提供します。（具体的には、Gram は `ruff server` サブコマンドを使用して Ruff を LSP サーバーとして実行します。）フォーマット時保存を含め、フォーマットとリントの両方が既定で有効になっています。

### フォーマットの設定

Python ファイルのフォーマット時保存は、`settings.json` で無効にすることができます:

```json
{
  "languages": {
    "Python": {
      "format_on_save": "off",
    },
  },
}
```

あるいは、リント用に Ruff を有効にしたまま、Python のフォーマットに `black` コマンドラインツールを使用することもできます:

```json
{
  "languages": {
    "Python": {
      "formatter": {
        "external": {
          "command": "black",
          "arguments": ["--stdin-filename", "{buffer_path}", "-"],
        },
      },
      // または `"formatter": null` でフォーマットを完全に無効化します。
    },
  },
}
```

### Ruff の設定

basedpyright と同様に、Gram で使用する場合、Ruff は Gram の言語サーバー設定と設定ファイル（`ruff.toml`）の両方からオプションを読み込みます。basedpyright とは異なり、_すべての_ オプションをこれらいずれの場所でも設定できるため、Ruff 設定をどこに記述するかは、プロジェクト間で共有しつつ Gram 固有にしたいか（その場合は言語サーバー設定を使用）、あるいは 1 つのプロジェクト固有でありつつすべての Ruff 呼び出しで共通にしたいか（その場合は `ruff.toml` を使用）によって決まります。

以下は、Gram の `settings.json` の言語サーバー設定を使用して、（Ruff をフォーマッターとして使い続けながら）Gram 内のすべての Ruff リントを無効にする例です:

```json
{
  "lsp": {
    "ruff": {
      "initialization_options": {
        "settings": {
          "exclude": ["*"],
        },
      },
    },
  },
}
```

そして以下は、Ruff のドキュメントから改編した、リントとフォーマットのオプションを含む `ruff.toml` の例です:

```toml
[lint]
# 行長違反の適用を回避 (`E501`)
ignore = ["E501"]

[format]
# フォーマット時にシングルクォートを使用。
quote-style = "single"
```

詳細については、
[設定ファイル](https://docs.astral.sh/ruff/configuration/)
および
[言語サーバー設定](https://docs.astral.sh/ruff/editors/settings/)
に関する Ruff のドキュメント、ならびに
[オプション一覧](https://docs.astral.sh/ruff/settings/)
を参照してください。

### 埋め込み言語のハイライト

Gram は、言語名を記載したコメントを追加することで、Python 文字列内に埋め込まれたコードの構文ハイライトをサポートします。

```python
# sql
query = "SELECT * FROM users"

#sql
query = """
    SELECT *
    FROM users
"""

result = func( #sql
    "SELECT * FROM users"
)
```

## デバッグ

Gram は `debugpy` アダプターを介した Python デバッグをサポートしています。設定なしで開始することも、`.gram/debug.json` でカスタム起動プロファイルを定義することもできます。

### 設定なしでデバッグを開始する

Gram はデバッグ可能な Python エントリポイントを自動的に検出できます。F4 を押すか（またはコマンドパレットから `debugger: start` を実行）、現在のプロジェクトで利用可能なオプションを表示します。これは以下に対して機能します:

- Python スクリプト
- モジュール
- pytest テスト

Gram は内部で `debugpy` を使用しますが、手動でのアダプター設定は不要です。

### カスタムデバッグ設定の定義

再利用可能なセットアップを行うには、プロジェクトルートに `.gram/debug.jsonc` ファイルを作成します。これにより、Gram がコードを実行およびデバッグする方法をより細かく制御できます。

- [debugpy 設定ドキュメント](https://github.com/microsoft/debugpy/wiki/Debug-configuration-settings#launchattach-settings)

#### アクティブなファイルのデバッグ

```json [debug]
[
  {
    "label": "Python Active File",
    "adapter": "Debugpy",
    "program": "$GRAM_FILE",
    "request": "launch"
  }
]
```

これにより、エディターで現在開いているファイルが実行されます。

#### Flask アプリのデバッグ

Flask を使用するプロジェクトでは、完全な起動設定を定義できます:

```
.venv/
app/
  init.py
  main.py
  routes.py
templates/
  index.html
static/
  style.css
requirements.txt
```

…以下の設定を使用できます:

```json [debug]
[
  {
    "label": "Python: Flask",
    "adapter": "Debugpy",
    "request": "launch",
    "module": "app",
    "cwd": "$GRAM_WORKTREE_ROOT",
    "env": {
      "FLASK_APP": "app",
      "FLASK_DEBUG": "1"
    },
    "args": [
      "run",
      "--reload", // ファイルの変更を監視する Flask のリローダーを有効化
      "--debugger" // Flask デバッガーを有効化
    ],
    "autoReload": {
      "enable": true
    },
    "jinja": true,
    "justMyCode": true
  }
]
```

これらを組み合わせて、Web サーバー、テストランナー、またはカスタムスクリプトに合わせてエクスペリエンスを調整できます。

## 生産的な Python 環境のトラブルシューティングと維持

Gram は設定のオーバーヘッドを最小限に抑えるよう設計されていますが、特に環境、言語サーバー、ツールに関連して問題が発生することがあります。ここでは、Python のセットアップをスムーズに維持する方法を説明します。

### 言語サーバー起動問題の解決

言語サーバーが応答しない場合や、診断や自動補完などの機能が利用できない場合:

- 使用しようとしている言語サーバーに関するエラーがないか、Gram のログ（{#action gram::OpenLog} アクションを使用）を確認してください。言語サーバーが起動に失敗した場合、ここに有用な情報が見つかる可能性が高いです。
- 言語サーバーログビューを使用して、影響を受けている言語サーバーのライフサイクルを確認します。このビューには {#action
  dev::OpenLanguageServerLogs} アクションを使用するか、ステータスバーの稲妻アイコンをクリックして言語サーバーを選択することでアクセスできます。このビューで最も有用なデータは次のとおりです:
  - 「Server Logs」: 言語サーバーによって出力されたエラーを表示します
  - 「Server Info」: 言語サーバーがどのように起動されたかに関する詳細を表示します
- `settings.json` または `pyrightconfig.json` の構文が正しいことを確認します。
- Gram を再起動して言語サーバーの接続を再初期化するか、{#action editor::RestartLanguageServer} アクションを使用して言語サーバーを再起動してみてください。

言語サーバーがインポートの解決に失敗しており、仮想環境を使用している場合は、セレクターで正しい環境が選択されていることを確認してください。「Server Info」ビューを使用して、Gram が言語サーバーに送信している仮想環境を確認できます。末尾の `* Configuration` セクションを探してください。
