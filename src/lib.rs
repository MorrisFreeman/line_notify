/// The LineNotify crate provides a simple interface for sending notifications through the LINE Notify API in Rust.
///
/// With this crate, you can send text messages and images via LINE Notify. A LINE Notify access token is required.
use reqwest::{multipart::{Form, Part}, Response};
use tokio;

/// `MyResult` is a type alias representing the outcome of the main operations in this crate.
/// It contains a `reqwest::Response` on success or a `Box<dyn std::error::Error>` on failure.
pub type MyResult = Result<Response, Box<dyn std::error::Error>>;

/// The `LineNotify` struct holds the configuration necessary for sending notifications to LINE Notify.
pub struct LineNotify {
    token: String,
    message: Option<String>,
    file_path: Option<String>,
}

impl LineNotify {
    /// The `new` function creates a new instance of `LineNotify` with the specified settings for sending notifications to LINE Notify.
    ///
    /// # Arguments
    ///
    /// * `token` - The access token for LINE Notify.
    /// * `message` - The message to be sent. If `None`, no message is sent.
    /// * `file_path` - The path to the image file to be sent. If `None`, no file is sent.
    ///
    /// # Example
    ///
    /// ```
    /// let notifier = LineNotify::new("YOUR_ACCESS_TOKEN".to_string(), Some("Hello, world!".to_string()), None);
    /// ```
    pub fn new(token: String, message: Option<String>, file_path: Option<String>) -> Self {
        LineNotify {
            token,
            message,
            file_path,
        }
    }

    /// The `send` asynchronous function sends the configured message and/or image file via LINE Notify.
    ///
    /// # Returns
    ///
    /// This function returns `MyResult`. On successful transmission, it returns `Ok(Response)`. On failure, it returns `Err(e)`.
    ///
    /// # Example
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let notifier = LineNotify::new("YOUR_ACCESS_TOKEN".to_string(), Some("Hello, world!".to_string()), None);
    ///     match notifier.send().await {
    ///         Ok(_) => println!("Notification sent successfully."),
    ///         Err(e) => println!("Failed to send notification: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn send(&self) -> MyResult {
        if self.message.is_none() && self.file_path.is_none() {
            return Err("message or file_path is required".into());
        }

        let api_url = "https://notify-api.line.me/api/notify";
        let mut form = Form::new();
        if let Some(message) = &self.message {
            form = form.text("message", message.to_string());
        }

        if let Some(file_path) = &self.file_path {
            let content: Vec<u8> = tokio::fs::read(file_path).await?;
            let part = Part::bytes(content).file_name(file_path.clone());
            form = form.part("imageFile", part);
        }

        let response = reqwest::Client::new()
            .post(api_url)
            .bearer_auth(&self.token)
            .multipart(form)
            .send()
            .await?;

        Ok(response)
    }
}
