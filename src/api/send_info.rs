use serde::Serialize;
use reqwest::Client;
use reqwest::header::CONTENT_TYPE;
#[derive(Serialize, Debug)]
pub struct Info {
    pub id: u64,
    pub name: String,
    pub active: bool,
}

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


// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[tokio::test]
//     async fn test_send_info() {
        
//         let url = "https://httpbin.org/post";
//         let info = Info {
//             id: 42,
//             name: "rus".into(),
//             active: true,
//         };

//         let status = send_info(url, &info).await.expect("request failed");
//         println!("Status: {}", status);
//         assert!(status.is_success());
//     }
// }