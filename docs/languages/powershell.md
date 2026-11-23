# PowerShell

Gram での PowerShell 言語サポートは、コミュニティが管理する
[Gram PowerShell 拡張機能](https://github.com/wingyplus/zed-powershell) によって提供されます。問題の報告先:
[github.com/wingyplus/zed-powershell/issues](https://github.com/wingyplus/zed-powershell/issues)

- Tree-sitter:
  [airbus-cert/tree-sitter-powershell](https://github.com/airbus-cert/tree-sitter-powershell)
- 言語サーバー:
  [PowerShell/PowerShellEditorServices](https://github.com/PowerShell/PowerShellEditorServices)

## セットアップ

### PowerShell 7+ のインストール {#powershell-install}

- macOS: `brew install powershell/tap/powershell`
- Alpine:
  [Alpine Linux への PowerShell のインストール](https://learn.microsoft.com/en-us/powershell/scripting/install/install-alpine)
- Debian:
  [Debian Linux への PowerShell のインストール](https://learn.microsoft.com/en-us/powershell/scripting/install/install-debian)
- RedHat:
  [RHEL への PowerShell のインストール](https://learn.microsoft.com/en-us/powershell/scripting/install/install-rhel)
- Ubuntu:
  [Ubuntu への PowerShell のインストール](https://learn.microsoft.com/en-us/powershell/scripting/install/install-ubuntu)
- Windows:
  [Windows への PowerShell のインストール](https://learn.microsoft.com/en-us/powershell/scripting/install/installing-powershell-on-windows)

Gram PowerShell 拡張機能は、PATH 内で見つかった `pwsh` 実行可能ファイルを既定で使用します。

### PowerShell Editor Services のインストール（任意） {#powershell-editor-services}

Gram PowerShell 拡張機能は
[PowerShell Editor Services](https://github.com/PowerShell/PowerShellEditorServices)
を自動的にダウンロードしようとします。

特定のバイナリを使用したい場合は、Gram の `settings.json` で指定できます:

```json
  "lsp": {
    "powershell-es": {
      "binary": {
        "path": "/path/to/PowerShellEditorServices"
      }
    }
  }
```
