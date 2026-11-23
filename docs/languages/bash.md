# Bash

Bash 言語のサポートは Gram にネイティブで組み込まれています。

- Tree-sitter:
  [tree-sitter/tree-sitter-bash](https://github.com/tree-sitter/tree-sitter-bash)
- 言語サーバー:
  [bash-lsp/bash-language-server](https://github.com/bash-lsp/bash-language-server)

## 構成

### `shellcheck`

`bash-language-server` は `shellcheck` をサポートしており、インストールされていれば
自動的に利用します。インストールはユーザーが手動で行う必要があります。

```sh
brew install shellcheck             # macOS (HomeBrew)
apt install shellcheck              # Ubuntu / Debian
pacman -S shellcheck                # Arch Linux
dnf install shellcheck              # Fedora
yum install shellcheck              # CentOS/RHEL
zypper install shellcheck           # openSUSE
choco install shellcheck            # Windows (Chocolatey)
```

そして、PATH から利用できることを確認します:

```sh
which shellcheck
shellcheck --version
```

報告される警告/エラーをカスタマイズしたい場合は、`.shellcheckrc` ファイルを
作成するだけです。プロジェクトのルートまたはホームディレクトリ
（`~/.shellcheckrc`）に置けます。詳しくは
[shellcheck のドキュメント](https://github.com/koalaman/shellcheck/wiki/Ignore#ignoring-one-or-more-types-of-errors-forever)
を参照してください。

### `smfmt`

同様に、`bash-language-server` は `$PATH` に `shfmt` があれば、フォーマットに
自動的に利用します。次のようにインストールします:

```sh
brew install shfmt          # MacOS (HomeBrew)
apt install shfmt           # Ubuntu / Debian
pacman -S shfmt             # Arch Linux
choco install shfmt         # Windows (chocolatey)
```

自動フォーマットを制御するには、Gram 設定の `format_on_save` を使うか、
コマンドパレットから `editor: format document` を手動で実行します。

### 関連項目:

- [Gram ドキュメント: 言語サポート: シェルスクリプト](./sh.md)
