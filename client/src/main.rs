use core::panic;

use crate::reqwest::Error;
use reqwest::{self, Client, StatusCode};
use serde_json::Value;
use tokio;

async fn local_client() -> Result<(), Error> {
    let url: &str = "http://localhost:9990";
    let client = Client::new();
    let response = client.get(url).send().await?;
    match response.status() {
        StatusCode::OK => {
            let result = response.json::<Value>().await?;
            println!("{}", result)
        }
        StatusCode::BAD_REQUEST => {
            eprintln!("{}", StatusCode::BAD_REQUEST)
        }
        StatusCode::BAD_GATEWAY => {
            eprintln!("{}", StatusCode::BAD_GATEWAY)
        }
        _ => {
            eprintln!("Unexpected error: {}", response.status())
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    match local_client().await {
        Err(e) => panic!("Could not make api request {}", e),
        _ => {}
    };
}
