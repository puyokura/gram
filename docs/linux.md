# Linux での Gram

[Repology](https://repology.org/project/gram/versions) もあわせて参照してください。

## ディストリビューションパッケージ

推奨されるインストール方法は、自動アップデートに対応する Gram リポジトリを追加することです。また、ビルド済み `.deb` および `.rpm` パッケージも [Gram リリース](https://codeberg.org/GramEditor/gram/releases) のアセットとして入手可能です。

### Alpine

Alpine Linux では [testing](https://pkgs.alpinelinux.org/package/edge/testing/x86_64/gram) リポジトリで公式パッケージが提供されています。testing リポジトリを有効化し、次を実行してください:

```bash
doas apk add gram@testing
```

### Arch

Arch Linux では [extra](https://archlinux.org/packages/extra/x86_64/gram) リポジトリで公式パッケージが提供されています。

```bash
sudo pacman -S gram
```

または、AUR から開発スナップショットをビルドしてインストールすることもできます: [`gram-git`](https://aur.archlinux.org/packages/gram-git)。AUR パッケージの整合性はご自身で検証してください。

### Debian / Ubuntu

Debian 12（Bookworm）、Ubuntu 24.04（Noble）以降が必要です。

```bash
# リポジトリキーを追加
sudo curl https://codeberg.org/api/packages/GramEditor/debian/repository.key -o /etc/apt/keyrings/forgejo-GramEditor.asc
# リポジトリを追加
echo "deb [arch=amd64 signed-by=/etc/apt/keyrings/forgejo-GramEditor.asc] https://codeberg.org/api/packages/GramEditor/debian gram release" | sudo tee -a /etc/apt/sources.list.d/gram.list
# パッケージキャッシュを更新してインストール
sudo apt update && sudo apt install gram
```

### Fedora / RHEL / Rocky / Alma

Fedora 42、RHEL 10.1、Rocky 10.1、Alma 10.1 以降が必要です。

#### Codeberg リポジトリ

```bash
sudo dnf config-manager addrepo --from-repofile="https://codeberg.org/api/packages/GramEditor/rpm.repo"
sudo dnf install gram
```

> [!NOTE]
>
> RPM パッケージの署名は現在機能していません。Gram をインストールするには、`/etc/yum.repos.d/rpm.repo` で `gpgcheck=0` を設定してください。

#### Terra リポジトリ

```bash
sudo dnf install --nogpgcheck --repofrompath 'terra,https://repos.fyralabs.com/terra$releasever' terra-release && sudo dnf install gram

# イミュータブル / アトミックエディションの場合
curl -fsSL https://github.com/terrapkg/subatomic-repos/raw/main/terra.repo | pkexec tee /etc/yum.repos.d/terra.repo
sudo rpm-ostree install terra-release && sudo dnf install gram
```

### Gentoo

> [!NOTE]
>
> Gentoo パッケージは Gram のオーバーレイ経由で提供されています。詳細およびインストール方法については、[Gram のオーバーレイリポジトリ](https://codeberg.org/GramEditor/gram-gentoo) を参照してください。

```bash
sudo eselect repository add gram git https://codeberg.org/GramEditor/gram-gentoo.git
sudo emerge --sync gram
sudo emerge -av app-editors/gram
```

### Nix

参照: <https://github.com/NixOS/nixpkgs/tree/master/pkgs/by-name/gr/gram>。

#### Modern Nix（推奨）

```bash
# 一時的な環境
nix shell nixpkgs#gram

# ユーザープロファイルへのインストール
nix profile add nixpkgs#gram
```

#### NixOS 設定

```nix
environment.systemPackages = [
  pkgs.gram
];
```

#### Legacy Nix（チャンネル）

```bash
# NixOS の場合
nix-env -iA nixos.gram

# NixOS 以外の Linux の場合
nix-env -iA nixpkgs.gram
```

### OpenSUSE

Leap 16（または Tumbleweed / Slowroll）以降が必要です。

```bash
sudo zypper addrepo https://codeberg.org/api/packages/GramEditor/rpm.repo
sudo zypper install gram
```

> [!NOTE]
>
> RPM パッケージの署名は現在機能していません。Gram をインストールするには、`/etc/yum.repos.d/rpm.repo` で `gpgcheck=0` を設定してください。

### Solus

```bash
sudo eopkg install gram
```

## Flatpak

ビルド済みの Flatpak パッケージは [Gram リリース](https://codeberg.org/GramEditor/gram/releases) のリリースアセットとして入手できます。以下を実行してインストールしてください:

```bash
flatpak install /path/to/app.liten.Gram-x86_64-${version}.flatpak
```

## Tarball からのインストール

ご利用のアーキテクチャ向けの tarball が [Gram リリース](https://codeberg.org/GramEditor/gram/releases) リポジトリで入手可能な場合:

1. [install.sh](https://codeberg.org/GramEditor/gram/raw/branch/main/script/install.sh) スクリプトをダウンロードします。
2. スクリプトを実行します。

```bash
./install.sh
```

これにより、最新の Gram リリースがダウンロードされ `$HOME/.local` にインストールされます。別の場所にインストールする場合は `--prefix` を指定してください:

```bash
./install.sh --prefix /usr/local ./gram-linux-x86_64-VERSION.tar.gz
```

`VERSION` は実際のリリースバージョンに置き換えてください。標準以外のプレフィックスを指定した場合は、ランチャーやアイコンを認識できるようにデスクトップのデータ検索パス（data search path）にその場所を含める必要があります。

## ソースからのビルド

Gram はオープンソースです。ビルド手順については [開発者向けノート](./development/linux.md) を参照してください。

## トラブルシューティング

### グラフィックスの問題

#### Gram がウィンドウを開けない

Gram は GPU を必要とし、レンダリングに [Vulkan](https://www.vulkan.org) を使用します。Gram の読み込みに失敗したり動作が重かったりする場合、Vulkan が原因である可能性があります。

`Gram failed to open a window: NoSupportedDeviceFound` エラーは、Vulkan が互換性のある GPU を検出できないことを示しています。トラブルシューティングを行うには、[`vkcube`](https://github.com/krh/vkcube) コマンド（通常は `vulkaninfo` または `vulkan-tools` パッケージに含まれています）を実行してください。

> [!NOTE]
>
> `vkcube -m x11` または `vkcube -m wayland` を実行して、X11 と Wayland の両方のモードをテストしてください。ディストリビューションによっては、Wayland 向けに `vkcube-wayland` を使用します。

これによりグラフィックス設定情報が出力され、回転する立方体が表示されます。これが失敗する場合は、Vulkan 互換の GPU ドライバーをインストールしてください。なお、一部のハードウェアは Vulkan をサポートしていません。

Gram が使用している GPU を確認するには、`~/.local/share/gram/logs/Gram.log` 内で `Using GPU: ...` を検索してください。

`ERROR_INITIALIZATION_FAILED`、`GPU Crashed`、`ERROR_SURFACE_LOST_KHR` などのエラーは、別の GPU ドライバーをインストールするか、別の GPU を選択することで解決する場合があります（[#14225](https://github.com/zed-industries/zed/issues/14225) を参照）。

- **PRIME**: 一部のシステムでは、`/etc/prime-discrete` を `on`（ディスクリートを強制）または `off`（統合GPUを強制）に設定します。
- **DRI_PRIME**: Gram を実行する際に環境変数 `DRI_PRIME=1` を設定して、ディスクリート GPU の使用を強制します。
- **AMD の長い行**: AMD GPU で長い行を選択した際に Gram がクラッシュする場合は、`GRAM_PATH_SAMPLE_COUNT=0` を設定してください（[#26143](https://github.com/zed-industries/zed/issues/26143) を参照）。
- **AMD の Broken Pipe**: AMD で「Broken Pipe」エラーが発生する場合は、RADV または Mesa ドライバーを試してください（[#13880](https://github.com/zed-industries/zed/issues/13880) を参照）。
- **AMDVLK**: デフォルトの `amdvlk` ドライバーで Gram の起動が一貫して失敗する場合（例: Omarchy 上、[#28851](https://github.com/zed-industries/zed/issues/28851) を参照）、`amdvlk` と `lib32-amdvlk` を削除し、代わりに `vulkan-radeon` をインストールしてください（[#14141](https://github.com/zed-industries/zed/issues/14141) を参照）。

詳細については、[Arch Linux Vulkan ガイド](https://wiki.archlinux.org/title/Vulkan) を参照してください。

#### Gram に特定の GPU を強制使用させる

**方法 A: 環境変数** `GRAM_DEVICE_ID={device_id}` を使用して GPU を指定します。デバイス ID は `lspci -nn | grep VGA` で確認できます:

```bash
08:00.0 VGA compatible controller [0300]: NVIDIA Corporation GA104 [GeForce RTX 3070] [10de:2484] (rev a1)
```

この例ではデバイス ID は `2484`（16進数）です。Gram にこの GPU の使用を強制します:

```bash
GRAM_DEVICE_ID=0x2484 gram
```

グローバルに定義する場合は、`.bashrc` などでこの環境変数をエクスポートしてください。

**方法 B: Mesa Device Select** Mesa を使用している場合、利用可能な GPU を一覧表示します:

```bash
MESA_VK_DEVICE_SELECT=list gram --foreground
```

次に `MESA_VK_DEVICE_SELECT=xxxx:yyyy` をエクスポートして特定のデバイスを選択します。Xwayland へフォールバックする場合は、さらに `WAYLAND_DISPLAY=""` をエクスポートしてください。

**方法 C: VKDeviceChooser** [vkdevicechooser](https://github.com/jiriks74/vkdevicechooser) を使用します。

#### デバッグレポートの生成

`--system-specs` を渡すとシステム仕様が出力されます:

```bash
gram --system-specs
```

エディターのログは `~/.local/share/gram/logs/Gram.log` に配置されています。グラフィックスの問題をデバッグするためのクリーンなログを生成するには:

```bash
truncate -s 0 ~/.local/share/gram/logs/Gram.log # ログファイルをクリア
GRAM_LOG=wgpu=info gram .
cat ~/.local/share/gram/logs/Gram.log
# 出力をコピー
```

Gram CLI が設定されている場合は、次のように実行することもできます:

```bash
GRAM_LOG=wgpu=info /path/to/gram/cli --foreground .
# 出力をコピー
```

### X11 スケールファクターの強制設定

X11 では、Gram は次の優先順位で高 DPI ディスプレイのスケールファクターを自動検出します:

1. `GPUI_X11_SCALE_FACTOR` 環境変数（設定されている場合）
2. X リソースデータベース（xrdb）の `Xft.dpi`
3. RandR による自動検出（モニターの解像度と物理サイズ）

スケールファクターをカスタマイズするには:

#### 現在のスケールファクターの確認

`Xft.dpi` が設定されているか確認します:

```bash
xrdb -query | grep Xft.dpi
```

空の場合は、Gram は RandR を使用して自動計算します。

#### 方法 1: Xft.dpi の設定（X リソースデータベース）

`Xft.dpi` を設定すると、X11 アプリケーション全体で一貫したスケーリングが適用されます。`~/.Xresources` を編集または作成してください:

```bash
vim ~/.Xresources
```

希望する DPI を追加します（例: 1x の場合は `96`、1.5x の場合は `144`、2x の場合は `192`、3x の場合は `288`）:

```ini
Xft.dpi: 96
```

設定を読み込みます:

```bash
xrdb -merge ~/.Xresources
```

Gram を再起動します。

#### 方法 2: GPUI_X11_SCALE_FACTOR の使用

この Gram 固有の環境変数は、自動検出をオーバーライドします。

```bash
GPUI_X11_SCALE_FACTOR=1.5 gram
```

小数値（例: `1.25`, `1.5`, `2.0`）、または `Xft.dpi` が設定されていても RandR 検出を強制する `randr` を指定できます。永続化するには、シェルプロファイルまたはデスクトップエントリに追加してください。

#### 方法 3: システム全体の RandR DPI の調整

X11 セッション全体で報告される DPI を変更します。`.xprofile` または `.xinitrc` に追加してください:

```bash
xrandr --dpi 192
```

`192` を希望する DPI に置き換えてください。

### フォントレンダリングパラメータ

Linux では、Gram はフォントレンダリングに関して次の環境変数を読み取ります:

- `GRAM_FONTS_GAMMA`: [GetGamma](https://learn.microsoft.com/en-us/windows/win32/api/dwrite/nf-dwrite-idwriterenderingparams-getgamma) の値に対応します。許容範囲: `[1.0, 2.2]`（範囲外は切り詰め）。デフォルト: `1.8`。
- `GRAM_FONTS_GRAYSCALE_ENHANCED_CONTRAST`: [GetGrayscaleEnhancedContrast](https://learn.microsoft.com/en-us/windows/win32/api/dwrite_1/nf-dwrite_1-idwriterenderingparams1-getgrayscaleenhancedcontrast) の値に対応します。許容範囲: `[0.0, ∞)`（範囲外は切り詰め）。デフォルト: `1.0`。
