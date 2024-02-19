# LineNotify

LineNotifyは、Rustを使用してLINE Notify APIを通じて通知を送信するためのシンプルなインターフェースを提供します。このクレートを使用することで、テキストメッセージと画像の両方をLINE Notifyを通じて送信できます。Rustの強力な非同期機能を活用します。

## 使い方
まず、LINE Notifyのウェブサイトに従ってLINE Notifyのアクセストークンを取得します。

次に、プロジェクトで以下のようにクレートを使用します：
```rs
use line_notify::LineNotify;
use tokio;

#[tokio::main]
async fn main() {
    let notifier = LineNotify::new(
        "YOUR_ACCESS_TOKEN".to_string(),
        Some("こんにちは、世界！".to_string()),
        None // または画像を送信する場合はSome("path/to/your/image.jpg".to_string())
    );

    match notifier.send().await {
        Ok(_) => println!("通知が正常に送信されました。"),
        Err(e) => println!("通知の送信に失敗しました: {}", e),
    }
}
```

## 貢献
貢献は大歓迎です！お気軽にプルリクエストを送ってください。

## ライセンス
このクレートはMITライセンスの下で公開されています。

## 免責事項
このクレートはLINE Corporationと公式には関連していません。
