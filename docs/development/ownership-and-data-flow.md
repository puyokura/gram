# GPUI: 所有権とデータフロー

GPUI では、アプリケーション内のすべてのモデルとビューは、実際には `App` という単一のトップレベルオブジェクトが所有します。新しいエンティティやビュー（総称して _エンティティ_ と呼びます）が作成されると、アプリケーションはそれらの状態の所有権を受け取り、各種のアプリサービスへの参加や他のエンティティとのやり取りを可能にします。

これを示すために、次の簡単なアプリを考えます。`run` をコールバック付きで呼び出してアプリを開始し、コールバックにはアプリケーションのすべての状態を所有する `App` への参照が渡されます。この `App` は、ウィンドウを開く、ダイアログを表示するといったアプリケーションレベルのサービスへの入り口です。また `insert_entity` メソッドを持ち、以下ではこれを呼び出してエンティティを作成し、その所有権をアプリケーションに渡しています。

```no_run
# use gpui::{App, AppContext, Application, Entity};
# struct Counter {
#     count: usize,
# }
Application::new().run(|cx: &mut App| {
    let _counter: Entity<Counter> = cx.new(|_cx| Counter { count: 0 });
    // ...
});
```

`new_entity` の呼び出しは _エンティティハンドル_ を返し、参照するオブジェクトの型に基づく型パラメーターを持ちます。この `Entity<Counter>` ハンドル自体は、エンティティの状態へのアクセス手段を提供しません。単なる不活性な識別子とコンパイル時の型タグにすぎず、アプリが所有する実際の `Counter` オブジェクトへの参照カウント付きポインターを保持しています。

Rust 標準ライブラリの `Rc` とよく似て、この参照カウントはハンドルがクローンされると増え、ドロップされると減り、基になるモデルの共有所有を可能にします。ただし `Rc` と異なり、モデルの状態へのアクセスは `App` への参照が利用可能なときにのみ提供されます。ハンドルは本当の意味で状態を _所有_ しておらず、真の所有者である `App` から状態にアクセスするために使えます。簡潔にするためセットアップコードの一部を省くと:

```no_run
# use gpui::{App, AppContext, Application, Context, Entity};
# struct Counter {
#     count: usize,
# }
Application::new().run(|cx: &mut App| {
    let counter: Entity<Counter> = cx.new(|_cx| Counter { count: 0 });
    // Call `update` to access the model's state.
    counter.update(cx, |counter: &mut Counter, _cx: &mut Context<Counter>| {
        counter.count += 1;
    });
});
```

カウンターを更新するには、ハンドルの `update` を呼び出し、コンテキスト参照とコールバックを渡します。コールバックにはカウンターへの可変参照が渡され、状態を操作できます。

コールバックには2つ目の `Context<Counter>` 参照も渡されます。この参照は `run` コールバックに渡される `App` 参照に似ています。`Context` は実際には `App` のラッパーで、どの特定のエンティティに結び付いているかを示す追加データを含みます。ここではカウンターです。

`App` が提供するアプリケーションレベルのサービスに加えて、`Context` はエンティティレベルのサービスへのアクセスも提供します。たとえば、このエンティティの状態が変化したことをオブザーバーに通知するために使えます。`cx.notify()` を呼び出して、例にこれを追加しましょう。

```no_run
# use gpui::{App, AppContext, Application, Entity};
# struct Counter {
#     count: usize,
# }
Application::new().run(|cx: &mut App| {
    let counter: Entity<Counter> = cx.new(|_cx| Counter { count: 0 });
    counter.update(cx, |counter, cx| {
        counter.count += 1;
        cx.notify(); // Notify observers
    });
});
```

次に、これらの通知を監視して反応させる必要があります。カウンターを更新する前に、それを監視する2つ目のカウンターを構築します。1つ目のカウンターが変わるたびに、そのカウントの2倍が2つ目のカウンターに代入されます。2つ目のカウンターに属する `Context` に対して `observe` を呼び出すことで、1つ目のカウンターが通知するたびに2つ目も通知されるようにしている点に注目してください。`observe` の呼び出しは `Subscription` を返し、これを `detach` することで、両方のカウンターが存在する限りこの振る舞いを維持します。この購読を保存しておき、任意のタイミングでドロップして振る舞いをキャンセルすることもできます。

`observe` のコールバックには、オブザーバーへの可変参照と、監視対象のカウンターの _ハンドル_ が渡されます。その状態には `read` メソッドでアクセスします。

```no_run
 # use gpui::{App, AppContext, Application, Entity, prelude::*};
 # struct Counter {
 #     count: usize,
 # }
 Application::new().run(|cx: &mut App| {
     let first_counter: Entity<Counter> = cx.new(|_cx| Counter { count: 0 });

     let second_counter = cx.new(|cx: &mut Context<Counter>| {
         // Note we can set up the callback before the Counter is even created!
         cx.observe(
             &first_counter,
             |second: &mut Counter, first: Entity<Counter>, cx| {
                 second.count = first.read(cx).count * 2;
             },
         )
         .detach();

         Counter { count: 0 }
     });

     first_counter.update(cx, |counter, cx| {
         counter.count += 1;
         cx.notify();
     });

     assert_eq!(second_counter.read(cx).count, 2);
 });
```

1つ目のカウンターを更新した後、監視するカウンターの状態が購読に従って維持されていることを確認できます。

エンティティの状態が変化したことを示す `observe` と `notify` に加えて、GPUI は型付きイベントを発行できる `subscribe` と `emit` も提供します。この仕組みを使うには、発行するオブジェクトが `EventEmitter` トレイトを実装している必要があります。

`CounterChangeEvent` という新しいイベント型を導入し、`Counter` がこの型のイベントを発行できることを示しましょう:

```no_run
use gpui::EventEmitter;
# struct Counter {
#     count: usize,
# }
struct CounterChangeEvent {
    increment: usize,
}

impl EventEmitter<CounterChangeEvent> for Counter {}
```

次に、監視を購読に置き換えて例を更新します。カウンターが増分されるたびに、増分の大きさを示す `Change` イベントが発行されます。

```no_run
# use gpui::{App, AppContext, Application, Context, Entity, EventEmitter};
# struct Counter {
#     count: usize,
# }
# struct CounterChangeEvent {
#     increment: usize,
# }
# impl EventEmitter<CounterChangeEvent> for Counter {}
Application::new().run(|cx: &mut App| {
    let first_counter: Entity<Counter> = cx.new(|_cx| Counter { count: 0 });

    let second_counter = cx.new(|cx: &mut Context<Counter>| {
        // Note we can set up the callback before the Counter is even created!
        cx.subscribe(&first_counter, |second: &mut Counter, _first: Entity<Counter>, event, _cx| {
            second.count += event.increment * 2;
        })
        .detach();

        Counter {
            count: first_counter.read(cx).count * 2,
        }
    });

    first_counter.update(cx, |first, cx| {
        first.count += 2;
        cx.emit(CounterChangeEvent { increment: 2 });
        cx.notify();
    });

    assert_eq!(second_counter.read(cx).count, 4);
});
```
