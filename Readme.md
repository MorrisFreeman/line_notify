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
    let token = "YOUR_ACCESS_TOKEN";
    let message = "Hello, World!";
    let line_notify = LineNotify::new(token);
    match line_notify.set_message(message).send().await {
        Ok(response) => {
            println!("Status: {}", response.status());
            println!("Headers:\n{:#?}", response.headers());
            println!("Body: {}", response.text().await.unwrap());
        },
        Err(e) => println!("Error: {}", e),
    }
}

```

## 貢献
貢献は大歓迎です！お気軽にプルリクエストを送ってください。

## ライセンス
このクレートはMITライセンスの下で公開されています。

## 免責事項
このクレートはLINE Corporationと公式には関連していません。
