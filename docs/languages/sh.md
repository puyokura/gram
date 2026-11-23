# シェルスクリプト

シェルスクリプト（bash、zsh、dash、sh）は Gram がネイティブでサポートしています。

- Tree-sitter:
  [tree-sitter/tree-sitter-bash](https://github.com/tree-sitter/tree-sitter-bash)

## 設定

シェルスクリプトの各種設定は、Gram のユーザー設定
（`~/.config/gram/settings.json`）または Gram のプロジェクト設定
（`.gram/settings.json`）で構成できます:

```json
  "languages": {
    "Shell Script": {
      "tab_size": 2,
      "hard_tabs": false
    }
  }
```

### フォーマット

Gram は、[`shfmt`](https://github.com/mvdan/sh) などの外部ツールを使ってシェルスクリプトを自動フォーマットできます。

1. `shfmt` をインストールします:

```sh
brew install shfmt            # macos (homebrew)
sudo apt-get install shfmt    # debian/ubuntu
dnf install shfmt             # fedora
yum install shfmt             # redhat
pacman -Sy shfmt              # archlinux
choco install shfmt           # windows (chocolatey)
```

2. `shfmt` がパス上にあることを確認し、バージョンを表示します:

```sh
which shfmt
shfmt --version
```

3. 保存時に `shfmt` でシェルスクリプトを自動フォーマットするよう Gram を設定します:

```json
  "languages": {
    "Shell Script": {
      "format_on_save": "on",
      "formatter": {
        "external": {
          "command": "shfmt",
          // Change `--indent 2` to match your preferred tab_size
          "arguments": ["--filename", "{buffer_path}", "--indent", "2"]
        }
      }
    }
  }
```

## 関連項目:

- [Bash の言語サポート](./bash.md)
- [Fish の言語サポート](./fish.md)
