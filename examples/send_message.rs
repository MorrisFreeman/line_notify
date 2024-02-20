use std::env;
use line_notify::LineNotify;
use tokio;

#[tokio::main]
async fn main() {
    let token = match env::var("LINE_NOTIFY_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            println!("LINE_NOTIFY_TOKEN is not set");
            return;
        }
    };
    let message = "Hello, World!";
    let line_notify = LineNotify::new(&token);
    match line_notify.set_message(message).send().await {
        Ok(response) => {
            println!("Status: {}", response.status());
            println!("Headers:\n{:#?}", response.headers());
            println!("Body: {}", response.text().await.unwrap());
        },
        Err(e) => println!("Error: {}", e),
    }
}
