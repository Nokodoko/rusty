use crate::reqwest::Error;
use core::panic;
use reqwest::{
    self,
    header::{HeaderMap, HeaderName, HeaderValue},
    Client, StatusCode,
};
use serde_json::Value;
use std::{collections::HashMap, str::FromStr};
use tokio;

async fn assist_ls<'a>(heads: HashMap<&'a str, &'a str>) -> Result<(), Error> {
    let url: &str = "https://api.openai.com/v1/assistants?order=desc&limit=20";
    let mut assist_headers = HeaderMap::new();
    for (k, v) in heads {
        let ass_head = HeaderName::from_str(k).expect("Invalid headers");
        let ass_values = HeaderValue::from_str(v).expect("Invalid headers");
        assist_headers.insert(ass_head, ass_values);
    }

    let client = Client::new();
    let response = client.get(url).headers(assist_headers).send().await?;
    match response.status() {
        StatusCode::OK => {
            let result = response.json::<Value>().await;
            print!("{:?}", result)
        }
        StatusCode::BAD_REQUEST => {
            eprintln!("{}", StatusCode::BAD_REQUEST)
        }
        StatusCode::BAD_GATEWAY => {
            eprintln!("{}", StatusCode::BAD_GATEWAY)
        }
        StatusCode::UNAUTHORIZED => {
            eprintln!("{}", StatusCode::UNAUTHORIZED)
        }
        _ => {
            eprintln!("Unexpected Error: {}", response.status())
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let api_key: &str = match std::env::var("OPENAI_API_KEY") {
        Ok(val) => Box::leak(val.into_boxed_str()),
        Err(e) => panic!("Api key not set: {}", e),
    };

    let authorization: &str = &format!("Bearer {}", api_key);
    let headers: HashMap<&str, &str> = vec![
        ("Content-Type", "application/json"),
        ("Authorization", authorization),
        ("OpenAI-Beta", "assistants=v1"),
    ]
    .into_iter()
    .collect();

    match assist_ls(headers).await {
        Err(e) => panic!("Could not execute api call: {}", e),
        _ => {}
    }
}

fn something(name: str) Result<(), Error>{
    let path: &str = format!("Something: {}", name)
}
