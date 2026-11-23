# タブスイッチャー

タブスイッチャーは、Gram で開いているタブ間を素早く移動する手段を提供します。開いているタブが最近の使用順に一覧表示されるため、直前に作業していた場所へすぐに戻れます。

## クイック切り替え

コマンドパレットから {#action tab_switcher::Toggle} を実行するのではなく、{#kb tab_switcher::Toggle} でタブスイッチャーを開いた場合、<kbd class="keybinding">ctrl</kbd> キーを押し続けている間はスイッチャーがアクティブなままになります。

<kbd class="keybinding">ctrl</kbd> を押し続けたまま <kbd class="keybinding">tab</kbd> を押すたびに次の項目へ移動し（<kbd class="keybinding">shift</kbd> で逆方向へ移動）、<kbd class="keybinding">ctrl</kbd> を離すと選択中の項目が確定してスイッチャーが閉じます。

## タブスイッチャーを開く

タブスイッチャーは {#action tab_switcher::Toggle} または {#action tab_switcher::ToggleAll} でも開けます。{#kb tab_switcher::Toggle} では現在のペインのタブのみが表示され、{#kb tab_switcher::ToggleAll} ではすべてのペインのタブが表示されます。

タブスイッチャーを開いている間は、次の操作が可能です:

- {#kb menu::SelectNext} を押して一覧の次のタブへ移動
- {#kb menu::SelectPrevious} を押して前のタブへ移動
- <kbd class="keybinding">enter</kbd> を押して選択中のタブを確定し、スイッチャーを閉じる
- <kbd class="keybinding">escape</kbd> を押してスイッチャーを閉じ、開く前の元のタブへ戻る
- {#kb tab_switcher::CloseSelectedItem} を押して現在選択中のタブを閉じる

一覧を移動すると、Gram はペインのアクティブな項目を選択中のタブに合わせて更新します。

## アクション一覧

| アクション                                | 説明                                               |
| ----------------------------------------- | -------------------------------------------------- |
| {#action tab_switcher::Toggle}            | 現在のペインのタブスイッチャーを開く               |
| {#action tab_switcher::ToggleAll}         | すべてのペインのタブを表示するタブスイッチャーを開く |
| {#action tab_switcher::CloseSelectedItem} | タブスイッチャーで選択中のタブを閉じる             |
