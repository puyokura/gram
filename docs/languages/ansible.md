# Ansible

Gram での Ansible サポートは、コミュニティが保守する
[Ansible 拡張機能](https://github.com/kartikvashistha/zed-ansible) を通じて提供されます。

- Tree-sitter:
  [zed-industries/tree-sitter-yaml](https://github.com/zed-industries/tree-sitter-yaml)
- 言語サーバー:
  [ansible/vscode-ansible](https://github.com/ansible/vscode-ansible/tree/main/packages/ansible-language-server)

## セットアップ

### ファイル検出

Ansible 以外の YAML ファイルを誤って扱わないよう、Ansible 言語は既定では
どのファイル拡張子にも関連付けられていません。この動作を変えるには、
プロジェクト設定（`.gram/settings.jsonc`）または Gram のユーザー設定
（`~/.config/gram/settings.jsonc`）に `"file_types"` セクションを追加し、
フォルダーや命名の慣習に合わせてください。例:

```json
"file_types": {
    "Ansible": [
      "**.ansible.yml",
      "**.ansible.yaml",
      "**/defaults/*.yml",
      "**/defaults/*.yaml",
      "**/meta/*.yml",
      "**/meta/*.yaml",
      "**/tasks/*.yml",
      "**/tasks/*.yaml",
      "**/handlers/*.yml",
      "**/handlers/*.yaml",
      "**/group_vars/*.yml",
      "**/group_vars/*.yaml",
      "**/host_vars/*.yml",
      "**/host_vars/*.yaml",
      "**/playbooks/*.yml",
      "**/playbooks/*.yaml",
      "**playbook*.yml",
      "**playbook*.yaml"
    ]
  }
```

このリストは必要に応じて自由に変更してください。

#### インベントリ

インベントリファイルが YAML 形式なら、次のいずれかの方法を選べます。

- インベントリファイルの先頭に次のコメントを追加して、`ansible-lint` の
  インベントリ JSON スキーマを付与する:

```yml
# yaml-language-server: $schema=https://raw.githubusercontent.com/ansible/ansible-lint/main/src/ansiblelint/schemas/inventory.json
```

- あるいは Gram 設定で YAML 言語サーバーの設定を変更し、インベントリのパターンに
  一致するすべてのインベントリファイルにこのスキーマを設定する
  （[参照](./yaml.md)）:

```json
"lsp": {
    "yaml-language-server": {
      "settings": {
        "yaml": {
          "schemas": {
            "https://raw.githubusercontent.com/ansible/ansible-lint/main/src/ansiblelint/schemas/inventory.json": [
              "./inventory/*.yaml",
              "hosts.yml",
            ]
          }
        }
      }
    }
},
```

### LSP の構成

既定では、次のデフォルト設定が Ansible 言語サーバーに渡されます。これは
[nvim-lspconfig](https://github.com/neovim/nvim-lspconfig/blob/03bc581e05e81d33808b42b2d7e76d70adb3b595/lua/lspconfig/configs/ansiblels.lua)
が Ansible 言語サーバーに設定しているデフォルトをそのまま反映したものです:

```json
{
  "ansible": {
    "ansible": {
      "path": "ansible",
    },
    "executionEnvironment": {
      "enabled": false,
    },
    "python": {
      "interpreterPath": "python3",
    },
    "validation": {
      "enabled": true,
      "lint": {
        "enabled": true,
        "path": "ansible-lint",
      },
    },
  },
}
```

> [!NOTE] リンティングを機能させるには、`ansible-lint` がインストールされ、
> PATH から見つけられることを確認してください。

必要に応じて、上記のデフォルト設定は Gram 設定ファイルの `"lsp"` セクションで
上書きできます。例:

```json
"lsp": {
  // 注意: Gram の Ansible 拡張機能はすべての設定に `ansible` プレフィックスを
  // 付けるため、`ansible.ansible.path` ではなく `ansible.path` を使用します。
  "ansible-language-server": {
    "settings": {
      "ansible": {
        "path": "ansible"
      },
      "executionEnvironment": {
        "enabled": false
      },
      "python": {
        "interpreterPath": "python3"
      },
      "validation": {
        "enabled": false, // 検証を無効化
        "lint": {
          "enabled": false, // ansible-lint を無効化
          "path": "ansible-lint"
        }
      }
    }
  }
}
```

サーバーに渡せるオプション/設定の完全な一覧は、プロジェクトのページ
（[こちら](https://github.com/ansible/vscode-ansible/blob/5a89836d66d470fb9d20e7ea8aa2af96f12f61fb/docs/als/settings.md)）にあります。
オプションの値は必要に応じて変更してください。
