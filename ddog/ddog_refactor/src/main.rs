use reqwest::{self, StatusCode};
use std::collections::HashMap;
use tokio;

const DASHBOARD_URL: &str = "https://api.ddog-gov.com/api/v1/dashboard";

async fn dashboard(headers: &Hashmap<&str, &str>) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let mut local_headers = HashMap::new();
    for (k, v) in headers {
        local_headers.insert(k.to_string(), v.to_string())
    }

    let response = client
        .get(DASHBOARD_URL)
        .headers(local_headers)
        .send()
        .await?;

    if response.status() != StatusCode::OK {
        println!("Error: {}", response.status());
    }else {
        let result = response.json()::<Value>().await?;
        println!("{:?}", result);
    }
    Ok(())
}

fn main() {
    let api_key: &'static str = match std::env::var("DD_API_KEY") {
        Ok(val) => Box::leak(val.into_boxed_str()),
        Err(_) => panic!("DD_API_KEY is not set")
    };
}
