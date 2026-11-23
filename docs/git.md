# Git

コードエディター内で Git および Git ホスティングサービスと連携するための統合サポートが備わっています。

現在サポートされている機能の概要は以下のとおりです。

- コミット
- ステージング、プッシュ、プル、フェッチ
- プロジェクト Diff: すべての変更点を確認できるマルチバッファービュー
- バッファーおよびエディタースクロールバー内の Diff インジケーター
- 未ステージングの変更に対するエディター内インライン Diff 切り替えおよび変更の取り消し（revert）
- プロジェクトパネルでの Git ステータス表示
- ブランチの作成と切り替え
- Git blame の表示
- Git stash の pop、apply、drop、表示

## Git パネル

Git パネルでは、ワーキングツリーとステージングエリアの状態を一目で把握できます。

Git パネルを開くには、{#action git_panel::ToggleFocus} を実行するか、ステータスバーの Git アイコンをクリックします。

Git パネルには、アクティブなリポジトリとブランチ、変更されたファイル、各ファイルの現在のステージング状態など、プロジェクトの状態が表示されます。

Gram はプロジェクトリポジトリを監視しているため、エディター内外で行われたすべての変更が Git パネルに即座に反映されます。

### 設定

エディターでコミットメッセージを自動改行（ハードラップ）する方法は、「Git Commit」言語の `preferred-line-length` 設定で構成します。デフォルトは `72` ですが、`0` 以上の任意の文字数を設定できます。

Git パネルでは `soft_wrap` 設定も構成でき、Git パネルへの入力中におけるコミットメッセージの表示方法を調整できます。デフォルト設定は `editor_width` ですが、`none`、`preferred_line_length`、`bounded` も選択可能です。

Git diff を左右に並べて開くには、`git.split_diff` を有効にします。この設定のデフォルトは `false` です。

#### 設定例

```json
"languages": {
  "Git Commit": {
    "soft_wrap": "editor_width",
    "preferred_line_length": 72
  },
}
```

## プロジェクト Diff

プロジェクト Diff（{#kb git::Diff}）を開くと、現在のすべての変更を確認できます。コマンドパレットまたは Git パネルの {#action git::Diff} アクションからアクセスできます。

プロジェクト Diff に表示されるすべての変更は、他のマルチバッファーと同様に動作します。すべてファイルの編集可能な抜粋です。

### 左右並列（Side-by-side）Diff

`git.split_diff` を有効にすると、プロジェクト、コミット、ファイルの diff が左側に旧バージョン、右側に現行バージョンという形で並べて開かれます。一致する行は両ペイン間で揃えて配置され、挿入行や削除行は対応するテキストが存在しない側に斜線（ハッチング）付きのパディングが表示されます。スクロールや折りたたみ（fold）は両ペイン間で同期されます。

アクティブな diff 表示は、`editor::SplitDiff` および `editor::UnsplitDiff` アクションで切り替えることができます。現行バージョン側は引き続き編集可能であるため、ステージング、取り消し、保存は通常の diff ビューと同様に動作します。

たとえば、このビューをデフォルトで有効にするには、`settings.json` に以下を追加します。

```json
{
  "git": {
    "split_diff": true
  }
}
```

タブバー上のボタンをクリックするか、対応するキーバインドを押すことで、ハンク単位やファイル全体をステージング/アンステージングできます。

## フェッチ、プッシュ、プル

Git パネルにあるボタン、またはコマンドパレットのアクション（{#action git::Fetch}、{#action git::Push}、{#action git::Pull}）を使用して、Gram 内から Git リポジトリのフェッチ、プッシュ、プルを実行できます。

## ステージングのワークフロー

Gram には、プロジェクト Diff を使用する方法と、パネルを直接操作する方法の 2 つの主なステージングワークフローがあります。

### プロジェクト Diff を使用する場合

プロジェクト Diff ビューでは、各ハンクにフォーカスを当て、タブバーのボタンをクリックするかキーバインド {#action git::StageAndNext}（{#kb git::StageAndNext}）を使用して個別にステージングできます。

同様に、キーバインド {#action git::StageAll}（{#kb git::StageAll}）ですべてのハンクを同時にステージングし、直後に {#action git::Commit}（{#kb git::Commit}）でコミットすることも可能です。

### Git パネルを使用する場合

パネルからコミットメッセージを入力し、コミットボタンを押すか {#action git::Commit} を実行するだけです。これにより、追跡対象のすべてのファイル（エントリーのチェックボックスに `[·]` で表示）が自動的にステージングされ、コミットされます。

各エントリーのチェックボックスを使って個別にステージングすることもできます。すべての変更をステージングするには、パネル上部のボタンをクリックするか {#action git::StageAll} を実行します。

## コミット

エディター内から変更をコミットするには、以下の 2 つの方法があります。

1. Git パネル下部のボタンを使用します。{#kb git::Commit} を押すと、ステージング済みのすべての変更が直ちにコミットされます。
2. Git パネルのコミットテキストエリアにフォーカスがある状態で、{#action git::ExpandCommitEditor} アクションを呼び出すか、キーボードショートカット {#kb git::ExpandCommitEditor} を使用します。

### コミットの取り消し（Undo）

Gram でコミットすると、Git パネルのコミットテキストエリアの直下にバーが表示され、直近に送信されたコミットが確認できます。

そこにある「Uncommit」ボタンを使用すると、`git reset HEADˆ --soft` と同様のコマンドが実行されます。

### コミット行の長さ

デフォルトでは、コミット行の長さは `72` に設定されています。これは `settings.json` で構成可能です。

[設定](#configuration) セクションの `preferred-line-length` を参照してください。

## スタッシュ（Stash）

Git stash を使用すると、コミットしていない変更を一時的に退避させ、クリーンな作業ディレクトリに戻すことができます。仕掛かり中の作業をコミットすることなく、素早くブランチを切り替えたり最新の更新をプルしたりする際に便利です。

### スタッシュの作成

現在のすべての変更をスタッシュするには、{#action git::StashAll} アクションを使用します。これにより、ステージング済みと未ステージングの両方の変更が新しいスタッシュエントリーに保存され、クリーンな作業ディレクトリが作成されます。

### スタッシュの管理

Gram には、{#action git::ViewStash} からアクセスできる高機能なスタッシュピッカーが用意されています。スタッシュピッカーからは以下の操作が可能です。

- **スタッシュ一覧の確認**: 保存されているすべてのスタッシュを説明文やタイムスタンプとともに閲覧
- **Diff を開く**: 各スタッシュにどのような変更が保存されているかを正確に確認
- **スタッシュの適用（Apply）**: スタッシュエントリーをリストに残したまま、スタッシュの変更を作業ディレクトリに適用
- **スタッシュの取り出し（Pop）**: スタッシュの変更を適用し、リストからそのスタッシュエントリーを削除
- **スタッシュの破棄（Drop）**: 変更を適用せずに不要なスタッシュエントリーを削除

### クイックスタッシュ操作

より素早いワークフローのために、Gram は直近のスタッシュを操作する直接アクションを提供しています。

- **最新のスタッシュを適用**: {#action git::StashApply} を使用すると、最新のスタッシュをリストから削除せずに適用します
- **最新のスタッシュを取り出す**: {#action git::StashPop} を使用すると、最新のスタッシュを適用してリストから削除します

### スタッシュ Diff ビュー

Diff ビューで特定のスタッシュを表示しているときは、インターフェイスから以下の追加オプションを利用できます。

- 現在のスタッシュを作業ディレクトリに適用する
- 現在のスタッシュを取り出す（適用して削除する）
- 変更を適用せずにスタッシュを削除する

スタッシュ Diff ビューを開くには、スタッシュピッカーからスタッシュを選択し、{#action stash_picker::ShowStashItem}（{#kb stash_picker::ShowStashItem}）キーバインドを使用します。

## Git 連携

Gram は主要な Git ホスティングサービスと連携しており、Git のコミットハッシュや Issue、Pull Request、Merge Request への参照をクリック可能なリンクとして扱えます。

Gram は現在、以下のホスティング版へのリンクに対応しています。
[GitHub](https://github.com), [GitLab](https://gitlab.com),
[Bitbucket](https://bitbucket.org), [Sourcehut](https://sr.ht),
[Codeberg](https://codeberg.org)。

セルフホスト版の GitHub、GitLab、Bitbucket インスタンスについては、コミットハッシュやパーマリンクが自組織のドメインに解決されるよう、`git_hosting_providers` 設定に追加してください。

```json
{
  "git_hosting_providers": [
    {
      "provider": "gitlab",
      "name": "Corp GitLab",
      "base_url": "https://git.example.corp",
    },
  ],
}
```

## パーマリンクのコピー

Git ホスティングサービス上のコードスニペットへの恒久的なリンク（パーマリンク）を作成できます。

これらのリンクは、特定のコミットにおけるファイル内の特定の行や行範囲を共有するのに便利です。コマンドパレットから（`permalink` を検索）、`editor::CopyPermalinkToLine` や `editor::OpenPermalinkToLine` アクションへの [カスタムキーバインド](./key-bindings.md#custom-key-bindings) を作成して呼び出すか、エディターで行を選択した状態で右クリックして `Copy Permalink` を選択します。

## Diff ハンクのキーボードショートカット

変更のあるファイルを表示している際、Gram は詳細なレビューのために展開または折りたたみが可能な Diff ハンクを表示します。

- **すべての Diff ハンクを展開**: {#action editor::ExpandAllDiffHunks}（{#kb editor::ExpandAllDiffHunks}）
- **すべての Diff ハンクを折りたたむ**: `Escape` を押す（{#action editor::Cancel} に割り当て）
- **選択した Diff ハンクの展開/折りたたみを切り替え**: {#action editor::ToggleSelectedDiffHunks}（{#kb editor::ToggleSelectedDiffHunks}）
- **ハンク間を移動**: {#action editor::GoToHunk} および {#action editor::GoToPreviousHunk}

> **ヒント:** 展開されているすべての Diff ハンクを折りたたんで変更の全体像に戻るには、`Escape` キーを押すのが最も手軽です。

## アクションリファレンス

| アクション | キーバインド |
| ----------------------------------------- | ------------------------------------- |
| {#action git::Add} | {#kb git::Add} |
| {#action git::StageAll} | {#kb git::StageAll} |
| {#action git::UnstageAll} | {#kb git::UnstageAll} |
| {#action git::ToggleStaged} | {#kb git::ToggleStaged} |
| {#action git::StageAndNext} | {#kb git::StageAndNext} |
| {#action git::UnstageAndNext} | {#kb git::UnstageAndNext} |
| {#action git::Commit} | {#kb git::Commit} |
| {#action git::ExpandCommitEditor} | {#kb git::ExpandCommitEditor} |
| {#action git::Push} | {#kb git::Push} |
| {#action git::ForcePush} | {#kb git::ForcePush} |
| {#action git::Pull} | {#kb git::Pull} |
| {#action git::PullRebase} | {#kb git::PullRebase} |
| {#action git::Fetch} | {#kb git::Fetch} |
| {#action git::Diff} | {#kb git::Diff} |
| {#action git::Restore} | {#kb git::Restore} |
| {#action git::RestoreFile} | {#kb git::RestoreFile} |
| {#action git::Branch} | {#kb git::Branch} |
| {#action git::Switch} | {#kb git::Switch} |
| {#action git::CheckoutBranch} | {#kb git::CheckoutBranch} |
| {#action git::Blame} | {#kb git::Blame} |
| {#action git::StashAll} | {#kb git::StashAll} |
| {#action git::StashPop} | {#kb git::StashPop} |
| {#action git::StashApply} | {#kb git::StashApply} |
| {#action git::ViewStash} | {#kb git::ViewStash} |
| {#action editor::ToggleGitBlameInline} | {#kb editor::ToggleGitBlameInline} |
| {#action editor::ExpandAllDiffHunks} | {#kb editor::ExpandAllDiffHunks} |
| {#action editor::ToggleSelectedDiffHunks} | {#kb editor::ToggleSelectedDiffHunks} |

> すべてのアクションにデフォルトのキーバインドが設定されているわけではありませんが、[キーマップのカスタマイズ](./key-bindings.md#user-keymaps) により割り当てることができます。

## Git CLI 設定

コマンドラインからコミットする際に Gram を [Git コミットメッセージエディター](https://git-scm.com/book/en/v2/Customizing-Git-Git-Configuration#_core_editor) としても使用したい場合は、`gram --wait` を使用できます。

```sh
git config --global core.editor "gram --wait"
```

または、シェルの環境設定（`~/.zshrc` や `~/.bashrc` など）に以下を追加します。

```sh
export GIT_EDITOR="gram --wait"
```
