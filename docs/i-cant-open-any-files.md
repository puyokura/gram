# ヘルプ: ファイルを開くことができない場合

## Linux: リンクをクリックしても機能しない

これらの機能は XDG デスクトップポータルによって提供されています。具体的には以下のとおりです。

- `org.freedesktop.portal.FileChooser`
- `org.freedesktop.portal.OpenURI`

`Hyprland` などの一部のウィンドウマネージャーは、デフォルトではファイルピッカーを提供していません。代替手段の検討にあたっては、[こちらのリスト](https://wiki.archlinux.org/title/XDG_Desktop_Portal#List_of_backends_and_interfaces) を参考にしてください。

## Linux: inotify を開始できなかった場合

Gram はファイルシステムの変更を監視するために inotify を使用しています。inotify を開始できない場合、Gram は正常に動作しません。

「too many open files」というエラーが表示される場合は、まず `sysctl fs.inotify` を確認してください。

- `max_user_instances` が 128 以上になっていることを確認してください（`sudo sysctl fs.inotify.max_user_instances=1024` で上限を変更できます）。Gram に必要な inotify インスタンスは 1 つだけです。
- `max_user_watches` が 8000 以上になっていることを確認してください（`sudo sysctl fs.inotify.max_user_watches=64000` で上限を変更できます）。Gram では、開いているすべてのプロジェクトのディレクトリごとに 1 つ、Git リポジトリごとに 1 つ、さらに設定、テーマ、キーマップ、拡張機能用として若干数の監視（watch）が必要です。

また、ファイルディスクリプタが不足している可能性もあります。上限は `ulimit` で確認でき、`/etc/security/limits.conf` を編集することで更新できます。

## Mac: ドキュメントのリンクをクリックしても機能しない

ソースコードからビルドして実行しており、Gram をアプリケーションとしてまだインストールしていない場合、`gram://` リンクは正常に機能しません。これらを機能させるには、アプリケーションバンドルをご自身でビルドするか、[Gram ウェブサイト](https://gram-editor.com) からビルド済みバンドルをインストールする必要があります。
