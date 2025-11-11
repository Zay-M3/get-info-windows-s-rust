use reqwest::Client;
use reqwest::header::CONTENT_TYPE;
use std::sync::RwLock;
use colored::*;

use crate::utils::interfase::Info;

// Global endpoint that can be changed at runtime
static ENDPOINT: RwLock<String> = RwLock::new(String::new());

/// Initialize the endpoint with the default value
pub fn init_endpoint() {
    let mut endpoint = ENDPOINT.write().unwrap();
    if endpoint.is_empty() {
        *endpoint = "http://127.0.0.1:8000/api/info".to_string();
    }
}

/// Get the current endpoint URL
pub fn get_endpoint() -> String {
    init_endpoint(); // Ensure it's initialized
    ENDPOINT.read().unwrap().clone()
}

/// Change the endpoint URL globally
/// 
/// Arguments:
/// 
/// * `new_url`: The new endpoint URL to use for API requests
/// 
/// Returns:
/// 
/// Returns Ok(()) if successful, or an error if the URL is invalid
pub fn change_endpoint(new_url: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Validate URL format (basic validation)
    if !new_url.starts_with("http://") && !new_url.starts_with("https://") {
        return Err("URL must start with http:// or https://".into());
    }
    
    let mut endpoint = ENDPOINT.write().unwrap();
    *endpoint = new_url.to_string();
    
    println!("{} Endpoint changed to: {}", "✓".bright_green().bold(), new_url.bright_cyan());
    Ok(())
}



/// The function `send_info` sends a POST request with JSON data to a specified URL asynchronously in
/// Rust.
/// 
/// Arguments:
/// 
/// * `info`: The `info` parameter is a reference to a struct or data type
/// named `Info`. This struct likely contains information that needs to be sent as JSON in the request
/// body to the current endpoint URL.
/// 
/// Returns:
/// 
/// The `send_info` function returns a `Result` containing either a `reqwest::StatusCode` if the request
/// is successful, or a boxed trait object that implements the `std::error::Error`, `Send`, and `Sync`
/// traits if an error occurs during the request.
/// 
pub async fn send_info(info: &Info) -> Result<reqwest::StatusCode, Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();
    let body = serde_json::to_vec(info)?;
    let endpoint_url = get_endpoint(); // Get current endpoint
    
    println!("{} Sending request to: {}", "→".bright_blue(), endpoint_url.bright_cyan());
    
    let resp = client
        .post(&endpoint_url)
        .header(CONTENT_TYPE, "application/json")
        .body(body)
        .send()
        .await?
        .error_for_status()?;  

    Ok(resp.status())

}