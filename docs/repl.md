# REPL

## はじめに

[Jupyter カーネル](https://docs.jupyter.org/en/latest/projects/kernels.html) のパワーをエディターに！ Gram に組み込まれた REPL 機能により、ノートブックのように手元のテキストファイルを使ってコードを対話的に実行できます。

## インストール

Gram は複数言語でのコード実行をサポートしています。利用を開始するには、使用したい言語のカーネルをインストールする必要があります。

**現在サポートされている言語:**

- [Python (ipykernel)](#python)
- [TypeScript (Deno)](#typescript-deno)
- [R (Ark)](#r-ark)
- [R (Xeus)](#r-xeus)
- [Julia](#julia)
- [Scala (Almond)](#scala)

カーネルのインストールが完了すると、各言語のファイルや、Markdown などのそれらの言語ブロックを含む場所で REPL を使用できるようになります。カーネルを新しく追加した場合は、`repl: refresh kernelspecs` コマンドを実行してエディター上で認識させてください。

## REPL の使用方法

REPL を開始するには、対象言語のファイルを開き、`repl: run` コマンド（macOS のデフォルトは `ctrl-shift-enter`）を実行してブロック、選択範囲、または行を実行します。ツールバーの REPL アイコンをクリックすることでも実行できます。

`repl: run` コマンドは選択範囲に対して実行され、結果が選択範囲の直下にインライン表示されます。

出力結果は `repl: clear outputs` コマンドを実行するか、ツールバーの REPL メニューからクリアできます。

### セルモード

Gram は、Python では `# %%`、TypeScript では `// %%` というセル区切り記号を使用した [スクリプト形式ノートブック](https://jupytext.readthedocs.io/en/latest/formats-scripts.html) をサポートしています。これにより、単一のファイルにコードを記述し、ノートブックのようにセル単位で実行できます。

`repl: run` コマンドは、`# %%` マーカーで区切られた各コードブロックを個別のセルとして実行します。

```python
# %% Cell 1
import time
import numpy as np

# %% Cell 2
import matplotlib.pyplot as plt
import matplotlib.pyplot as plt
from matplotlib import style

style.use("ggplot")
```

## 言語別のセットアップ手順

### Python {#python}

#### グローバル環境

<div class="warning">

macOS では、システムの Python は動作_しません_。[pyenv](https://github.com/pyenv/pyenv?tab=readme-ov-file#installation) をセットアップするか、仮想環境（venv）を使用してください。

</div>

現在の Python 環境でカーネルを利用可能にするには、次を実行します:

```sh
pip install ipykernel
python -m ipykernel install --user
```

#### Conda 環境

```sh
source activate myenv
conda install ipykernel
python -m ipykernel install --user --name myenv --display-name "Python (myenv)"
```

#### pip による仮想環境（virtualenv）

```sh
source activate myenv
pip install ipykernel
python -m ipykernel install --user --name myenv --display-name "Python (myenv)"
```

### R (Ark Kernel) {#r-ark}

お使いの OS 向けのリリースをダウンロードして [Ark](https://github.com/posit-dev/ark/releases) をインストールします。例えば macOS の場合は、`ark` バイナリを展開して `/usr/local/bin` に配置します。その後、次を実行します:

```sh
ark --install
```

### R (Xeus Kernel) {#r-xeus}

- [Xeus-R](https://github.com/jupyter-xeus/xeus-r) をインストールします。
- Gram 用の R 拡張機能をインストールします（Gram の拡張機能で `R` を検索）。

<!--
TBD: Improve R REPL (Ark Kernel) instructions
-->

### TypeScript: Deno {#typescript-deno}

- [Deno をインストール](https://docs.deno.com/runtime/manual/getting_started/installation/) した後、Deno の Jupyter カーネルをインストールします:

```sh
deno jupyter --install
```

<!--
TBD: Improve R REPL (Ark Kernel) instructions
-->

### Julia

- [公式サイト](https://julialang.org/downloads/) から Julia をダウンロードしてインストールします。
- Gram 用の Julia 拡張機能をインストールします（Gram の拡張機能で `Julia` を検索）。

<!--
TBD: Improve Julia REPL instructions
-->

### Scala

- `cs setup`（Coursier）を使用して [Scala をインストール](https://www.scala-lang.org/download/) します:
  - `brew install coursier/formulas/coursier && cs setup`
- REPL (Almond) の [セットアップ手順](https://almond.sh/docs/quick-start-install):
  - `brew install --cask temurin`（Eclipse 財団公式の OpenJDK バイナリ）
  - `brew install coursier/formulas/coursier && cs setup`
  - `coursier launch --use-bootstrap almond -- --install`

## 言語ごとに使用するカーネルの変更 {#changing-kernels}

Gram はシステム上で利用可能なカーネルを自動検出します。特定の言語に対して別のデフォルトカーネルを設定したい場合は、`settings.json` でサポート対象言語ごとにカーネルを割り当てることができます。

```json
{
  "jupyter": {
    "kernel_selections": {
      "python": "conda-env",
      "typescript": "deno",
      "javascript": "deno",
      "r": "ark",
    },
  },
}
```

## カーネルスペックの確認・デバッグ

利用可能なカーネルは `repl: sessions` コマンドで確認できます。実行可能なカーネルを再スキャンして更新するには、`repl: refresh kernelspecs` コマンドを使用してください。

`jupyter` がインストールされている場合は、`jupyter kernelspec list` を実行して利用可能なカーネルを一覧表示できます。

```sh
$ jupyter kernelspec list
Available kernels:
  ark                   /Users/z/Library/Jupyter/kernels/ark
  conda-base            /Users/z/Library/Jupyter/kernels/conda-base
  deno                  /Users/z/Library/Jupyter/kernels/deno
  python-chatlab-dev    /Users/z/Library/Jupyter/kernels/python-chatlab-dev
  python3               /Users/z/Library/Jupyter/kernels/python3
  ruby                  /Users/z/Library/Jupyter/kernels/ruby
  rust                  /Users/z/Library/Jupyter/kernels/rust
```

> 注: Gram は Python 環境内のカーネルを検出するために `sys.prefix` と `CONDA_PREFIX` を最大限活用します。明示的に指定したい場合は、該当の環境に入った状態で `python -m ipykernel install --user --name myenv --display-name "Python (myenv)"` を実行して直接カーネルをインストールしてください。
