# YAML

YAML のサポートは Gram でネイティブに利用できます。

- Tree-sitter:
  [zed-industries/tree-sitter-yaml](https://github.com/zed-industries/tree-sitter-yaml)
- 言語サーバー:
  [redhat-developer/yaml-language-server](https://github.com/redhat-developer/yaml-language-server),
  [home-operations/yayamlls](https://github.com/home-operations/yayamlls)

## 言語サーバーの選択

`yaml-language-server` は既定で使われる Node.js 製の LSP です。`yayamlls` は静的リンクされた Go バイナリで、Kubernetes と Flux をサポートしますが、フォーマッターはありません。

[yayamlls 拡張機能](https://github.com/home-operations/yayamlls/tree/main/editors/zed)（バイナリを自動でダウンロードします）をインストールしている場合:

```json
"languages": {
  "YAML": {
    "language_servers": ["yayamlls", "!yaml-language-server"]
  }
}
```

拡張機能を使わない場合は、`yayamlls` を自分でインストールし、同梱のバイナリを上書きします。`lsp` キーは既知の LSP 識別子しか受け付けないためです:

```json
"lsp": {
  "yaml-language-server": {
    "binary": {
      "path": "yayamlls",
      "arguments": [],
      "ignore_system_version": true
    }
  }
}
```

> [!NOTE]
> `arguments` は空でなければなりません。Gram はこのサーバーを `--stdio` 付きで起動しますが、`yayamlls` はそれを受け付けません。

## フォーマット

既定では、Gram は YAML ファイルのフォーマットに Prettier を使用します。たとえば yaml ファイルでシングルクォートを使うには、`.prettierrc` に次を追加します:

```json
{
  "overrides": [
    {
      "files": ["*.yaml", "*.yml"],
      "options": {
        "singleQuote": true
      }
    }
  ]
}
```

### LSP によるフォーマット

YAML のフォーマットに Prettier の代わりに `yaml-language-server` を使うには、Gram の `settings.jsonc` に次を追加します:

```json
"languages": {
  "YAML": {
    "formatter": "language_server"
  }
}
```

`yayamlls` にはフォーマット機能がありません。

## 設定

言語サーバーのオプションは `settings.jsonc` ファイルで設定できます。詳しくは
[言語の設定](../configuring-languages.md)のドキュメントを参照してください。

### yaml-language-server

詳しくは [yaml-language-server の設定](https://github.com/redhat-developer/yaml-language-server?tab=readme-ov-file#language-server-settings)を参照してください。
設定キーは入れ子にする必要があるため、`yaml.keyOrdering` は `{"yaml":
{"keyOrdering": true}}` になります:

```json
"lsp": {
  "yaml-language-server": {
    "settings": {
      "yaml": {
        "keyOrdering": true,
        "format": {
          "singleQuote": true
        },
        "schemas": {
          "https://getcomposer.org/schema.json": ["/*"],
          "../relative/path/schema.json": ["/config*.yaml"]
        },
        "schemaStore": {
          "enable": true
        },
        "customTags": [
          "!Ref scalar",
          "!Sub mapping",
          "!GetAtt sequence"
        ]
      }
    }
  }
}
```

スキーマは [Json Schema Store](https://schemastore.org) から自動検出されます。`schemas` キーはそれを上書きし、`schemaStore.enable: false` は無効にします。[カスタムタグ](https://github.com/redhat-developer/yaml-language-server#adding-custom-tags)には `scalar`、`mapping`、`sequence` のサフィックスが必要です（CloudFormation など）。

モデルインで単一ファイルのスキーマを固定でき、両方のサーバーで動作します:

```yaml
# yaml-language-server: $schema=https://www.schemastore.org/github-action.json
name: Issue Assignment
on:
  issues:
    types: [opened]
```

### yayamlls

詳しくは [yayamlls のドキュメント](https://github.com/home-operations/yayamlls#configuration)を参照してください。設定はワークスペースのルートにある `.yayamlls.yaml` に置かれ、エディターに依存しない形で管理できます:

```yaml
schemas:
  "https://json.schemastore.org/github-workflow.json":
    - ".github/workflows/*.yml"
catalog: true
kubernetes:
  enabled: false
customTags:
  - "!Ref"
```

同じキーを Gram から設定することもでき、その場合はフラットに、`yaml` ラッパーなしで記述します。拡張機能を使う場合は `yayamlls` キーを、上記のバイナリ上書きを使う場合は `yaml-language-server` キーを使用します:

```json
"lsp": {
  "yayamlls": {
    "initialization_options": {
      "catalog": true,
      "kubernetes": { "enabled": false }
    }
  }
}
```

スキーマはドキュメントごとに、モデルイン、`schemas` の glob、Schema Store カタログ（`catalog: false` で無効化）、Kubernetes の `apiVersion`+`kind` の順で解決されます。カスタムタグは素の形で列挙し、それを持つノードは検証をスキップします。

診断は、`# yayamlls-disable-line`、ブロックを囲む `# yayamlls-disable` と `# yayamlls-enable`、または `# yayamlls-disable-file` で抑制できます。

Flux の `HelmRelease` と `Kustomization` ドキュメントは、レンダリング後のマニフェストに対しても検証され、**View rendered** と **Diff rendered** のコードレンズが提供されます。このレンズは `window/showDocument` を使用しますが、Zed はローカルファイルでこれを無視します（[zed#53123](https://github.com/zed-industries/zed/discussions/53123)）。

CI では、`yayamlls validate [--render] <paths>` が同じ方法でスキーマを解決し、エラー時は `1`、使用法や I/O の失敗時は `2`、それ以外は `0` で終了します。
