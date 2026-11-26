[CmdletBinding()]
Param(
    [Parameter()][string]$OutDir = "dist/gram-portable-windows-x86_64",
    [Parameter()][switch]$UseSpectreStub,
    [Parameter()][switch]$SkipZip
)

$ErrorActionPreference = 'Stop'
$PSNativeCommandUseErrorActionPreference = $true

$repoRoot = Split-Path -Parent $PSScriptRoot
Push-Location $repoRoot

try {
    $env:RUSTC_WRAPPER = ""
    $cargoArgs = @(
        "build",
        "--release",
        "--package", "gram",
        "--package", "cli",
        "--features", "gram/portable-offline"
    )

    $stubDir = $null
    if ($UseSpectreStub) {
        $stubDir = Join-Path $env:TEMP "gram-msvc-spectre-stub"
        $stubSrc = Join-Path $stubDir "src"
        New-Item -ItemType Directory -Path $stubSrc -Force | Out-Null
        Set-Content -Path (Join-Path $stubDir "Cargo.toml") -Encoding UTF8 -Value @'
[package]
name = "msvc_spectre_libs"
version = "0.1.3"
edition = "2021"

[features]
error = []
'@
        Set-Content -Path (Join-Path $stubSrc "lib.rs") -Encoding UTF8 -Value ""
        $stubTomlPath = $stubDir.Replace('\', '/')
        $cargoArgs += @("--config", "patch.crates-io.msvc_spectre_libs.path='$stubTomlPath'")
    }

    Write-Host "Building Gram (Windows portable + offline release)..."
    & cargo @cargoArgs

    $releaseDir = Join-Path $repoRoot "target/release"
    $gramExe = Join-Path $releaseDir "gram.exe"
    $cliExe = Join-Path $releaseDir "cli.exe"
    if (-not (Test-Path $gramExe)) { throw "Missing built binary: $gramExe" }
    if (-not (Test-Path $cliExe)) { throw "Missing built binary: $cliExe" }

    $bundleDir = if ([System.IO.Path]::IsPathRooted($OutDir)) { $OutDir } else { Join-Path $repoRoot $OutDir }
    if (Test-Path $bundleDir) {
        Remove-Item -Path $bundleDir -Recurse -Force
    }
    New-Item -ItemType Directory -Path (Join-Path $bundleDir "bin") -Force | Out-Null
    New-Item -ItemType Directory -Path (Join-Path $bundleDir "data/config") -Force | Out-Null
    New-Item -ItemType Directory -Path (Join-Path $bundleDir "data/extensions") -Force | Out-Null

    Copy-Item -Path $gramExe -Destination (Join-Path $bundleDir "Gram.exe") -Force
    Copy-Item -Path $cliExe -Destination (Join-Path $bundleDir "bin/gram.exe") -Force

    # Bundle ConPTY runtime files if produced by build scripts
    foreach ($aux in @("conpty.dll", "OpenConsole.exe")) {
        $found = Get-ChildItem -Path $releaseDir -Filter $aux -Recurse -ErrorAction SilentlyContinue | Select-Object -First 1
        if ($found) {
            Copy-Item -Path $found.FullName -Destination (Join-Path $bundleDir $aux) -Force
        }
    }

    # Portable & Offline markers (also compiled in via portable-offline feature)
    Set-Content -Path (Join-Path $bundleDir "portable.txt") -Encoding UTF8 -Value "Gram Portable Mode: all settings, data, logs, cache, and extensions are stored under ./data"
    Set-Content -Path (Join-Path $bundleDir "offline.txt") -Encoding UTF8 -Value "Gram Offline Mode: all outbound HTTP requests and auto-downloads are disabled"

    # Default offline-safe portable settings
    Set-Content -Path (Join-Path $bundleDir "data/config/settings.json") -Encoding UTF8 -Value @'
{
  "node": {
    "allow_binary_download": false,
    "allow_prettier_download": false,
    "allow_npm_install": false
  },
  "granted_extension_capabilities": []
}
'@

    Set-Content -Path (Join-Path $bundleDir "README-PORTABLE.txt") -Encoding UTF8 -Value @'
Gram Windows 完全ポータブル・完全オフライン版

特徴:
- レジストリ登録完全なし（インストーラ不要、フォルダ削除のみで完全に元通り）
- 完全ポータブル（設定・DB・ログ・一時ファイル・拡張機能はすべて ./data 配下に保存）
- 完全オフライン（すべての外部 HTTP 通信・自動更新・バイナリ自動ダウンロードを遮断）

起動方法:
- Gram.exe を直接起動してください。
- CLI からは bin\gram.exe を利用できます。
'@

    if (-not $SkipZip) {
        $zipPath = "$bundleDir.zip"
        if (Test-Path $zipPath) {
            Remove-Item -Path $zipPath -Force
        }
        Compress-Archive -Path (Join-Path $bundleDir "*") -DestinationPath $zipPath -Force
        Write-Host "Created portable archive: $zipPath"
    }

    Write-Host "Created portable bundle: $bundleDir"
} finally {
    Pop-Location
}
