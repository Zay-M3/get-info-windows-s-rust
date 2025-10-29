use serde::Serialize;
use reqwest::Client;
use reqwest::header::CONTENT_TYPE;
#[derive(Serialize, Debug)]
pub struct Info {
    pub id: u64,
    pub name: String,
    pub active: bool,
    pub winput: String,
}

/// The function `send_info` sends a POST request with JSON data to a specified URL asynchronously in
/// Rust.
/// 
/// Arguments:
/// 
/// * `url`: The `url` parameter is a reference to a string that represents the URL to which the
/// information will be sent.
/// * `info`: The `info` parameter in the `send_info` function is a reference to a struct or data type
/// named `Info`. This struct likely contains information that needs to be sent as JSON in the request
/// body to the specified URL.
/// 
/// Returns:
/// 
/// The `send_info` function returns a `Result` containing either a `reqwest::StatusCode` if the request
/// is successful, or a boxed trait object that implements the `std::error::Error`, `Send`, and `Sync`
/// traits if an error occurs during the request.
/// 
pub async fn send_info(url: &str, info: &Info) -> Result<reqwest::StatusCode, Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();
    let body = serde_json::to_vec(info)?;
    let resp = client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .body(body)
        .send()
        .await?
        .error_for_status()?; 

    Ok(resp.status())
}
