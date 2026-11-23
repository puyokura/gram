# PHP

PHP のサポートは
[PHP 拡張機能](https://github.com/zed-extensions/php) を通じて利用できます。

- Tree-sitter: https://github.com/tree-sitter/tree-sitter-php
- 言語サーバー:
  - [phpactor](https://github.com/phpactor/phpactor)
  - [intelephense](https://github.com/bmewburn/vscode-intelephense/)

## 言語サーバーの選択

PHP 拡張機能は `phpactor` と `intelephense` の両方の言語サーバーをサポートしています。

既定では `phpactor` が有効になっています。

### Phpactor

Gram の PHP 拡張機能は `phpactor` を自動的にインストールできますが、`php` がインストールされ PATH で利用可能になっている必要があります:

```sh
# brew install php            # macOS
# sudo apt-get install php    # Debian/Ubuntu
# yum install php             # CentOS/RHEL
# pacman -S php               # Arch Linux
which php
```

### Intelephense

[Intelephense](https://intelephense.com/) はフリーミアムモデルで提供されている PHP 向けの
[プロプライエタリな](https://github.com/bmewburn/vscode-intelephense/blob/master/LICENSE.txt#L29)
言語サーバーです。特定の機能を利用するには [プレミアムライセンス](https://intelephense.com/) の購入が必要です。

`intelephense` に切り替えるには、`settings.json` に次を追加します:

```json
{
  "languages": {
    "PHP": {
      "language_servers": ["intelephense", "!phpactor", "!phptools", "..."],
    },
  },
}
```

プレミアム機能を使用するには、ホームディレクトリ内の `~/intelephense/licence.txt` に
[licence.txt ファイル](https://intelephense.com/faq.html) を配置します。あるいは、`intelephense` 言語サーバーの初期化オプションとして、ライセンスキーまたはライセンスキーを含むファイルへのパスを渡すこともできます。これを行うには、`settings.jsonc` に次を追加します:

```json
{
  "lsp": {
    "intelephense": {
      "initialization_options": {
        "licenceKey": "/path/to/licence.txt",
      },
    },
  },
}
```

## PHPDoc

Gram は PHPDoc コメントの構文ハイライトをサポートしています。

- Tree-sitter:
  [claytonrcarter/tree-sitter-phpdoc](https://github.com/claytonrcarter/tree-sitter-phpdoc)

## Xdebug の設定

Gram の PHP 拡張機能は PHP および Xdebug 用のデバッグアダプターを提供します。アダプター名は `Xdebug` です。使用例は次のとおりです:

```json
[
  {
    "label": "PHP: Listen to Xdebug",
    "adapter": "Xdebug",
    "request": "launch",
    "port": 9003
  },
  {
    "label": "PHP: Debug this test",
    "adapter": "Xdebug",
    "request": "launch",
    "program": "vendor/bin/phpunit",
    "args": ["--filter", "$GRAM_SYMBOL"]
  }
]
```

問題が発生した場合は、次の点を確認してください:

- 実行している PHP のバージョンに対応する Xdebug がインストールされているか
- Xdebug が `debug` モードで実行されるよう設定されているか
- Xdebug が実際にデバッグセッションを開始しているか
- Xdebug と Gram の間でホストとポートが一致しているか
- デバッグ対象のページで `xdebug_info()` 関数を使用して診断ログを確認
