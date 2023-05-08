use serde::{Deserialize, Serialize};
use surf::Client;

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();
    let url = "http://localhost:8080"; // replace with your Boringtun API endpoint

    let request_body = serde_json::json!({
        "method": "ping",
        "params": [],
        "id": 1,
    });

    let response = client
        .post(url)
        .body_json(&request_body)?
        .recv_json::<Response>()
        .await?;

    println!("{}", response.message);

    Ok(())
}
