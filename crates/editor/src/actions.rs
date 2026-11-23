//! This module contains all actions supported by [`Editor`].
use super::*;
use gpui::{Action, actions};
use project::project_settings::GoToDiagnosticSeverityFilter;
use schemars::JsonSchema;
use util::serde::default_true;

/// 現在の選択範囲の次の一致箇所を選択します。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct SelectNext {
    #[serde(default)]
    pub replace_newest: bool,
}

/// 現在の選択範囲の前の一致箇所を選択します。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct SelectPrevious {
    #[serde(default)]
    pub replace_newest: bool,
}

/// カーソルを現在の行の先頭に移動します。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct MoveToBeginningOfLine {
    #[serde(default = "default_true")]
    pub stop_at_soft_wraps: bool,
    #[serde(default)]
    pub stop_at_indent: bool,
}

/// カーソルから現在の行の先頭まで選択します。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct SelectToBeginningOfLine {
    #[serde(default)]
    pub(super) stop_at_soft_wraps: bool,
    #[serde(default)]
    pub stop_at_indent: bool,
}

/// カーソルから現在の行の先頭まで削除します。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct DeleteToBeginningOfLine {
    #[serde(default)]
    pub(super) stop_at_indent: bool,
}

/// カーソルを1ページ上に移動します。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct MovePageUp {
    #[serde(default)]
    pub(super) center_cursor: bool,
}

/// カーソルを1ページ下に移動します。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct MovePageDown {
    #[serde(default)]
    pub(super) center_cursor: bool,
}

/// カーソルを現在の行の末尾に移動します。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct MoveToEndOfLine {
    #[serde(default = "default_true")]
    pub stop_at_soft_wraps: bool,
}

/// カーソルから現在の行の末尾まで選択します。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct SelectToEndOfLine {
    #[serde(default)]
    pub(super) stop_at_soft_wraps: bool,
}

/// カーソル位置で利用可能なコードアクションの表示を切り替えます。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct ToggleCodeActions {
    // Source from which the action was deployed.
    #[serde(default)]
    #[serde(skip)]
    pub deployed_from: Option<CodeActionSource>,
    // Run first available task if there is only one.
    #[serde(default)]
    #[serde(skip)]
    pub quick_launch: bool,
}

#[derive(PartialEq, Clone, Debug)]
pub enum CodeActionSource {
    Indicator(DisplayRow),
    RunMenu(DisplayRow),
    QuickActionBar,
}

/// 現在選択されている補完候補を確定して挿入します。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct ConfirmCompletion {
    #[serde(default)]
    pub item_ix: Option<usize>,
}

/// 複数の補完候補を1つの補完に合成します。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct ComposeCompletion {
    #[serde(default)]
    pub item_ix: Option<usize>,
}

/// 現在選択されているコードアクションを確定して適用します。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct ConfirmCodeAction {
    #[serde(default)]
    pub item_ix: Option<usize>,
}

/// 選択行のコメント記号を切り替えます。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct ToggleComments {
    #[serde(default)]
    pub advance_downwards: bool,
    #[serde(default)]
    pub ignore_indent: bool,
}

/// カーソルを指定行数上に移動します。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct MoveUpByLines {
    #[serde(default)]
    pub(super) lines: u32,
}

/// カーソルを指定行数下に移動します。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct MoveDownByLines {
    #[serde(default)]
    pub(super) lines: u32,
}

/// 選択範囲を指定行数上に拡張します。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct SelectUpByLines {
    #[serde(default)]
    pub(super) lines: u32,
}

/// 選択範囲を指定行数下に拡張します。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct SelectDownByLines {
    #[serde(default)]
    pub(super) lines: u32,
}

/// エディター内のすべての抜粋を展開します。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct ExpandExcerpts {
    #[serde(default)]
    pub(super) lines: u32,
}

/// 現在位置より上の抜粋を展開します。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct ExpandExcerptsUp {
    #[serde(default)]
    pub(super) lines: u32,
}

/// 現在位置より下の抜粋を展開します。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct ExpandExcerptsDown {
    #[serde(default)]
    pub(super) lines: u32,
}

/// エディターでのテキスト入力を処理します。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
pub struct HandleInput(pub String);

/// カーソルから次の単語の末尾まで削除します。
/// 長さ2以上の空白シーケンスがある場合は、その手前で停止します。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct DeleteToNextWordEnd {
    #[serde(default)]
    pub ignore_newlines: bool,
    // Whether to stop before the end of the next word, if language-defined bracket is encountered.
    #[serde(default)]
    pub ignore_brackets: bool,
}

/// カーソルから前の単語の先頭まで削除します。
/// 長さ2以上の空白シーケンスがある場合は、その手前で停止します。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct DeleteToPreviousWordStart {
    #[serde(default)]
    pub ignore_newlines: bool,
    // Whether to stop before the start of the previous word, if language-defined bracket is encountered.
    #[serde(default)]
    pub ignore_brackets: bool,
}

/// カーソルから次のサブワードの末尾まで削除します。
/// 長さ2以上の空白シーケンスがある場合は、その手前で停止します。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct DeleteToNextSubwordEnd {
    #[serde(default)]
    pub ignore_newlines: bool,
    // Whether to stop before the start of the previous word, if language-defined bracket is encountered.
    #[serde(default)]
    pub ignore_brackets: bool,
}

/// カーソルから前のサブワードの先頭まで削除します。
/// 長さ2以上の空白シーケンスがある場合は、その手前で停止します。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct DeleteToPreviousSubwordStart {
    #[serde(default)]
    pub ignore_newlines: bool,
    // Whether to stop before the start of the previous word, if language-defined bracket is encountered.
    #[serde(default)]
    pub ignore_brackets: bool,
}

/// カーソルから行末まで切り取ります。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct CutToEndOfLine {
    #[serde(default)]
    pub stop_at_newlines: bool,
}

/// 指定されたインデントレベルのすべてのコードブロックを折りたたみます。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
pub struct FoldAtLevel(pub u32);

/// 現在のカーソル位置から最も近い利用可能なタスクを生成して実行します。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct SpawnNearestTask {
    #[serde(default)]
    pub reveal: task::RevealStrategy,
}

#[derive(Clone, PartialEq, Action)]
#[action(no_json, no_register)]
pub struct DiffClipboardWithSelectionData {
    pub clipboard_text: String,
    pub editor: Entity<Editor>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Default)]
pub enum UuidVersion {
    #[default]
    V4,
    V7,
}

/// 選択範囲を行ごとの選択に分割します。
#[derive(PartialEq, Clone, Deserialize, Default, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct SplitSelectionIntoLines {
    /// 分割後、カーソルに縮小せずテキストを選択状態のまま維持します。
    #[serde(default)]
    pub keep_selections: bool,
}

/// ファイル内の次の診断へ移動します。
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct GoToDiagnostic {
    #[serde(default)]
    pub severity: GoToDiagnosticSeverityFilter,
}

/// ファイル内の前の診断へ移動します。
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct GoToPreviousDiagnostic {
    #[serde(default)]
    pub severity: GoToDiagnosticSeverityFilter,
}

/// 現在の選択範囲の上にカーソルを追加します。
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct AddSelectionAbove {
    #[serde(default = "default_true")]
    pub skip_soft_wrap: bool,
}

/// 現在の選択範囲の下にカーソルを追加します。
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct AddSelectionBelow {
    #[serde(default = "default_true")]
    pub skip_soft_wrap: bool,
}

/// カーソル位置にスニペットを挿入します。
#[derive(PartialEq, Clone, Default, Debug, Deserialize, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct InsertSnippet {
    /// Language name if using a named snippet, or `None` for a global snippet
    ///
    /// This is typically lowercase and matches the filename containing the snippet, without the `.json` extension.
    pub language: Option<String>,
    /// Name if using a named snippet
    pub name: Option<String>,

    /// Snippet body, if not using a named snippet
    // todo(andrew): use `ListOrDirect` or similar for multiline snippet body
    pub snippet: Option<String>,
}

actions!(
    debugger,
    [
        /// 現在のカーソル位置までプログラムを実行します。
        RunToCursor,
        /// デバッガーコンテキストで選択テキストを評価します。
        EvaluateSelectedText
    ]
);

actions!(
    go_to_line,
    [
        /// 指定行へ移動ダイアログの表示を切り替えます。
        #[action(name = "Toggle")]
        ToggleGoToLine
    ]
);

actions!(
    editor,
    [
        /// エディター内のすべての差分ハンクを適用します。
        ApplyAllDiffHunks,
        /// 現在位置の差分ハンクを適用します。
        ApplyDiffHunk,
        /// カーソルの前の文字を削除します。
        Backspace,
        /// 現在の行のGit Blame情報を表示します。
        BlameHover,
        /// 現在の操作をキャンセルします。
        Cancel,
        /// 実行中のflycheck操作をキャンセルします。
        CancelFlycheck,
        /// 保留中の言語サーバーの処理をキャンセルします。
        CancelLanguageServerWork,
        /// flycheckの結果を消去します。
        ClearFlycheck,
        /// 名前変更操作を確定します。
        ConfirmRename,
        /// カーソル位置に挿入して補完を確定します。
        ConfirmCompletionInsert,
        /// 既存のテキストを置換して補完を確定します。
        ConfirmCompletionReplace,
        /// コンテキストメニューの最初の項目に移動します。
        ContextMenuFirst,
        /// コンテキストメニューの最後の項目に移動します。
        ContextMenuLast,
        /// コンテキストメニューの次の項目に移動します。
        ContextMenuNext,
        /// コンテキストメニューの前の項目に移動します。
        ContextMenuPrevious,
        /// インデントをタブからスペースに変換します。
        ConvertIndentationToSpaces,
        /// インデントをスペースからタブに変換します。
        ConvertIndentationToTabs,
        /// 選択したテキストをkebab-caseに変換します。
        ConvertToKebabCase,
        /// 選択したテキストをlowerCamelCaseに変換します。
        ConvertToLowerCamelCase,
        /// 選択したテキストを小文字に変換します。
        ConvertToLowerCase,
        /// 選択したテキストの大文字・小文字を反転します。
        ConvertToOppositeCase,
        /// 選択したテキストをセンテンスケースに変換します。
        ConvertToSentenceCase,
        /// 選択したテキストをsnake_caseに変換します。
        ConvertToSnakeCase,
        /// 選択したテキストをタイトルケースに変換します。
        ConvertToTitleCase,
        /// 選択したテキストをUpperCamelCaseに変換します。
        ConvertToUpperCamelCase,
        /// 選択したテキストを大文字に変換します。
        ConvertToUpperCase,
        /// 選択したテキストにROT13暗号を適用します。
        ConvertToRot13,
        /// 選択したテキストにROT47暗号を適用します。
        ConvertToRot47,
        /// 選択したテキストをクリップボードにコピーします。
        Copy,
        /// 前後の空白をトリムして選択テキストをクリップボードにコピーします。
        CopyAndTrim,
        /// 現在のファイル位置をクリップボードにコピーします。
        CopyFileLocation,
        /// ハイライトされたテキストをJSONとしてコピーします。
        CopyHighlightJson,
        /// 現在のファイル名をクリップボードにコピーします。
        CopyFileName,
        /// 拡張子を除いたファイル名をクリップボードにコピーします。
        CopyFileNameWithoutExtension,
        /// 現在の行へのパーマリンクをコピーします。
        CopyPermalinkToLine,
        /// 選択したテキストをクリップボードに切り取ります。
        Cut,
        /// カーソルの後の文字を削除します。
        Delete,
        /// 現在の行を削除します。
        DeleteLine,
        /// カーソルから行末まで削除します。
        DeleteToEndOfLine,
        /// クリップボードのテキストと現在の選択範囲の差分を表示します。
        DiffClipboardWithSelection,
        /// アクティブなすべてのカーソルの名前を表示します。
        DisplayCursorNames,
        /// 現在の行を下に行ごと複製します。
        DuplicateLineDown,
        /// 現在の行を上に行ごと複製します。
        DuplicateLineUp,
        /// 現在の選択範囲を複製します。
        DuplicateSelection,
        /// エディター内のすべての差分ハンクを展開します。
        #[action(deprecated_aliases = ["editor::ExpandAllHunkDiffs"])]
        ExpandAllDiffHunks,
        /// エディター内のすべての差分ハンクを折りたたみます。
        CollapseAllDiffHunks,
        /// カーソル位置のマクロを再帰的に展開します。
        ExpandMacroRecursively,
        /// 検索で次の一致箇所を検索します。
        FindNextMatch,
        /// 検索で前の一致箇所を検索します。
        FindPreviousMatch,
        /// 現在のコードブロックを折りたたみます。
        Fold,
        /// エディター内の折りたたみ可能なすべての領域を折りたたみます。
        FoldAll,
        /// インデントレベル1のすべてのコードブロックを折りたたみます。
        #[action(name = "FoldAtLevel_1")]
        FoldAtLevel1,
        /// インデントレベル2のすべてのコードブロックを折りたたみます。
        #[action(name = "FoldAtLevel_2")]
        FoldAtLevel2,
        /// インデントレベル3のすべてのコードブロックを折りたたみます。
        #[action(name = "FoldAtLevel_3")]
        FoldAtLevel3,
        /// インデントレベル4のすべてのコードブロックを折りたたみます。
        #[action(name = "FoldAtLevel_4")]
        FoldAtLevel4,
        /// インデントレベル5のすべてのコードブロックを折りたたみます。
        #[action(name = "FoldAtLevel_5")]
        FoldAtLevel5,
        /// インデントレベル6のすべてのコードブロックを折りたたみます。
        #[action(name = "FoldAtLevel_6")]
        FoldAtLevel6,
        /// インデントレベル7のすべてのコードブロックを折りたたみます。
        #[action(name = "FoldAtLevel_7")]
        FoldAtLevel7,
        /// インデントレベル8のすべてのコードブロックを折りたたみます。
        #[action(name = "FoldAtLevel_8")]
        FoldAtLevel8,
        /// インデントレベル9のすべてのコードブロックを折りたたみます。
        #[action(name = "FoldAtLevel_9")]
        FoldAtLevel9,
        /// エディター内のすべての関数本体を折りたたみます。
        FoldFunctionBodies,
        /// 現在のコードブロックとその子をすべて再帰的に折りたたみます。
        FoldRecursive,
        /// 選択された範囲を折りたたみます。
        FoldSelectedRanges,
        /// 最後にアクティブだったバッファーへフォーカスを戻します。
        ToggleFocus,
        /// 現在位置の折りたたみを切り替えます。
        ToggleFold,
        /// 現在位置の再帰的な折りたたみを切り替えます。
        ToggleFoldRecursive,
        /// バッファー内のすべての折りたたみまたはマルチバッファーのすべての抜粋を切り替えます。
        ToggleFoldAll,
        /// ドキュメント全体をフォーマットします。
        Format,
        /// 選択したテキストのみをフォーマットします。
        FormatSelections,
        /// カーソル位置のシンボルの宣言へ移動します。
        GoToDeclaration,
        /// 分割ペインで宣言へ移動します。
        GoToDeclarationSplit,
        /// カーソル位置のシンボルの定義へ移動します。
        GoToDefinition,
        /// 分割ペインで定義へ移動します。
        GoToDefinitionSplit,
        /// 次の差分ハンクへ移動します。
        GoToHunk,
        /// 前の差分ハンクへ移動します。
        GoToPreviousHunk,
        /// カーソル位置のシンボルの実装へ移動します。
        GoToImplementation,
        /// 分割ペインで実装へ移動します。
        GoToImplementationSplit,
        /// ファイル内の次の変更へ移動します。
        GoToNextChange,
        /// 現在のファイルの親モジュールへ移動します。
        GoToParentModule,
        /// ファイル内の前の変更へ移動します。
        GoToPreviousChange,
        /// カーソル下のシンボルの次の参照へ移動します。
        GoToNextReference,
        /// カーソル下のシンボルの前の参照へ移動します。
        GoToPreviousReference,
        /// カーソル位置のシンボルの型定義へ移動します。
        GoToTypeDefinition,
        /// 分割ペインで型定義へ移動します。
        GoToTypeDefinitionSplit,
        /// 次のドキュメントハイライトへ移動します。
        GoToNextDocumentHighlight,
        /// 前のドキュメントハイライトへ移動します。
        GoToPreviousDocumentHighlight,
        /// 半ページ下にスクロールします。
        HalfPageDown,
        /// 半ページ上にスクロールします。
        HalfPageUp,
        /// カーソル位置のシンボルのホバー情報を表示します。
        Hover,
        /// 選択行のインデントを増やします。
        Indent,
        /// カーソル位置にUUID v4を挿入します。
        InsertUuidV4,
        /// カーソル位置にUUID v7を挿入します。
        InsertUuidV7,
        /// 現在の行と次の行を結合します。
        JoinLines,
        /// キルリングにコピーします（Emacsスタイル）。
        KillRingCopy,
        /// 行末までをキルリングに切り取ります（Emacsスタイル）。
        KillLine,
        /// キルリングに切り取ります（Emacsスタイル）。
        KillRingCut,
        /// キルリングからヤンク（貼り付け）します（Emacsスタイル）。
        KillRingYank,
        /// カーソルを1行下に移動します。
        LineDown,
        /// カーソルを1行上に移動します。
        LineUp,
        /// カーソルを下に移動します。
        MoveDown,
        /// カーソルを左に移動します。
        MoveLeft,
        /// 現在の行を下に移動します。
        MoveLineDown,
        /// 現在の行を上に移動します。
        MoveLineUp,
        /// カーソルを右に移動します。
        MoveRight,
        /// カーソルをドキュメントの先頭に移動します。
        MoveToBeginning,
        /// カーソルを外側の対応する括弧に移動します。
        MoveToEnclosingBracket,
        /// 最も近い外側の区切り文字（括弧、中括弧、丸括弧、引用符など）の内側を選択します。
        SelectInsideDelimiters,
        /// 最も近い外側の区切り文字を自身を含めて選択します。
        SelectAroundDelimiters,
        /// カーソルをドキュメントの末尾に移動します。
        MoveToEnd,
        /// カーソルを段落の末尾に移動します。
        MoveToEndOfParagraph,
        /// カーソルを次のサブワードの末尾に移動します。
        MoveToNextSubwordEnd,
        /// カーソルを次の単語の末尾に移動します。
        MoveToNextWordEnd,
        /// カーソルを前のサブワードの先頭に移動します。
        MoveToPreviousSubwordStart,
        /// カーソルを前の単語の先頭に移動します。
        MoveToPreviousWordStart,
        /// カーソルを段落の先頭に移動します。
        MoveToStartOfParagraph,
        /// カーソルを現在の抜粋の先頭に移動します。
        MoveToStartOfExcerpt,
        /// カーソルを次の抜粋の先頭に移動します。
        MoveToStartOfNextExcerpt,
        /// カーソルを現在の抜粋の末尾に移動します。
        MoveToEndOfExcerpt,
        /// カーソルを前の抜粋の末尾に移動します。
        MoveToEndOfPreviousExcerpt,
        /// カーソルを1つ外側の構文ノードの先頭に移動します。
        MoveToStartOfLargerSyntaxNode,
        /// カーソルを1つ外側の構文ノードの末尾に移動します。
        MoveToEndOfLargerSyntaxNode,
        /// カーソルを上に移動します。
        MoveUp,
        /// 改行を挿入してカーソルを移動します。
        Newline,
        /// 現在の行の上に新しい行を挿入します。
        NewlineAbove,
        /// 現在の行の下に新しい行を挿入します。
        NewlineBelow,
        /// 次の画面へスクロールします。
        NextScreen,
        /// スニペットの次のタブストップがあれば移動します。
        NextSnippetTabstop,
        /// カーソル位置でコンテキストメニューを開きます。
        OpenContextMenu,
        /// 現在のファイルから抜粋を開きます。
        OpenExcerpts,
        /// 分割ペインで抜粋を開きます。
        OpenExcerptsSplit,
        /// 変更提案エディターを開きます。
        OpenProposedChangesEditor,
        /// カーソル位置のシンボルのドキュメントを開きます。
        OpenDocs,
        /// 現在の行へのパーマリンクを開きます。
        OpenPermalinkToLine,
        /// エディター内で名前が選択されているファイルを開きます。
        #[action(deprecated_aliases = ["editor::OpenFile"])]
        OpenSelectedFilename,
        /// すべての選択範囲をマルチバッファーで開きます。
        OpenSelectionsInMultibuffer,
        /// カーソル位置のURLを開きます。
        OpenUrl,
        /// import文を整理します。
        OrganizeImports,
        /// 選択行のインデントを減らします。
        Outdent,
        /// コンテキストに基づいて自動的にインデントを調整します。
        AutoIndent,
        /// 1ページ下にスクロールします。
        PageDown,
        /// 1ページ上にスクロールします。
        PageUp,
        /// クリップボードから貼り付けます。
        Paste,
        /// スニペットの前のタブストップがあれば移動します。
        PreviousSnippetTabstop,
        /// 元に戻した最後の編集をやり直します。
        Redo,
        /// 最後の選択範囲の変更をやり直します。
        RedoSelection,
        /// カーソル位置のシンボルの名前を変更します。
        Rename,
        /// 現在のファイルの言語サーバーを再起動します。
        RestartLanguageServer,
        /// 現在のファイルをシステムファイルマネージャーで表示します。
        RevealInFileManager,
        /// 選択行の順序を反転します。
        ReverseLines,
        /// ファイルをディスクから再読み込みします。
        ReloadFile,
        /// 推奨行長に収まるようにテキストを折り返します。
        Rewrap,
        /// 選択範囲または行を後方に回転します。
        RotateSelectionsBackward,
        /// 選択範囲または行を前方に回転します。
        RotateSelectionsForward,
        /// flycheck診断を実行します。
        RunFlycheck,
        /// カーソルが表示領域の下端にくるようスクロールします。
        ScrollCursorBottom,
        /// カーソルが表示領域の中央にくるようスクロールします。
        ScrollCursorCenter,
        /// カーソル位置を表示領域の中央、上端、下端の間で順に切り替えます。
        ScrollCursorCenterTopBottom,
        /// カーソルが表示領域の上端にくるようスクロールします。
        ScrollCursorTop,
        /// エディター内のすべてのテキストを選択します。
        SelectAll,
        /// 現在の選択範囲のすべての一致箇所を選択します。
        SelectAllMatches,
        /// 現在の抜粋の先頭まで選択します。
        SelectToStartOfExcerpt,
        /// 次の抜粋の先頭まで選択します。
        SelectToStartOfNextExcerpt,
        /// 現在の抜粋の末尾まで選択します。
        SelectToEndOfExcerpt,
        /// 前の抜粋の末尾まで選択します。
        SelectToEndOfPreviousExcerpt,
        /// 選択範囲を下に拡張します。
        SelectDown,
        /// 外側のシンボルを選択します。
        SelectEnclosingSymbol,
        /// 1つ外側の構文ノードを選択します。
        SelectLargerSyntaxNode,
        /// 次の兄弟構文ノードを選択します。
        SelectNextSyntaxNode,
        /// 前の兄弟構文ノードを選択します。
        SelectPreviousSyntaxNode,
        /// 選択範囲を左に拡張します。
        SelectLeft,
        /// 現在の行を選択します。
        SelectLine,
        /// 選択範囲を1ページ下に拡張します。
        SelectPageDown,
        /// 選択範囲を1ページ上に拡張します。
        SelectPageUp,
        /// 選択範囲を右に拡張します。
        SelectRight,
        /// 1つ内側の構文ノードを選択します。
        SelectSmallerSyntaxNode,
        /// ドキュメントの先頭まで選択します。
        SelectToBeginning,
        /// ドキュメントの末尾まで選択します。
        SelectToEnd,
        /// 段落の末尾まで選択します。
        SelectToEndOfParagraph,
        /// 次のサブワードの末尾まで選択します。
        SelectToNextSubwordEnd,
        /// 次の単語の末尾まで選択します。
        SelectToNextWordEnd,
        /// 前のサブワードの先頭まで選択します。
        SelectToPreviousSubwordStart,
        /// 前の単語の先頭まで選択します。
        SelectToPreviousWordStart,
        /// 段落の先頭まで選択します。
        SelectToStartOfParagraph,
        /// 選択範囲を上に拡張します。
        SelectUp,
        /// カーソル位置でコード補完候補を表示します。
        ShowCompletions,
        /// システム文字パレットを表示します。
        ShowCharacterPalette,
        /// 現在の関数のシグネチャヘルプを表示します。
        ShowSignatureHelp,
        /// 単語補完を表示します。
        ShowWordCompletions,
        /// 選択行をランダムにシャッフルします。
        ShuffleLines,
        /// シグネチャヘルプポップアップ内の次のシグネチャに移動します。
        SignatureHelpNext,
        /// シグネチャヘルプポップアップ内の前のシグネチャに移動します。
        SignatureHelpPrevious,
        /// 選択行を長さ順に並べ替えます。
        SortLinesByLength,
        /// 選択行を大文字・小文字を区別せずに並べ替えます。
        SortLinesCaseInsensitive,
        /// 選択行を大文字・小文字を区別して並べ替えます。
        SortLinesCaseSensitive,
        /// 現在のファイルの言語サーバーを停止します。
        StopLanguageServer,
        /// コンテキストに応じて補完を表示、タブ文字を挿入、またはインデントします。
        SuperTab,
        /// ソースファイルとヘッダーファイルを切り替えます。
        SwitchSourceHeader,
        /// タブ文字を挿入またはインデントします。
        Tab,
        /// タブ文字を削除またはインデントを減らします。
        Backtab,
        /// 現在の行のブレークポイントを切り替えます。
        ToggleBreakpoint,
        /// 選択したテキストの大文字・小文字を反転します。
        ToggleCase,
        /// 現在の行のブレークポイントを無効化します。
        DisableBreakpoint,
        /// 現在の行のブレークポイントを有効化します。
        EnableBreakpoint,
        /// ブレークポイントのログメッセージを編集します。
        EditLogBreakpoint,
        /// シグネチャヘルプの自動表示を切り替えます。
        ToggleAutoSignatureHelp,
        /// インラインGit Blameの表示を切り替えます。
        ToggleGitBlameInline,
        /// カーソル位置のBlameコミットを開きます。
        OpenGitBlameCommit,
        /// 診断パネルの表示を切り替えます。
        ToggleDiagnostics,
        /// インデントガイドの表示を切り替えます。
        ToggleIndentGuides,
        /// インレイヒントの表示を切り替えます。
        ToggleInlayHints,
        /// インライン値の表示を切り替えます。
        ToggleInlineValues,
        /// インライン診断の表示を切り替えます。
        ToggleInlineDiagnostics,
        /// 行番号の表示を切り替えます。
        ToggleLineNumbers,
        /// ミニマップの表示を切り替えます。
        ToggleMinimap,
        /// 現在の選択範囲の始点と終点を入れ替えます。
        SwapSelectionEnds,
        /// 現在位置にマークを設定します。
        SetMark,
        /// 相対行番号の表示を切り替えます。
        ToggleRelativeLineNumbers,
        /// 選択したハンクの差分表示を切り替えます。
        #[action(deprecated_aliases = ["editor::ToggleHunkDiff"])]
        ToggleSelectedDiffHunks,
        /// 選択メニューの表示を切り替えます。
        ToggleSelectionMenu,
        /// 折り返し（ソフトラップ）モードを切り替えます。
        ToggleSoftWrap,
        /// 分割差分ビューを切り替えます。
        ToggleSplitDiff,
        /// タブバーの表示を切り替えます。
        ToggleTabBar,
        /// カーソルの前後の文字を入れ替えます。
        Transpose,
        /// 最後の編集を元に戻します。
        Undo,
        /// 最後の選択範囲の変更を元に戻します。
        UndoSelection,
        /// 折りたたまれたすべての領域を展開します。
        UnfoldAll,
        /// カーソル位置の行を展開します。
        UnfoldLines,
        /// カーソル位置を再帰的に展開します。
        UnfoldRecursive,
        /// 重複行を削除します（大文字・小文字を区別しない）。
        UniqueLinesCaseInsensitive,
        /// 重複行を削除します（大文字・小文字を区別する）。
        UniqueLinesCaseSensitive,
        /// 現在の選択範囲から外側の構文ノード（括弧やクロージャなど）を取り除きます。
        UnwrapSyntaxNode,
        /// 選択範囲を言語で指定されたタグで囲みます。
        WrapSelectionsInTag,
        /// バッファーから末尾の空白をすべて削除します。
        RemoveTrailingWhitespace,
    ]
);

/// カーソル位置のシンボルのすべての参照を検索します。
#[derive(PartialEq, Clone, Deserialize, JsonSchema, Action)]
#[action(namespace = editor)]
#[serde(deny_unknown_fields)]
pub struct FindAllReferences {
    #[serde(default = "default_true")]
    pub always_open_multibuffer: bool,
}

impl Default for FindAllReferences {
    fn default() -> Self {
        Self {
            always_open_multibuffer: true,
        }
    }
}
