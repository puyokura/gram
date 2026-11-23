# 開発: 用語集

## 命名規則

これらは基本的にコードベース全体に当てはまります。ここで Name は任意の名前を指し、例としては `AnyElement` や `LspStore` があります。

- `AnyName`: _name_ の型消去版。`Box<dyn NameTrait>` を想像してください。
- `NameStore`: 操作がローカルで実行されるかリモートで実行されるかを抽象化するラッパー型。

## GPUI

### 状態管理

- `App`: すべてのエンティティを含むアプリケーション全体の状態を保持するシングルトン。重要: `App` は `Send` ではないため、`App` はそれを作成したスレッド（通常はメイン/UI スレッド）にのみ存在します。したがって、`&mut App` を見たときは UI スレッド上にいることを知っておいてください。
- `Context`: 特定の `Entity` に特化した振る舞いを持つ、`App` 構造体のラッパー。`(&mut App, Entity<V>)` と考えてください。特化した振る舞いは `Context` の API に現れます。たとえば `App::spawn` は `AsyncFnOnce(AsyncApp) -> Ret` を取り、`Context::spawn` は `AsyncFnOnce(WeakEntity<V>, AsyncApp) -> Ret` を取ります。
- `AsyncApp`: async コンテキストで使うための、所有権を持つ `App` の版。この型は _依然として_ `Send` ではありません（つまり `AsyncApp` = メインスレッド上にいる）。また、`App` がこのクロージャの実行時点で終了している可能性を考慮し、その使用は失敗しうるものになります。`AsyncApp` の利便性は、通常 `App` と `&mut App` 経由でやり取りする点にあります。`&mut App` は async クロージャでの使用に不便ですが、`AsyncApp` は所有権を持つため、async クロージャでも気軽に使えます。
- `AppContext`: `App`、`AsyncApp`、`Context` とそれぞれのテスト版を抽象化するトレイト。
- `Task`: バックグラウンドまたはフォアグラウンドのエグゼキューター上で実行中、または実行予定の future。通常の Future と異なり、Task は実行を開始するのに `.await` を必要としません。ただし、タスクの結果を得るには await が必要です。
- `Executor`: フォアグラウンドまたはバックグラウンドのスレッドで実行するタスクを生成するために使います。タスクは可能な限りバックグラウンドスレッドで実行してください。
  - `BackgroundExecutor`: `Task` を実行するスレッドプール。
  - `ForegroundExecutor`: `Task` を実行するメインスレッド。
- `Entity`: gpui が管理する構造体への、強い型付きの参照。事実上、`App::EntityMap` へのポインター兼マップキーです。
- `WeakEntity`: すでに存在しない可能性がある `Entity` への、実行時にチェックされる参照。[`std::rc::Weak`](https://doc.rust-lang.org/std/rc/struct.Weak.html) に似ています。
- `Global`: 値が1つだけあり、それが `App` に保存されるシングルトン型。
- `Event`: `Entity` が購読者に送信できるデータ型。
- `Action`: ユーザーのキーボード入力を表し、リスナーが処理できるイベント。例: `file finder: toggle`
- `Observing`: エンティティが変更を通知したときに反応すること。
- `Subscription`: アプリケーションの状態変化に反応するために使うイベントハンドラー。
  1. 発行されたイベントの処理
  2. エンティティの `{new,release,on notify}` の監視

### UI

- `View`: `Render` の実装を通じて `Element` を生成できる `Entity`。
- `Element`: レイアウトして画面に描画できる型。
- `element expression`: エレメントツリーを構築する式。例:

```rust
h_flex()
    .id(text[i])
    .relative()
    .when(selected, |this| {
        this.child(
            div()
                .h_4()
                .absolute()
                etc etc
```

- `Component`: レンダリングして `Element` に変換できるビルダー。
- `Dispatch tree`: TODO
- `Focus`: キー入力が最初に処理される場所。
- `Focus tree`: 現在フォーカスを持つ場所から UI ルートまでの経路。例 <img> TODO

## Gram UI

- `Window`: デスクトップ環境上の Gram ウィンドウを表す Gram 内の構造体（下の画像を参照）。複数の Gram インスタンスを開いている場合は複数存在しえます。主にレンダリングのために引き渡されます。
- `Modal`: UI の他の部分の上に浮かぶ UI 要素。
- `Picker`: UI の上に浮かぶ（Modal）項目リストを表す構造体。項目を選択して確定できます。選択時や確定時に何が起こるかは、そのピッカーのデリゲートによって決まります（下の画像の「Model」はピッカーです）。
- `PickerDelegate`: `Picker` の振る舞いを特化するために使うトレイト。`Picker` は `PickerDelegate` を `delegate` フィールドに保存します。
- `Center`: Gram ウィンドウの中央部分。中央は複数の `Pane` に分割されます。コードベースでは `Workspace` 構造体のフィールドです（下の画像を参照）。
- `Pane`: `Center` 内で項目を配置できる領域。エディター、マルチバッファー、ターミナルなどを置けます（下の画像を参照）。
- `Panel`: `Panel` トレイトを実装する `Entity`。`Dock` に配置できます。下の画像では、左ドックの `ProjectPanel` と下部ドックの `DebugPanel` が見えます。なお `Editor` は `Panel` を実装していないため、`Panel` ではありません。
- `Dock`: `Pane` に似た、開閉できる UI 要素。同時に最大3つまで開けます（中央の左右と下）。ドックには `Pane` ではなく、1つ以上の `Panel` が入ります（画像を参照）。

<img width="1921" height="1080" alt="Screenshot for the Pane and Dock features" src="https://github.com/user-attachments/assets/2cb1170e-2850-450d-89bb-73622b5d07b2" />

- `Project`: 1つ以上の `Worktree`。
- `Worktree`: ローカルまたはリモートのファイルを表します。

<img width="552" height="1118" alt="Screenshot for the Worktree feature" src="https://github.com/user-attachments/assets/da5c58e4-b02e-4038-9736-27e3509fdbfa" />

- [マルチバッファー](../multibuffers.md): エディターのリスト。マルチバッファーでは複数ファイルを同時に編集できます。Gram の操作が複数の位置を返したとき（例: _検索_ や _定義へ移動_）にマルチバッファーが開きます。下の画像のプロジェクト検索を参照してください。

<img width="800" height="886" alt="Screenshot for the MultiBuffer feature" src="https://github.com/user-attachments/assets/d59dcecd-8ab6-4172-8fb6-b1fc3c3eaf9d" />

## エディター

- `Editor`: _その_ テキストエディター。Gram のほとんどすべてが `Editor` であり、1行の入力欄でさえそうです。上の画像の各ペインには、1つ以上の `Editor` インスタンスが入っています。
- `Workspace`: ウィンドウのルート。
- `Entry`: ファイル、フォルダー、保留中のフォルダー、または未読み込みのフォルダー。
- `Buffer`: 「ファイル」のメモリ内表現で、構文木、Git の状態、診断などの関連データを伴います。
- `pending selection`: マウスを押してドラッグしている途中で、まだボタンを離していない状態。

## デバッガー

- `DapStore`: デバッガーセッションを管理するエンティティ。
- `debugger::Session`: デバッグセッションのライフサイクルと DAP との通信を管理するエンティティ。
- `BreakpointStore`: ローカルおよびリモートの Gram インスタンスにおけるブレークポイントの状態を管理するエンティティ。
- `DebugSession`: デバッグセッションの UI と実行状態を管理します。
- `RunningState`: デバッグセッションのすべてのビューを直接管理します。
- `VariableList`: デバッグセッションの変数およびウォッチリストのビュー。
- `Console`: TODO
- `Terminal`: TODO
- `BreakpointList`: TODO
