/* use serde::{Deserialize, Serialize};
use surf::Client;

#[derive(Serialize, Deserialize)]
struct Response {
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();
    let url = "https://rpc.testnet.near.org";
    // let jsonrpc = "2.0";
    let id = "dev-1668546365877-28764765342385";
    let method = "query";
    let params = r#"{"request_type": "call_function", "finality": "final", "account_id": "dev-1668546365877-28764765342385", "method_name": "nft_metadata", "args_base64": "e30="}"#;


    let request_body = serde_json::json!({
        "method": method,
        "params": params,
        "id": id,
    });

    let response = client
        .post(url)
        .body_json(&request_body)?
        .recv_json::<Response>()
        .await?;

    println!("{}", response.message);

    Ok(())
} */

use std::error::Error;
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
struct RpcRequest {
    jsonrpc: String,
    method: String,
    params: String,
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RpcResponse {
    result: Option<String>,
}

async fn call_rpc() -> Result<(), Box<dyn Error>> {
    let url = "https://rpc.testnet.near.org";
    let params =
        r#"{"request_type": "call_function", "finality": "final", "account_id": "dev-1668546365877-28764765342385", "method_name": "nft_metadata", "args_base64": "e30="}"#;
    let request = RpcRequest {
        jsonrpc: "2.0".to_owned(),
        method: "query".to_owned(),
        params: params.to_string(),
        id: "dev-1668546365877-28764765342385".to_owned(),
    };
    let mut res = surf::post(url).body(surf::Body::from_json(&request)?).await?;
    let response: RpcResponse = res.body_json().await?;
    println!("RPC response: {:?}", response);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    call_rpc().await?;
    Ok(())
}