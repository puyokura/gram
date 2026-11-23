use heck::ToTitleCase;

pub(crate) fn localize_enum_label(label: &str, should_do_title_case: bool) -> String {
    let localized = match label {
        // Common boolean / toggle / frequency / mode labels
        "Off" | "off" => Some("オフ"),
        "On" | "on" => Some("オン"),
        "Enabled" | "enabled" => Some("有効"),
        "Disabled" | "disabled" => Some("無効"),
        "Always" | "always" => Some("常に"),
        "Never" | "never" => Some("しない"),
        "Auto" | "auto" => Some("自動"),
        "Default" | "default" => Some("既定"),
        "None" | "none" => Some("なし"),
        "All" | "all" => Some("すべて"),
        "System" | "system" => Some("システム"),
        "Custom" | "custom" => Some("カスタム"),

        // Theme / Appearance
        "Light" | "light" => Some("ライト"),
        "Dark" | "dark" => Some("ダーク"),
        "Static" | "static" => Some("固定"),
        "Dynamic" | "dynamic" => Some("動的"),
        "Comfortable" | "comfortable" => Some("ゆったり"),
        "Standard" | "standard" => Some("標準"),
        "Tight" | "tight" => Some("コンパクト"),

        // Sizes / Positions / Directions
        "XSmall" | "x_small" => Some("極小"),
        "Small" | "small" => Some("小"),
        "Medium" | "medium" => Some("中"),
        "Large" | "large" => Some("大"),
        "XLarge" | "x_large" => Some("特大"),
        "Full" | "full" => Some("全幅"),
        "Left" | "left" => Some("左"),
        "Right" | "right" => Some("右"),
        "Up" | "up" => Some("上"),
        "Down" | "down" => Some("下"),
        "Top" | "top" => Some("上部"),
        "Bottom" | "bottom" => Some("下部"),

        // Dock / Window / Workspace
        "Contained" | "contained" => Some("枠内（中央）"),
        "LeftAligned" | "left_aligned" => Some("左揃え"),
        "RightAligned" | "right_aligned" => Some("右揃え"),
        "Client" | "client" => Some("クライアント側"),
        "Server" | "server" => Some("サーバー側"),
        "OnlyLeft" | "only_left" => Some("左のみ"),
        "Both" | "both" => Some("両方"),
        "PlatformDefault" | "platform_default" => Some("OS の既定"),
        "CloseWindow" | "close_window" => Some("ウィンドウを閉じる"),
        "KeepWindowOpen" | "keep_window_open" => Some("ウィンドウを開いたままにする"),
        "QuitApp" | "quit_app" => Some("アプリを終了"),
        "EmptyTab" | "empty_tab" => Some("空のタブ"),
        "LastWorkspace" | "last_workspace" => Some("前回のワークスペース"),
        "LastSession" | "last_session" => Some("前回のセッション"),
        "Launchpad" | "launchpad" => Some("ランチャー"),
        "OnPaneFocus" | "on_pane_focus" => Some("ペインにフォーカス時"),
        "Hover" | "hover" => Some("ホバー時"),
        "Hidden" | "hidden" => Some("非表示"),
        "NonUtf8" | "non_utf8" => Some("UTF-8 以外のみ"),
        "AfterDelay" | "after_delay" => Some("遅延後に自動保存"),
        "History" | "history" => Some("履歴順"),
        "Neighbour" | "neighbour" => Some("隣のタブ"),
        "LeftNeighbour" | "left_neighbour" => Some("左隣のタブ"),
        "Subpixel" | "subpixel" => Some("サブピクセル"),
        "Grayscale" | "grayscale" => Some("グレースケール"),

        // Project Panel / File Finder / Image Viewer
        "DirectoriesFirst" | "Directories first" | "directories_first" => Some("フォルダー優先"),
        "SmartSort" | "Smart sort" | "smart_sort" => Some("スマート並び替え"),
        "Mixed" | "mixed" => Some("混在"),
        "FilesFirst" | "Files first" | "files_first" => Some("ファイル優先"),
        "Icon" | "icon" => Some("アイコン"),
        "LabelColor" | "label_color" => Some("ラベル色"),
        "Indexed" | "indexed" => Some("インデックス済みのみ"),
        "Smart" | "smart" => Some("スマート"),
        "Binary" | "binary" => Some("2進接頭辞 (KiB)"),
        "Decimal" | "decimal" => Some("10進接頭辞 (KB)"),

        // Debugger
        "Statement" | "statement" => Some("ステートメント単位"),
        "Line" | "line" => Some("行"),
        "Instruction" | "instruction" => Some("命令単位"),

        // Editor
        "Symbol" | "symbol" => Some("シンボル"),
        "Wrapped" | "wrapped" => Some("折り返し行を含める"),
        "Inlay" | "inlay" => Some("インレイ"),
        "Border" | "border" => Some("枠線"),
        "Background" | "background" => Some("背景"),
        "Tab" | "tab" => Some("タブ"),
        "Indent" | "indent" => Some("インデント"),
        "Gutter" | "gutter" => Some("ガター"),
        "Selection" | "selection" => Some("選択範囲のみ"),
        "Select" | "select" => Some("選択"),
        "Open" | "open" => Some("開く"),
        "LeftOpen" | "left_open" => Some("左辺なし"),
        "RightOpen" | "right_open" => Some("右辺なし"),
        "LeftOnly" | "left_only" => Some("左辺のみ"),
        "Information" | "information" | "Info" | "info" => Some("情報"),
        "Warning" | "warning" => Some("警告"),
        "Error" | "error" => Some("エラー"),
        "Errors" | "errors" => Some("エラーのみ"),
        "Hint" | "hint" => Some("ヒント"),
        "Alt" | "alt" => Some("Alt"),
        "CmdOrCtrl" | "cmd_or_ctrl" => Some("Cmd / Ctrl"),
        "OnePage" | "one_page" => Some("1ページ分"),
        "VerticalScrollMargin" | "vertical_scroll_margin" => Some("垂直スクロール余白分"),
        "Bar" | "bar" => Some("縦棒"),
        "Block" | "block" => Some("ブロック"),
        "Underline" | "underline" => Some("下線"),
        "Hollow" | "hollow" => Some("枠線ブロック"),
        "FindAllReferences" | "find_all_references" => Some("すべての参照を検索"),
        "OnTyping" | "on_typing" => Some("入力時"),
        "OnTypingAndMovement" | "on_typing_and_movement" => Some("入力および移動時"),
        "Inline" | "inline" => Some("インライン"),
        "Clipboard" | "clipboard" => Some("クリップボード"),
        "Primary" | "primary" => Some("プライマリ選択"),
        "AllEditors" | "all_editors" => Some("すべてのエディター"),
        "ActiveEditor" | "active_editor" => Some("アクティブなエディターのみ"),

        // Language / Formatting / Whitespace / Indent guides
        "PreferLine" | "prefer_line" => Some("行優先"),
        "EditorWidth" | "editor_width" => Some("エディター幅"),
        "PreferredLineLength" | "preferred_line_length" => Some("推奨行長"),
        "Bounded" | "bounded" => Some("上限付き"),
        "Boundary" | "boundary" => Some("境界のみ"),
        "Trailing" | "trailing" => Some("行末のみ"),
        "InComments" | "in_comments" => Some("コメント内"),
        "InSelections" | "in_selections" => Some("選択範囲内"),
        "Anywhere" | "anywhere" => Some("どこでも"),
        "Insert" | "insert" => Some("挿入"),
        "Replace" | "replace" => Some("置換"),
        "ReplaceSubsequence" | "replace_subsequence" => Some("部分一致を置換"),
        "ReplaceSuffix" | "replace_suffix" => Some("接尾辞を置換"),
        "Fallback" | "fallback" => Some("フォールバック"),
        "Fixed" | "fixed" => Some("固定色"),
        "IndentAware" | "indent_aware" => Some("インデント連動"),

        // Git
        "TrackedFiles" | "tracked_files" => Some("追跡対象ファイルのみ"),
        "Hide" | "hide" => Some("非表示"),
        "StagedHollow" | "staged_hollow" => Some("ステージ済みを枠線表示"),
        "UnstagedHollow" | "unstaged_hollow" => Some("未ステージを枠線表示"),
        "FileNameFirst" | "file_name_first" => Some("ファイル名優先"),
        "FilePathFirst" | "file_path_first" => Some("ファイルパス優先"),

        // Terminal
        "Program" | "program" => Some("プログラムを指定"),
        "WithArguments" | "with_arguments" => Some("引数付きで指定"),
        "CurrentProjectDirectory" | "current_project_directory" => Some("現在のプロジェクトディレクトリ"),
        "FirstProjectDirectory" | "first_project_directory" => Some("最初のプロジェクトディレクトリ"),
        "AlwaysHome" | "always_home" => Some("常にホームディレクトリ"),
        "TerminalControlled" | "terminal_controlled" => Some("ターミナルの制御に従う"),

        // Base Keymaps (keep proper names readable)
        "VSCode" | "VS Code" => Some("VS Code"),
        "JetBrains" => Some("JetBrains"),
        "SublimeText" | "Sublime Text" => Some("Sublime Text"),
        "Atom" => Some("Atom"),
        "TextMate" => Some("TextMate"),
        "Emacs" => Some("Emacs"),
        "Minimal" => Some("最小構成"),

        _ => None,
    };

    if let Some(ja) = localized {
        ja.to_string()
    } else if should_do_title_case {
        label.to_title_case()
    } else {
        label.to_string()
    }
}
