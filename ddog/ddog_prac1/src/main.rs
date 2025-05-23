use reqwest::header::HeaderMap;
use serde_json::Value;
use std::env;
use tokio;

const DASHBOARD_URL: &str = "https://api.ddog-gov.com/api/v1/dashboard";
const USAGE_STATS_URL: &str = "https://api.ddog-gov.com/api/v1/usage/hourly_usage";

async fn dashboards(headers: &HeaderMap) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let mut local_headers = HeaderMap::new();
    for (k, v) in headers.iter() {
        local_headers.insert(k, v.clone());
    }

    let response = client
        .get(DASHBOARD_URL)
        .headers(local_headers)
        .send()
        .await?;
    if response.status() != reqwest::StatusCode::OK {
        println!("Error: {}", response.status());
        println!("{:?}", headers);
    } else {
        let result = response.json::<Value>().await?;
        println!("{:?}", result);
    }
    Ok(())
}

async fn usage(headers: &HeaderMap) -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let mut local_headers = HeaderMap::new();
    for (k, v) in headers.iter() {
        local_headers.insert(k, v.clone());
    }

    let response = client
        .get(USAGE_STATS_URL)
        .headers(local_headers)
        .send()
        .await?;
    if response.status() != reqwest::StatusCode::OK {
        println!("Error: {}", response.status());
    } else {
        let result = response.json::<Value>().await?;
        println!("{:?}", result);
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let api_key: &'static str = match env::var("DD_API_KEY") {
        Ok(val) => Box::leak(val.into_boxed_str()),
        Err(_) => panic!("DD_API_KEY is not set"),
    };
    let app_key: &'static str = match env::var("DD_APP_KEY") {
        Ok(val) => Box::leak(val.into_boxed_str()),
        Err(_) => panic!("DD_APP_KEY is not set"),
    };

    let mut fs_headers = HeaderMap::new();
    fs_headers.insert("Accept", "application/json".parse().unwrap());
    fs_headers.insert("DD-API-KEY", api_key.parse().unwrap());
    fs_headers.insert("DD_APPLICATION_KEY", app_key.parse().unwrap());

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("No argument was provided");
        return;
    }
    let resource = &args[1];
    match resource.as_str() {
        "dashboard" => {
            if let Err(e) = dashboards(&fs_headers).await {
                eprintln!("dashboard api call failed: {}", e)
            }
        }
        "usage" => {
            if let Err(e) = usage(&fs_headers).await {
                eprintln!("usage api call failed: {}", e)
            }
        }
        _ => println!("Invalid resource provided: {}", resource),
    }
}
/*
use std::collections::HashMap;
    let mut fs_headers = vec![
        ("Accept", "application/json"),
        ("DD-API-KEY", api_key),
        ("DD_APPLICATION_KEY", app_key),
    ]
    .into_iter()
    .collect::<HashMap<&str, &str>>();
*/
