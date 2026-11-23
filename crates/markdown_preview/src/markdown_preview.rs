use gpui::{App, actions};
use workspace::Workspace;

pub mod markdown_elements;
pub mod markdown_parser;
pub mod markdown_preview_view;
pub mod markdown_renderer;

pub use app_actions::preview::markdown::{OpenPreview, OpenPreviewToTheSide};

actions!(
    markdown,
    [
        /// Markdown プレビューを 1 ページ上にスクロールします。
        #[action(deprecated_aliases = ["markdown::MovePageUp"])]
        ScrollPageUp,
        /// Markdown プレビューを 1 ページ下にスクロールします。
        #[action(deprecated_aliases = ["markdown::MovePageDown"])]
        ScrollPageDown,
        /// 約 1 行分上にスクロールします。
        ScrollUp,
        /// 約 1 行分下にスクロールします。
        ScrollDown,
        /// Markdown プレビューを 1 要素分上にスクロールします。
        ScrollUpByItem,
        /// Markdown プレビューを 1 要素分下にスクロールします。
        ScrollDownByItem,
        /// Markdown プレビューの先頭にスクロールします。
        ScrollToTop,
        /// Markdown プレビューの末尾にスクロールします。
        ScrollToBottom,
        /// エディターと同期する追従 Markdown プレビューを開きます。
        OpenFollowingPreview
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        markdown_preview_view::MarkdownPreviewView::register(workspace, window, cx);
    })
    .detach();
}
