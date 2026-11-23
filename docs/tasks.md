# タスク

Gram は、統合ターミナルを使ってコマンドを起動（および再実行）し、結果を出力する方法をサポートしています。これらのコマンドは、Gram の状態の限定されたサブセット（編集中のファイルへのパスや選択中のテキストなど）を読み取れます。

```json [tasks]
[
  {
    "label": "Example task",
    "command": "for i in {1..5}; do echo \"Hello $i/5\"; sleep 1; done",
    // "args": [],
    // コマンド用の環境変数の上書き。設定のターミナル環境に追加されます。
    "env": { "foo": "bar" },
    // コマンドを起動する作業ディレクトリ。既定値は現在のプロジェクトルートです。
    // "cwd": "/path/to/working/directory",
    // プロセスの起動に新しいターミナルタブを使うか、既存のタブを再利用するか。既定値は `false`。
    "use_new_terminal": false,
    // 同じタスクの複数インスタンスの同時実行を許可するか、既存の実行が終わるまで待つか。既定値は `false`。
    "allow_concurrent_runs": false,
    // コマンド開始後、ターミナルペインとタブをどう扱うか:
    // * `always` — 常にタスクのペインを表示し、その中の対応するタブにフォーカスする（既定値）
    // * `no_focus` — 常にタスクのペインを表示し、そのタブを追加するがフォーカスはしない
    // * `never` — フォーカスは変更せず、タスクのタブをペイン内に追加/再利用する
    "reveal": "always",
    // コマンド終了後、ターミナルペインとタブをどう扱うか:
    // * `never` — コマンド終了時に何もしない（既定値）
    // * `always` — 常にターミナルタブを隠し、それがペイン内の最後のタブだった場合はペインも隠す
    // * `on_success` — タスク成功時のみターミナルタブを隠し、それ以外は `always` と同様に動作する
    "hide": "never",
    // ターミナル内でタスクを実行するときに使うシェル。
    // 3 つの値を指定できます:
    // 1.（既定値） /etc/passwd にあるシステム既定のターミナル構成を使う
    //      "shell": "system"
    // 2. プログラムを指定する:
    //      "shell": {
    //        "program": "sh"
    //      }
    // 3. 引数付きのプログラムを指定する:
    //     "shell": {
    //         "with_arguments": {
    //           "program": "/bin/bash",
    //           "args": ["--login"]
    //         }
    //     }
    "shell": "system",
    // 起動したタスクの出力にタスクの行を表示するか。既定値は `true`。
    "show_summary": true,
    // 起動したタスクの出力にコマンドラインを表示するか。既定値は `true`。
    "show_command": true,
    // タスク実行前に保存する編集済みバッファー:
    // * `none` — どのバッファーも保存しない（既定値）
    // * `all` — 編集済みのすべてのバッファーを保存する
    // * `current` — 現在のバッファーのみ保存する
    "save": "none",
    // インラインの実行可能インジケーター用タグ、または複数タスクの一括起動に使います。
    // "tags": []
  }
]
```

タスクを使うワークフローを支えるアクションは 2 つあります: `task: spawn` と `task: rerun` です。`task: spawn` は現在のファイルで利用可能なすべてのタスクを表示するモーダルを開きます。`task: rerun` は直近に起動したタスクを再実行します。タスクモーダルからもタスクを再実行できます。

既定では、タスクの再実行は同じターミナルを再利用します（`"use_new_terminal": false` が既定値のため）。その代わり、前のタスクが終了するまで待ってから開始します（`"allow_concurrent_runs": false` が既定値のため）。

`"use_new_terminal": false` を維持したまま `"allow_concurrent_runs": true` を設定すると、再実行時に前のタスクをキャンセルできるようになります。

## タスクテンプレート

タスクは次の場所で定義できます:

- グローバルの `tasks.jsonc` ファイル。ここで定義したタスクは、作業するすべての Gram プロジェクトで利用できます。このファイルは通常 `~/.config/gram/tasks.json` にあります。`gram: open tasks` アクションで編集できます。
- ワークツリー固有（ローカル）の `.gram/tasks.jsonc` ファイル。ここで定義したタスクは、そのワークツリーを含むプロジェクトで作業するときのみ利用できます。ワークツリー固有のタスクは `gram: open project tasks` アクションで編集できます。
- [ワンショットタスク](#oneshot-tasks)としてその場で。これらのタスクはプロジェクト固有で、セッションをまたいで永続化されません。
- 言語拡張機能によって。

## 変数

Gram のタスクはシェルと同じように動作します。つまり、sh 風の `$VAR_NAME` 構文で環境変数を参照できます。利便性のため、いくつかの追加の環境変数も設定されています。これらの変数を使うと、現在のエディターから情報を取得してタスク内で利用できます。次の変数が利用可能です:

- `GRAM_COLUMN`: 現在の行の列
- `GRAM_ROW`: 現在の行の行番号
- `GRAM_FILE`: 現在開いているファイルの絶対パス（例: `/Users/my-user/path/to/project/src/main.rs`）
- `GRAM_FILENAME`: 現在開いているファイルのファイル名（例: `main.rs`）
- `GRAM_DIRNAME`: 現在開いているファイルの絶対パスからファイル名を除いたもの（例: `/Users/my-user/path/to/project/src`）
- `GRAM_RELATIVE_FILE`: `GRAM_WORKTREE_ROOT` からの相対パスで表した、現在開いているファイルのパス（例: `src/main.rs`）
- `GRAM_RELATIVE_DIR`: `GRAM_WORKTREE_ROOT` からの相対パスで表した、現在開いているファイルのディレクトリのパス（例: `src`）
- `GRAM_STEM`: 現在開いているファイルのステム（拡張子を除いたファイル名）（例: `main`）
- `GRAM_SYMBOL`: 現在選択中のシンボル。シンボルのブレッドクラムに最後に表示されたシンボルと一致するはずです（例: `mod tests > fn test_task_contexts`）
- `GRAM_SELECTED_TEXT`: 現在選択中のテキスト
- `GRAM_WORKTREE_ROOT`: 現在のワークツリーのルートへの絶対パス（例: `/Users/my-user/path/to/project`）
- `GRAM_CUSTOM_RUST_PACKAGE`:（Rust 固有）$GRAM_FILE のソースファイルが属する親パッケージの名前。

タスク内で変数を使うには、ドル記号（`$`）を前に付けます:

```json
{
  "label": "echo current file's path",
  "command": "echo $GRAM_FILE",
}
```

与えられた変数が利用できない場合の既定値を指定できる、冗長な構文も使えます: `${GRAM_FILE:default_value}`

これらの環境変数は、タスクの `cwd`、`args`、`label` フィールドでも使えます。

### 変数のクォート

スペースやその他の特殊文字を含むパスを扱う場合は、変数が適切にエスケープされていることを確認してください。

たとえば、次の書き方（パスにスペースがあると失敗します）ではなく:

```json
{
  "label": "stat current file",
  "command": "stat $GRAM_FILE",
}
```

次のように記述してください:

```json
{
  "label": "stat current file",
  "command": "stat",
  "args": ["$GRAM_FILE"],
}
```

または、次のようにエスケープした引用符を明示的に含めます:

```json
{
  "label": "stat current file",
  "command": "stat \"$GRAM_FILE\"",
}
```

### 変数によるタスクのフィルタリング

タスク一覧を決める時点で存在しない変数を含むタスク定義は除外されます。たとえば、次のタスクはテキストが選択されている場合にのみ起動モーダルに表示されます:

```json
{
  "label": "selected text",
  "command": "echo \"$GRAM_SELECTED_TEXT\"",
}
```

そのような変数に既定値を設定すると、そのタスクは常に表示されます:

```json
{
  "label": "selected text with default",
  "command": "echo \"${GRAM_SELECTED_TEXT:no text selected}\"",
}
```

## ワンショットタスク

`task: spawn` で開くのと同じタスクモーダルは、任意の bash 風コマンドの実行もサポートしています。モーダルのテキストフィールドにコマンドを入力し、`opt-enter` で起動します。

タスクモーダルは、これらのアドホックなコマンドをセッションの間だけ保持します。直近に起動したタスクがこれらであれば、`task: rerun` でも再実行されます。

モーダルで現在選択中のタスクを調整することもできます（既定のキーバインドは `tab`）。こうするとそのコマンドがプロンプトに入り、編集してワンショットタスクとして起動できます。

### エフェメラルタスク

モーダル経由でタスクを起動するときに `cmd` 修飾キーを使えます。この方法で起動したタスクは使用回数が増えないため、`task: rerun` で再起動されず、タスクモーダルでの順位も高くなりません。エフェメラルタスクの想定用途は、`task: rerun` を継続的に使う流れを途切れさせないことです。

### タスク再実行の詳細な制御

既定では、タスクは一度だけ変数をコンテキストに取り込み、その「解決済みタスク」が常に再実行されます。

これはタスクの `"reevaluate_context"` 引数で制御できます。`true` に設定すると、実行のたびにタスクが再評価されます。

```json
{
  "context": "Workspace",
  "bindings": {
    "alt-t": ["task::Rerun", { "reevaluate_context": true }],
  },
}
```

## タスクのカスタムキーバインド

`task::Spawn` に追加の引数を渡すことで、タスク用の独自のキーバインドを定義できます。前述の `echo current file's path` タスクを `alt-g` に割り当てたい場合は、[`keymap.json`](./key-bindings.md) ファイルに次のスニペットを追加します:

```json
{
  "context": "Workspace",
  "bindings": {
    "alt-g": ["task::Spawn", { "task_name": "echo current file's path" }],
  },
}
```

これらのタスクには、起動したタスクをどこに表示するかを制御する「ターゲット」も指定できることに注意してください。中央エリアで使いたいターミナルアプリケーションを起動する場合に便利です:

```json [tasks]
// tasks.jsonc 内
{
  "label": "start lazygit",
  "command": "lazygit -p $GRAM_WORKTREE_ROOT"
}
```

```json
// keymap.jsonc 内
{
  "context": "Workspace",
  "bindings": {
    "alt-g": [
      "task::Spawn",
      { "task_name": "start lazygit", "reveal_target": "center" },
    ],
  },
}
```

## 実行可能タグをタスクテンプレートに割り当てる

Gram は、インラインの実行可能インジケーターの既定アクションを、ワークスペースローカルおよびグローバルの `tasks.jsonc` ファイルで上書きできます。優先順位は次のとおりです:

1. ワークスペースの `tasks.jsonc`
2. グローバルの `tasks.jsonc`
3. 言語が提供するタグの割り当て（既定）。

タスクにタグを付けるには、タスクテンプレートの `tags` フィールドに実行可能タグ名を追加します:

```json
{
  "label": "echo current file's path",
  "command": "echo $GRAM_FILE",
  "tags": ["rust-test"],
}
```

こうすることで、実行可能インジケーターに表示するタスクを変更できます。

## 実行可能項目に割り当てたタスクを実行するキーバインド

実行可能項目に割り当てたタスク定義がある場合は、[コードアクション](./configuring-languages?#code-actions.md) を使って素早く実行できます。コードアクションは `editor: Toggle Code Actions` コマンド、または `cmd-.`/`ctrl-.` ショートカットで呼び出せます。タスクはドロップダウンの先頭に表示されます。その行に追加のコードアクションがない場合、タスクは即座に実行されます。
