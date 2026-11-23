use gpui::{Action, Context, Div, Entity, InteractiveElement, Window, div};
use workspace::Workspace;

use crate::BufferSearchBar;

/// Registrar は search とその後続の利用者との依存関係を反転させ、後続の利用者が検索アクションの種類を知らずに登録できるようにします。
pub trait SearchActionsRegistrar {
    fn register_handler<A: Action>(&mut self, callback: impl ActionExecutor<A>);
}

type SearchBarActionCallback<A> = fn(&mut BufferSearchBar, &A, &mut Window, &mut Context<BufferSearchBar>);

type GetSearchBar<T> = for<'a, 'b> fn(&'a T, &'a mut Window, &mut Context<'b, T>) -> Option<Entity<BufferSearchBar>>;

/// 取り出し可能な div に検索アクションを登録します。
pub struct DivRegistrar<'a, 'b, T: 'static> {
    div: Option<Div>,
    cx: &'a mut Context<'b, T>,
    search_getter: GetSearchBar<T>,
}

impl<'a, 'b, T: 'static> DivRegistrar<'a, 'b, T> {
    pub fn new(search_getter: GetSearchBar<T>, cx: &'a mut Context<'b, T>) -> Self {
        Self {
            div: Some(div()),
            cx,
            search_getter,
        }
    }
    pub fn into_div(self) -> Div {
        // This option is always Some; it's an option in the first place because we want to call methods
        // on div that require ownership.
        self.div.unwrap()
    }
}

impl<T: 'static> SearchActionsRegistrar for DivRegistrar<'_, '_, T> {
    fn register_handler<A: Action>(&mut self, callback: impl ActionExecutor<A>) {
        let getter = self.search_getter;
        self.div = self.div.take().map(|div| {
            div.on_action(self.cx.listener(move |this, action, window, cx| {
                let should_notify = (getter)(this, window, cx)
                    .map(|search_bar| {
                        search_bar.update(cx, |search_bar, cx| callback.execute(search_bar, action, window, cx))
                    })
                    .unwrap_or(false);
                if should_notify {
                    cx.notify();
                } else {
                    cx.propagate();
                }
            }))
        });
    }
}

/// アクティブなペインにアクションを登録します。
impl SearchActionsRegistrar for Workspace {
    fn register_handler<A: Action>(&mut self, callback: impl ActionExecutor<A>) {
        self.register_action(move |workspace, action: &A, window, cx| {
            if workspace.has_active_modal(window, cx) && !workspace.hide_modal(window, cx) {
                cx.propagate();
                return;
            }

            let pane = workspace.active_pane();
            let callback = callback.clone();
            pane.update(cx, |this, cx| {
                this.toolbar().update(cx, move |this, cx| {
                    if let Some(search_bar) = this.item_of_type::<BufferSearchBar>() {
                        let should_notify = search_bar.update(cx, move |search_bar, cx| {
                            callback.execute(search_bar, action, window, cx)
                        });
                        if should_notify {
                            cx.notify();
                        } else {
                            cx.propagate();
                        }
                    }
                })
            });
        });
    }
}

type DidHandleAction = bool;
/// 前提条件が満たされた場合に基になるアクションを実行する可能性があります（例: バッファー検索バーが表示されている）。
pub trait ActionExecutor<A: Action>: 'static + Clone {
    fn execute(
        &self,
        search_bar: &mut BufferSearchBar,
        action: &A,
        window: &mut Window,
        cx: &mut Context<BufferSearchBar>,
    ) -> DidHandleAction;
}

/// 検索バーがパネルから閉じられたときにアクションを実行します。
pub struct ForDismissed<A>(pub(super) SearchBarActionCallback<A>);
impl<A> Clone for ForDismissed<A> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<A: Action> ActionExecutor<A> for ForDismissed<A> {
    fn execute(
        &self,
        search_bar: &mut BufferSearchBar,
        action: &A,
        window: &mut Window,
        cx: &mut Context<BufferSearchBar>,
    ) -> DidHandleAction {
        if search_bar.is_dismissed() {
            self.0(search_bar, action, window, cx);
            true
        } else {
            false
        }
    }
}

/// 検索バーが展開されているときにアクションを実行します。
pub struct ForDeployed<A>(pub(super) SearchBarActionCallback<A>);
impl<A> Clone for ForDeployed<A> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<A: Action> ActionExecutor<A> for ForDeployed<A> {
    fn execute(
        &self,
        search_bar: &mut BufferSearchBar,
        action: &A,
        window: &mut Window,
        cx: &mut Context<BufferSearchBar>,
    ) -> DidHandleAction {
        if search_bar.is_dismissed() || search_bar.active_searchable_item.is_none() {
            false
        } else {
            self.0(search_bar, action, window, cx);
            true
        }
    }
}

/// 検索バーに一致があるか保留中の外部クエリがあるときにアクションを実行します。
/// 表示されているかどうかに関わらず実行します。
pub struct WithResultsOrExternalQuery<A>(pub(super) SearchBarActionCallback<A>);
impl<A> Clone for WithResultsOrExternalQuery<A> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<A: Action> ActionExecutor<A> for WithResultsOrExternalQuery<A> {
    fn execute(
        &self,
        search_bar: &mut BufferSearchBar,
        action: &A,
        window: &mut Window,
        cx: &mut Context<BufferSearchBar>,
    ) -> DidHandleAction {
        let has_external_query = cfg_select! {
            target_os = "macos" => search_bar.pending_external_query.is_some(),
            _ => false,
        };
        if has_external_query || search_bar.active_match_index.is_some() {
            self.0(search_bar, action, window, cx);
            true
        } else {
            false
        }
    }
}
