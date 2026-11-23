# ドキュメント

Gram はソースコードエディター兼統合開発環境（IDE）です。豊富な機能を備えていますが、まだまだ開発途上のプロジェクトでもあります。コミュニティプロジェクトであり、現在のコミュニティは余暇に開発を進める 1 人の気難しいヒキガエルと、少数ながら熱心なコントリビューターたち（彼らに多大なる愛を）で構成されています。本プロジェクトは、安定性、信頼性、そして敬意を重んじる方向へと、ゆっくりと着実に前進することを目指しています。

Gram には数多くの魅力があります。

- 現在の時点でほぼ問題なく動作します。私自身、日々のコード作成に毎日使用しています。
- 優れたパフォーマンス。
- Mac、Linux、FreeBSD、Windows のクロスプラットフォーム対応。
- SSH 経由のリモート開発。
- `DAP` プロトコルによる統合デバッガー対応。
- Jupyter Notebook 対応（REPL モード）。
- 組み込みの Git 対応。
- 実用的な Vim および Helix モーダル編集モード。
- Zed Wasm 拡張機能の一部/限定的なサポート。
- AI 連携や収益化スキームは一切なし。
- テレメトリーやプロプライエタリなサーバーコンポーネントは一切なし。
- ビデオや音声によるチャット機能はなし。
- 勝手な自動アップデートはなし。
- 明示的な許可なしに何かをインストールしないよう配慮。
- 統合ドキュメント。
- 多数の言語と言語サーバーの組み込みサポート。
- 豊富な組み込みテーマ。

本プロジェクトは「完璧なエディター」というビジョンから始まったのではなく、ソフトウェア開発の世界、世界全体、そして VS Code、Zed、Vim などのエディターで起きてきた、そして今も起きていることに対する反発から始まりました。その核となる哲学は「手間を惜しまず手作業でコードを書くことは正しく、素晴らしいことである」ということであり、まさにそれを行うための優れたツールとなることを目指しています。

Gram は Zed エディターの **ハードフォーク** としてスタートしました。このフォークの背景と動機の詳細については、[ミッションステートメント](./mission.md) をお読みください。

### 移行ガイド

- [Zed](./migrate/zed.md) からの移行
- [VS Code](./migrate/vs-code.md) からの移行

## 主な機能

- [デバッガー](./debugger.md): デバッガーアダプタープロトコル（DAP）の統合サポート。
- [リモート開発](./remote-development.md): SSH 経由でリモートサーバーに接続し、ローカルプロジェクトのように編集。
- [拡張機能](./extensions.md): 拡張機能システムを使用して、言語、テーマ、アイコンの追加サポートを導入。
- [対応言語](./languages.md)
- [言語サーバー](./language-servers.md): Gram は言語サーバーを利用して、さまざまなプログラミング言語の高度なセマンティック機能を提供します。

## 開発

- [開発](./development.md)
  - [macOS](./development/macos.md)
  - [Linux](./development/linux.md)
  - [Windows](./development/windows.md)
  - [FreeBSD](./development/freebsd.md)
  - [デバッガーの使用方法](./development/debuggers.md)
  - [用語集](./development/glossary.md)
- [クラッシュのデバッグ](./development/debugging-crashes.md)
- [肥大化の調査](./development/bloat.md)
- [所有権とデータフロー](./development/ownership-and-data-flow.md)

## 設定

- [Gram の設定](./configuring-gram.md)
- [言語の設定](./configuring-languages.md)
  - [ツールチェーン](./toolchains.md)
- [キーバインド](./key-bindings.md)
  - [すべてのアクション](./all-actions.md)
- [スニペット](./snippets.md)
- [テーマ](./themes.md)
- [アイコンテーマ](./icon-themes.md)
- [外観のカスタマイズ](./visual-customization.md)
- [Vim モード](./vim.md)
- [Helix モード](./helix.md)
- [SuperTab](./supertab.md)

## Gram の使い方

- [マルチバッファー](./multibuffers.md)
- [コマンドパレット](./command-palette.md)
- [コマンドラインインターフェイス](./command-line-interface.md)
- [アウトラインパネル](./outline-panel.md)
- [コード補完](./completions.md)
- [Git](./git.md)
- [デバッガー](./debugger.md)
- [診断](./diagnostics.md)
- [タスク](./tasks.md)
- [タブスイッチャー](./tab-switcher.md)
- [リモート開発](./remote-development.md)
- [環境変数](./environment.md)
- [REPL](./repl.md)

## プラットフォーム対応

- [Windows](./windows.md)
- [Linux](./linux.md)

## トラブルシューティング

- [トラブルシューティング](./troubleshooting.md)
- [アンインストール](./uninstall.md)

## 拡張機能

> **注意:** Zed の拡張機能システムはクローズドソースのサーバーコンポーネントに依存していましたが、Gram ではそれが完全に除去されています。代わりに、すべての拡張機能はソースコードからビルドする必要があります。現在、拡張機能レジストリは存在しないため、拡張機能はサジェストポップアップまたは URL 経由でインストールする必要があり、Wasm 拡張機能のコンパイルには rustup がインストールされている必要があります。

- [概要](./extensions.md)
- [拡張機能のインストール](./extensions/installing-extensions.md)
- [拡張機能の開発](./extensions/developing-extensions.md)
- [拡張機能の機能](./extensions/capabilities.md)
- [言語拡張機能](./extensions/languages.md)
- [デバッガー拡張機能](./extensions/debugger-extensions.md)
- [テーマ拡張機能](./extensions/themes.md)
- [アイコンテーマ拡張機能](./extensions/icon-themes.md)

## 統合および関連ツール

主に各種パッケージマネージャーや Linux ディストリビューションで Gram を利用できるようにするための関連プロジェクトがいくつかあります。このリストはすべてを網羅しているわけではありません。パッケージングの取り組みや Gram 固有の拡張機能などをご存知の場合は、<https://codeberg.org/GramEditor/gram> へお気軽にプルリクエストをお寄せください。

- **Homebrew (Mac):** <https://formulae.brew.sh/cask/gram>
- **Arch Linux:** <https://archlinux.org/packages/extra/x86_64/gram>
- **Arch Linux (AUR):** <https://aur.archlinux.org/packages/gram-git>
- **Alpine Linux:**
  <https://pkgs.alpinelinux.org/package/edge/testing/x86_64/gram>
- **Gentoo Linux:** <https://codeberg.org/GramEditor/gram-gentoo>
- **Raycast (Mac):** <https://www.raycast.com/justyt65/gram>
- **Chimera Linux (WIP):** <https://github.com/chimera-linux/cports/pull/5506>

## 貢献の受け入れに関する法的注意

過去に Zed をインストールし、そのライセンス契約に同意したことがある場合、コードがオープンソースライセンスであるにもかかわらず、Gram への貢献が法的に制限される可能性があります。私は弁護士ではありませんし、彼らが使用しているライセンスは少なくともヨーロッパの法廷では通用しないのではないかと推測していますが、確かなことはわかりません。まさにその理由から、私は彼らのライセンスに決して同意しませんでした。それこそが、このフォークが存在する主な理由です。

パッチの貢献を希望される場合は、ご自身に法的な貢献の権利があることを確認および保証する全責任を負っていただく必要があります。

## コミュニティはあなた自身です

Gram は、単なる言葉だけでなく精神的にもオープンソースであることを誇りにしています。とはいえ、エディターに何を含めるかについては強い意見を持っています。たとえば、Zed からフォークした主な動機は、道徳的に同意できない特定の「機能」を削除することでした。しかしもちろん、ご自身が適切と思われる方法で自由にカスタマイズしていただいて構いません。

公式の Discord や Reddit コミュニティはありませんが、[gram@rooms.slidge.im][xmpp-link] に Gram 向けの XMPP チャットがあります。任意の XMPP クライアントから接続可能であり、[簡易 Web UI][xmpp-webui] も利用できます。チャットログは [オンラインアーカイブ][xmpp-archive] でも閲覧可能です。

[xmpp-link]: xmpp:gram@rooms.slidge.im?join
[xmpp-webui]: https://slidge.im/gram/#/guest?join=gram@rooms.slidge.im
[xmpp-archive]: https://rooms.slidge.im:5281/muc_log/gram/

## 厳格な AI/LLM 不採用ポリシー

AI はもうお断りです。以前はここに、もっとマイルドな表現の声明を載せていました。それはまだ「バイブコーディング（vibe-coding）」といった概念をよく知らず、単にチャットボックスやオートコンプリート形式の AI にうんざりしていた初期の頃に書いたものでした。当時でも十分酷かったのですが、それが（これを書いている2026年3月時点で）どのような姿になったかを見て、私はまったく賛同できなくなりました。このポリシーは、[Zig 言語プロジェクトの行動規範][zig-coc] から引用したものです。

> Issue での LLM の使用禁止。
>
> プルリクエストでの LLM の使用禁止。
>
> バグトラッカーへのコメント（翻訳を含む）での LLM の使用禁止。英語での投稿を推奨しますが、必須ではありません。母国語で投稿し、他のメンバーが各自好みの翻訳ツールを使って意図を読み取ることに任せていただいて構いません。

Zed のコードベースには大量の AI 生成コードが含まれています。これ以上 1 行たりとも増やす必要はありません。

[zig-coc]: https://ziglang.org/code-of-conduct/

## SciActive 人間による貢献ポリシー

Gram は [SciActive 人間による貢献ポリシー 2-UP](./HUMAN-CONTRIBUTION-POLICY-2-UP.md) に準拠しています。
