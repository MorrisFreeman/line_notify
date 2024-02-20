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
    image_thumb: Option<String>,
    image_full: Option<String>,
    image_file: Option<String>,
}

impl LineNotify {
    /// The `new` function creates a new instance of `LineNotify` with the specified settings for sending notifications to LINE Notify.
    ///
    /// # Arguments
    ///
    /// * `token` - The access token for LINE Notify.
    /// * `message` - The message to be sent. If `None`, no message is sent.
    /// * `image_thumb` - The URL of the thumbnail image to be sent. If `None`, no thumbnail image is sent.
    /// * `image_full` - The URL of the full-size image to be sent. If `None`, no full-size image is sent.
    /// * `image_file` - The path to the image file to be sent. If `None`, no image file is sent.
    ///
    /// # Example
    ///
    /// ```
    /// let notifier = LineNotify::new("YOUR_ACCESS_TOKEN".to_string());
    /// ```
    pub fn new(token: String) -> Self {
        LineNotify {
            token,
            message: None,
            image_thumb: None,
            image_full: None,
            image_file: None,

        }
    }

    /// The `set_message` function sets the message to be sent.
    pub fn set_message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }

    /// The `set_image_thumb` function sets the URL of the thumbnail image to be sent.
    pub fn set_image_thumb(mut self, file_path: &str) -> Self {
        self.image_thumb = Some(file_path.to_string());
        self
    }

    /// The `set_image_full` function sets the URL of the full-size image to be sent.
    pub fn set_image_full(mut self, file_path: &str) -> Self {
        self.image_full = Some(file_path.to_string());
        self
    }

    /// The `set_image_file` function sets the path to the image file to be sent.
    pub fn set_image_file(mut self, file_path: &str) -> Self {
        self.image_file = Some(file_path.to_string());
        self
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
    ///     let line_notifier = LineNotify::new(token);
    ///     match line_notifier.set_message(message).send().await {
    ///         Ok(_) => println!("Notification sent"),
    ///         Err(e) => println!("Error: {}", e),
    ///     }
    /// }
    /// ```
    pub async fn send(&self) -> MyResult {
        if self.message.is_none() {
            return Err("No message specified".into());
        }

        if self.image_thumb.is_none() ^ self.image_full.is_none() {
            return Err("Both image thumbnail and full-size URLs must be specified".into());
        }

        let api_url = "https://notify-api.line.me/api/notify";
        let mut form = Form::new();
        if let Some(message) = &self.message {
            form = form.text("message", message.to_string());
        }

        if let Some(url) = &self.image_thumb {
            form = form.text("imageThumbnail", url.to_string());
        }

        if let Some(url) = &self.image_full {
            form = form.text("imageFullsize", url.to_string());
        }

        if let Some(file_path) = &self.image_file {
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
