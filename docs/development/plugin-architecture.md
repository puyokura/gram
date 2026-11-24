# Gram プラグイン対応化アーキテクチャ設計書

本書は、Gram 日本語版フォーク（`puyokura/gram`）におけるプラグイン対応化の全体設計、制約、段階的実装計画を定める。

## 1. 設計原則と非目標

### 1.1 中核原則

1. **日本語 UI ファースト（`ja` 既定）とプラグインによる言語切替の両立**
   - 現行 Gram の日本語 UI を既定として維持しつつ、再ビルド不要で言語パックプラグインから UI 言語を切替・上書き可能にする。
   - メモリ上に展開する翻訳テーブルは「選択中ロケール 1 つ ＋ フォールバック（`en`）」のみとし、未使用ロケールは保持しない。
2. **常駐負荷最小（ゼロコスト抽象化）**
   - 静的リソースのみのプラグイン（テーマ・アイコンテーマ・スニペット・キーバインド・言語パック）は導入時または起動時に既存レジストリへ直接変換ロードし、常駐プロセス・常駐ランタイムを一切起動しない（追加メモリ 0 MB、Node.js 不要）。
   - 動的プラグイン（Wasm / Node.js）は実際に有効化され、発火条件を満たしたときだけオンデマンド起動する。
3. **GPUI メインスレッドおよび内蔵ターミナルへの完全無干渉**
   - プラグイン処理はすべて別プロセスまたはバックグラウンドスレッドへ隔離し、GPUI メインスレッドの描画や内蔵ターミナル（AI エージェント稼働中を含む）の入出力を絶対にブロックしない。
   - プラグインの環境変数を本体・内蔵ターミナル PTY へ漏らさず、内蔵ターミナル PTY へのアクセス権限は既定禁止とする。
4. **既存資産の徹底活用**
   - 既存の `ExtensionHostProxy`（`crates/extension/src/extension_host_proxy.rs`）、`ExtensionCapability`（`crates/extension/src/capabilities.rs`）、`theme_importer`（`crates/theme_importer/src/vscode/`）、`snippet_provider`（`crates/snippet_provider`）、`node_runtime`（`crates/node_runtime`）、`wasm_host`（`crates/extension_host/src/wasm_host.rs`）を基盤として拡張する。

### 1.2 永久に採用しない機能（非目標）

- **Webview（ブラウザエンジン / Chromium / WebView2 等）の組み込み**: メモリ消費と描画負荷の最大要因であるため永久に不採用とする。UI 拡張は GPUI ネイティブ描画の宣言的コンポーネントのみに限定する。
- **オンライン VS Code マーケットプレイス検索・自動通信・自動更新**: `.vsix` は上級者向けとして、ユーザーがブラウザ等で取得したローカルファイルを手動導入する方式に限定する。

---

## 2. 全体アーキテクチャ

```text
+-------------------------------------------------------------------+
| GPUI メインスレッド（UI描画 / 内蔵ターミナル描画: ブロック厳禁）       |
|  - CommandPalette（日本語検索・説明併記）                           |
|  - ThemeRegistry / SnippetRegistry / LocaleRegistry               |
|  - 宣言的 UI コンポーネント（StatusBar / Notification / QuickPick） |
+-------------------------------+-----------------------------------+
                                | 非同期チャネル / ExtensionHostProxy
                                v
+-------------------------------------------------------------------+
| ExtensionHostProxy（crates/extension）                             |
|  既存: Theme / Grammar / Language / LanguageServer / Snippet / DAP |
|  追加: Locale / Command / Ui / Editor                              |
+---------------+-------------------------------+-------------------+
                |                               |
                v                               v
+-------------------------------+   +-------------------------------+
| 静的リソース層（常駐 0 MB）     |   | 動的実行層（オンデマンド隔離）   |
|  - crates/i18n (LocaleStore)  |   |  - Wasmtime Host (wit v0.7)   |
|  - crates/vscode_compat       |   |    ResourceLimiter 64-128 MB  |
|    .vsix -> Theme/Icon/       |   |  - VS Code 互換 Node.js Host  |
|    Snippet/Keybinding/Locale  |   |    1プロセス共有 / 256 MB上限   |
+-------------------------------+   |    15分アイドルで自動終了       |
                                    |    OS低優先度 + Job Object     |
                                    +-------------------------------+
```

---

## 3. 領域別詳細設計

### 3.1 日本語 UI 強化 ＋ プラグイン対応 i18n 基盤（`crates/i18n` + `ExtensionLocaleProxy`）

#### 3.1.1 クレート構成と責務

- 新規クレート `crates/i18n` を追加し、UI 文字列解決・ロケール切替・アクション日本語メタデータ（表示名・説明・検索エイリアス）を一元管理する。
- 既存の `crates/extension/src/extension_host_proxy.rs` に `ExtensionLocaleProxy` トレイトを追加し、`ExtensionHostProxy` へ登録可能にする。

```rust
pub trait ExtensionLocaleProxy: Send + Sync + 'static {
    /// プラグインから提供されたロケール辞書（JSON/キーバリュー）を登録する
    fn register_locale_pack(&self, extension_id: Arc<str>, locale: LocaleId, entries: HashMap<String, String>) -> Result<()>;

    /// プラグイン無効化・アンインストール時に該当ロケール辞書を解除する
    fn unregister_locale_pack(&self, extension_id: &str);
}
```

#### 3.1.2 ロケール解決順序とメモリ設計

1. **既定ロケール**: `ja`（現行 Gram 日本語 UI をそのまま正本として保持）。
2. **解決優先順位**:
   1. 選択中ロケールに対するプラグイン上書き辞書（言語パック）
   2. 選択中ロケールの組み込み辞書（`ja` または `en`）
   3. フォールバックロケール（`en`）の組み込み辞書
   4. キー文字列そのもの（欠落時も空文字にせず判読可能に保つ）
3. **メモリ最小化**:
   - アクティブにメモリ展開するのは「現在の選択ロケール 1 つ」と「フォールバック `en`」のみ。
   - 他のロケールパックがディスク上に複数導入されていても、選択切替時まで JSON パース・メモリ展開を行わない。
   - 文字列参照は `Arc<str>` / `Cow<'static, str>` で共有し、フレーム毎の不要なアロケーションを避ける。

#### 3.1.3 コマンドパレットの日本語 UI 強化

- `crates/i18n` に主要アクション ID（例: `workspace::Save`, `editor::ToggleComments`, `pane::SplitRight` 等）の日本語ラベル・簡潔な説明・検索用キーワードを定義する（言語パックからの上書きにも対応）。
- `crates/command_palette` の検索・描画を拡張する:
  - **二言語ハイブリッド検索**: 元の英語アクション名（例: `workspace: save`）と日本語ラベル・説明（例: `ファイルを保存`）の両方を対象にファジー検索できる。
  - **日本語説明の併記**: リスト項目に英語アクション名（または日本語ラベル）と簡潔な日本語説明を 1 行内にスリムに表示し、検索一致位置をハイライトする。
  - 既存の英語コマンド名検索・履歴（`COMMAND_PALETTE_HISTORY`）・コマンドエイリアス（`command_aliases`）との後方互換を 100% 維持する。

---

### 3.2 VS Code 拡張（`.vsix`）のローカル手動導入（`crates/vscode_compat`）

#### 3.2.1 導入フロー（オフライン完結）

1. ユーザーが手元で取得した `.vsix` ファイルを指定してローカル導入を実行する（コマンドまたは設定画面からのファイル指定）。
2. `crates/vscode_compat` が `.vsix`（ZIP アーカイブ、`async_zip` を再利用）を検査し、以下を検証する:
   - パストラバーサル（`..` や絶対パス、シンボリックリンク逸脱）の拒否
   - アーカイブ展開サイズ・エントリ数の上限チェック（ZIP ボム対策）
   - `extension/package.json` のパースとマニフェスト抽出（`name`, `publisher`, `version`, `engines.vscode`, `contributes`, `activationEvents`, `main`, `browser`）
3. 静的リソースのみか、動的 JS エントリポイント（`main`）を含むかを判定し、必要な `ExtensionCapability` 一覧を生成してユーザーへ提示・明示承認を求める。
4. 承認後、ローカルの拡張ディレクトリへ展開し、静的リソースを即座に Gram 形式へ変換して各プロキシへ登録する。

#### 3.2.2 静的リソース層（常駐 0 MB・Node.js 不要）

| VS Code `contributes` | 変換先・接続先 | 備考 |
| --- | --- | --- |
| `themes` | `crates/theme_importer` の `VsCodeThemeConverter` で Gram テーマ JSON へ変換 → `ExtensionThemeProxy` | `uiTheme`（`vs-dark` / `vs` / `hc-black`）から `AppearanceContent` を決定 |
| `iconThemes` | VS Code アイコンテーマ JSON（`iconDefinitions`, `fileExtensions`, `fileNames`, `folderNames`）を Gram `IconThemeFamilyContent` へ変換 → `ExtensionThemeProxy` | フォントアイコン未対応分は安全にスキップし警告記録 |
| `snippets` | VS Code スニペット JSON を `ExtensionSnippetProxy::register_snippet` へ直接登録 | Gram の `snippet_provider` は VS Code スニペット形式と高い互換性を持つ |
| `keybindings` | `key` / `mac` / `win` / `linux` / `command` / `when` を Gram キーマップ定義へ変換し、対応可能なコマンドのみ安全にマッピング | 未知コマンドは無視し、既存キーバインドを破壊しない |
| `localizations` | `translations` 配列内の JSON を読み込み、`ExtensionLocaleProxy::register_locale_pack` へ登録 | `languageId`（例: `ja`, `en`）単位でオンデマンド展開 |

- `contributes` に含まれる静的リソース変換はすべて純粋 Rust で完結させ、Node.js を一切起動しない。

---

### 3.3 動的プラグイン実行基盤

#### 3.3.1 `ExtensionHostProxy` の拡張

既存の 6 プロキシ（`Theme`, `Grammar`, `Language`, `LanguageServer`, `Snippet`, `DebugAdapterProvider`）に加え、以下 4 プロキシを段階的に追加する:

1. `ExtensionLocaleProxy`: ロケール辞書の登録・解除（Step 1）
2. `ExtensionCommandProxy`: プラグイン提供コマンドの登録・解除・非同期実行ディスパッチ（Step 3）
3. `ExtensionUiProxy`: 宣言的 UI コンポーネント（ステータスバー項目・通知・QuickPick・入力ボックス・軽量ツリー/リスト）の更新とユーザー応答の非同期返却（Step 3）
4. `ExtensionEditorProxy`: アクティブ文書メタデータ・テキストスナップショット・保存イベント・診断（Diagnostics）の非同期連携（Step 3）

#### 3.3.2 VS Code 互換 Node.js ホスト（Step 4）

- **起動条件**: `main` を持つ `.vsix` が 1 つ以上有効化され、かつその `activationEvents`（`onLanguage:<id>`, `onCommand:<id>`, `workspaceContains:<glob>`, `*` 等）が発火した時点でのみオンデマンド起動する。
- **プロセス構成**: 全 JS 拡張で **1 プロセス共有**（拡張ごとに Node.js を乱立させない）。既存の `crates/node_runtime` を利用する。
- **メモリ・ライフサイクル制御**:
  - Node.js 起動引数に `--max-old-space-size=256` を必須付与する。
  - アクティブな拡張タスク・LSP・コマンドが存在しない状態が **15 分** 続いたら自動終了（アイドルシャットダウン）し、次回の発火時に再起動する。
- **対応 API スコープ（厳格に限定）**:
  - `vscode.commands`（登録・実行）
  - `vscode.window`（ステータスバー、通知 `showInformationMessage`/`showWarningMessage`/`showErrorMessage`、`showQuickPick`、`showInputBox`、軽量 `TreeDataProvider`）
  - `vscode.workspace`（文書オープン/変更/保存イベント、設定読み取り、デバウンス済みファイル監視）
  - `vscode.languages`（`DiagnosticCollection`、LSP クライアント起動ブリッジ）
  - **非対応（明示エラー返却）**: `createWebviewPanel`、`registerWebviewViewProvider`、`window.createTerminal`（PTY アクセス既定禁止）

#### 3.3.3 Gram ネイティブ Wasm API（`extension.wit` v0.7）

- 既存の `crates/extension_host/src/wasm_host/wit/`（現行 v0.6.0 まで実装済み）へ v0.7 を追加し、`commands` / `ui` / `editor` / `locale` インターフェースを定義する。
- Wasmtime の `StoreLimitsBuilder`（`ResourceLimiter`）を全 Wasm `Store` に適用し、**1 プラグインあたりメモリ上限 64〜128 MB**（既定 64 MB、設定またはマニフェスト上限 128 MB）およびテーブル要素数上限を強制する。
- 不要になった（無効化・アンロード・アイドル上限到達）Wasm インスタンスは即座に `Drop` して線形メモリを OS へ返却する。

#### 3.3.4 権限モデル（`ExtensionCapability` 統一）

- 既存の `ExtensionCapability`（`process:exec`, `download_file`, `npm:install`）を拡張し、プラグイン種別（Wasm / `.vsix`）を問わず同一の権限審査モデルへ統一する:
  - `workspace:read` / `workspace:write`（ワークスペース内ファイルアクセス）
  - `ui:notify` / `ui:status_bar` / `ui:prompt`（宣言的 UI 操作）
  - `command:register`（コマンドパレット登録）
  - `network:http`（明示ホスト限定、既定拒否）
  - `terminal:pty`（**既定禁止**、明示許可がない限り全面拒否）
- `.vsix` 導入時には、マニフェストとエントリポイント種別から必要権限を自動抽出し、一覧表示してユーザーの明示承認を得たうえで記録する。

---

### 3.4 低メモリ ＆ 内蔵ターミナル（AI エージェント）完全無干渉の制約

| 制約項目 | 実装メカニズム |
| --- | --- |
| **スレッド・プロセス隔離** | すべてのプラグイン実行（Wasm / Node.js / `.vsix` 展開変換）はバックグラウンドスレッド（`BackgroundExecutor` / Tokio）または別プロセスで実行し、GPUI メインスレッドとは有界チャネル経由で非同期通信する。メインスレッド上でプラグイン完了を同期待機しない。 |
| **OS 優先度・メモリ上限回収** | プラグインホストプロセスは OS 低優先度で起動する。Windows では `BELOW_NORMAL_PRIORITY_CLASS` を付与し、Win32 Job Object（`JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE` + `JOB_OBJECT_LIMIT_PROCESS_MEMORY`）へ紐付けてメモリ上限を強制し、Gram 終了時に子プロセスを一括回収する。Unix 系では `nice` 値引き上げ＋プロセスグループ管理を行う。 |
| **環境変数・PTY の完全隔離** | プラグインホスト起動時は最小限の許可リスト環境変数のみ渡し、プラグイン用環境変数を本体プロセスや内蔵ターミナル PTY へ一切伝播させない。内蔵ターミナル PTY への読み書き・生成 API は既定禁止とする。 |
| **ファイル監視のイベント嵐遮断** | ファイル変更通知はホスト側で **デバウンス（例: 200ms）＋バッチ化** してからプラグインへ配送する。`target/`、`.git/`、`node_modules/`、`tmp/`（および `.gitignore` 相当のビルド出力）を既定除外し、AI エージェントが大量にファイル生成・ビルドしている最中でもプラグインホストへイベント嵐を送らない。 |

---

## 4. 段階的実装計画と検証基準

小さく動く単位ごとにビルド・テストを実測しながら進める。

### Step 1: `crates/i18n` ＋ `ExtensionLocaleProxy` ＋ 日本語 UI 強化
- **成果物**:
  - `crates/i18n`: `LocaleId`, `LocaleRegistry`（選択ロケール 1 つ＋フォールバック `en` のみ展開、プラグイン上書き対応）、主要アクション日本語カタログ（ラベル・説明・検索キーワード）。
  - `crates/extension`: `ExtensionLocaleProxy` トレイトと `ExtensionHostProxy` 実装。
  - `crates/command_palette`: 日本語ラベル・説明の併記、英語／日本語ハイブリッド検索の実装。
  - `crates/gram`: 起動時の `i18n::init` および `ExtensionLocaleProxy` 登録接続。
- **検証基準**:
  - `cargo test -p i18n`、`cargo test -p extension`、`cargo test -p command_palette` 全 PASS。
  - 言語パック上書き・解除・フォールバック（`ja` → `en` → キー返却）・未使用ロケール非展開・日本語クエリでのコマンドパレット検索一致を単体テストで実証。

### Step 2: `crates/vscode_compat` ローカル `.vsix` 静的ローダー
- **成果物**:
  - `crates/vscode_compat`: `.vsix`（ZIP）安全展開（パストラバーサル・展開サイズ制限・エントリ数制限）、`package.json` パース、必要権限抽出、静的リソース変換器（`themes` / `iconThemes` / `snippets` / `keybindings` / `localizations`）。
  - `ExtensionHostProxy`（`Theme` / `Snippet` / `Locale`）への登録ブリッジ。
- **検証基準**:
  - `cargo test -p vscode_compat` 全 PASS。
  - テスト用 `.vsix` アーカイブを実生成し、展開・セキュリティ拒否（パストラバーサル/サイズ超過）・テーマ/アイコン/スニペット/キーバインド/言語パック変換を実測検証。

### Step 3: 動的実行基盤の共通プロキシ・権限・Wasm `ResourceLimiter` 強化
- **成果物**:
  - `ExtensionCommandProxy` / `ExtensionUiProxy` / `ExtensionEditorProxy` の定義と登録。
  - `ExtensionCapability` の拡張と `.vsix` 権限審査フロー。
  - `crates/extension_host`: Wasmtime `StoreLimits`（64〜128 MB 上限）導入、`extension.wit` v0.7 基盤、ファイル監視デバウンス＋除外フィルタ（`target/`, `.git/`, `node_modules/`, `tmp/`）。

### Step 4: VS Code 互換 Node.js ホスト（オンデマンド・低優先度・15 分アイドル終了）
- **成果物**:
  - 1 プロセス共有 Node.js 拡張ホスト（`--max-old-space-size=256`、15 分アイドル自動終了、Activation Events オンデマンド起動）。
  - Windows `BELOW_NORMAL_PRIORITY_CLASS` ＋ Job Object 隔離、環境変数隔離、PTY 禁止ガード。
  - 限定 VS Code API（commands / status bar / notifications / QuickPick / InputBox / document events / diagnostics / LSP bridge）の JSON-RPC 実装。
