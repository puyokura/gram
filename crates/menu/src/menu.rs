use gpui::actions;

// If the editor binary doesn't use anything in this crate, it will be optimized away
// and the actions won't initialize. So we just provide an empty initialization function
// to be called from main.
//
// These may provide relevant context:
// https://github.com/rust-lang/rust/issues/47384
// https://github.com/mmastrac/rust-ctor/issues/280
pub fn init() {}

actions!(
    menu,
    [
        /// 現在のメニュー操作をキャンセルします。
        Cancel,
        /// 選択中のメニュー項目を確定します。
        Confirm,
        /// セカンダリ確定アクションを実行します。
        SecondaryConfirm,
        /// メニュー内の前の項目を選択します。
        SelectPrevious,
        /// メニュー内の次の項目を選択します。
        SelectNext,
        /// メニュー内の最初の項目を選択します。
        SelectFirst,
        /// メニュー内の最後の項目を選択します。
        SelectLast,
        /// サブメニューに入ります（子メニューへ移動）。
        SelectChild,
        /// サブメニューから出ます（親メニューへ移動）。
        SelectParent,
        /// メニューを最初からやり直します。
        Restart,
        EndSlot,
    ]
);
