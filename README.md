<div align="center">

![Gram Logo](./assets/images/docs_logo.png)

# [Gram（日本語版）](https://github.com/puyokura/gram)

本リポジトリ（`puyokura/gram`）は、[Gram Editor](https://codeberg.org/GramEditor/gram) の UI・設定項目・内蔵ドキュメントを全面的に日本語化したフォークです。上流の **GNU General Public License v3.0 or later (GPL-3.0-or-later)** ライセンスを継承して公開しています。

Gram はソースコードエディター兼 IDE であり、Zed エディターのハードフォークとして開発されています。AI 統合・テレメトリ・プロプライエタリなサーバー機能を含まず、高速で信頼性が高く、手になじむ開発環境を目指しています。

![Gram Editor](./assets/images/docs_editor.png)

</div>

## 改変およびライセンス継承の明示（GPLv3 第5条に基づく表示）

- 上流プロジェクト: [GramEditor/gram](https://codeberg.org/GramEditor/gram)（Zed Editor からのフォーク）
- 改変内容: アプリケーションメニュー、設定画面（設定項目・説明文・ドロップダウン選択肢）、各パネル・モーダル・ダイアログ・通知、コマンドパレット、キーマップエディター、CLI ヘルプ、既定設定ファイルのコメント、および内蔵ドキュメント（`docs/`）の日本語化、ならびに GitHub Actions / GitHub Releases による配布対応
- 改変時期: 2026年
- ライセンス: 本リポジトリ全体の改変物および配布物は、上流と同じ **GNU General Public License v3.0 or later (GPL-3.0-or-later)** の下で提供されます。全文は [LICENSE](./LICENSE) を参照してください。本ソフトウェアは無保証（WITHOUT ANY WARRANTY）で提供されます。

## 主な機能

- クロスプラットフォーム対応（Windows / macOS / Linux / *BSD）
- SSH 経由のリモート開発
- DAP（Debug Adapter Protocol）による統合デバッガー
- Jupyter Notebook 対応（REPL モード）
- 組み込み Git サポート（差分・ステージ・コミット・ブランチ・スタッシュ・Git グラフ）
- Vim / Helix モーダル編集モード
- Zed Wasm 拡張機能の一部サポート
- AI 機能・テレメトリ・プロプライエタリなサーバー機能・強制自動更新なし
- 内蔵ドキュメント（アプリ内から日本語で閲覧可能）
- 多数の言語および LSP（Language Server Protocol）の組み込みサポート
- 豊富なカラーテーマ・アイコンテーマ

## インストール

ビルド済みバイナリは [GitHub Releases](https://github.com/puyokura/gram/releases) からダウンロードできます。

各プラットフォーム別の詳細やビルド手順については、内蔵ドキュメントまたは `docs/` 配下のガイドを参照してください。

- [Linux ガイド](./docs/linux.md)
- [Windows 開発・ビルドガイド](./docs/development/windows.md)
- [macOS 開発・ビルドガイド](./docs/development/macos.md)

## 上流プロジェクト・コミュニティ

上流の Gram Editor に関する公式情報・クレジットは以下のページを参照してください。

- 上流リポジトリ: <https://codeberg.org/GramEditor/gram>
- 公式サイト・ドキュメント: <https://gram-editor.com/docs>
- クレジット: <https://gram-editor.com/credits>
- 上流の理念（Mission Statement）: [docs/mission.md](./docs/mission.md)

## ライセンス

Gram Editor および本日本語版フォークは **GPL-3.0-or-later** でライセンスされています。元の Zed エディターのコードベースは Apache-2.0 / AGPL-3.0 / GPL-3.0 のトリプルライセンスですが、Gram および本リポジトリにおけるすべての改変は GPL-3.0-or-later の下で公開されます。

本プロジェクトは元のソースコードおよび各依存ライブラリのライセンスに従います（同梱の [assets/licenses.md](./assets/licenses.md) および [LICENSE](./LICENSE) を参照してください）。
