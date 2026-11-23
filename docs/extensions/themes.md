# テーマ

拡張機能の `themes` フォルダーには、1つ以上のテーマファイルを置きます。

各テーマファイルは、[`https://zed.dev/schema/themes/v0.2.0.json`](https://zed.dev/schema/themes/v0.2.0.json) で定義された JSON スキーマに従う必要があります。

テーマの作成について詳しくは、[この Zed のブログ記事](https://zed.dev/blog/user-themes-now-in-preview)を参照してください。

## テーマの JSON 構造

Gram のテーマの構造は、[Zed Theme JSON Schema](https://zed.dev/schema/themes/v0.2.0.json) で定義されています。

Gram のテーマは、次の要素を含む Theme Family オブジェクトで構成されます:

- `name`: テーマファミリーの名前
- `author`: テーマファミリーの作者名
- `themes`: テーマファミリーに属する Theme オブジェクトの配列

Theme オブジェクトの主要な構成要素は次のとおりです:

1. テーマのメタデータ:

   - `name`: テーマの名前
   - `appearance`: "light" または "dark"

2. `style` 以下のスタイルプロパティ。たとえば:

   - `background`: メインの背景色
   - `foreground`: メインのテキスト色
   - `accent`: 強調やハイライトに使うアクセント色

3. シンタックスハイライト:

   - `syntax`: 各種の構文要素（キーワード、文字列、コメントなど）の色定義を含むオブジェクト

4. UI 要素:

   - 各種 UI コンポーネントの色。たとえば:
     - `element.background`: UI 要素の背景色
     - `border`: 状態ごと（通常、フォーカス、選択）の枠線の色
     - `text`: 状態ごと（通常、ミュート、アクセント）のテキスト色

5. エディター固有の色:

   - エディター関連要素の色。たとえば:
     - `editor.background`: エディターの背景色
     - `editor.gutter`: ガターの色
     - `editor.line_number`: 行番号の色

6. ターミナルの色:
   - 統合ターミナル用の ANSI 色定義
