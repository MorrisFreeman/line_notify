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
    let line_notify = LineNotify::new(token, Some(message.to_string()), None);
    match line_notify.send().await {
        Ok(_) => println!("Notification sent"),
        Err(e) => println!("Error: {}", e),
    }
}
