use crate::reqwest::Error;
use polars::frame::DataFrame;
use polars::prelude::NamedFrom;
use polars::series::Series;
use reqwest::{
    self,
    header::{HeaderMap, HeaderName, HeaderValue},
    StatusCode,
};
use serde_json::Value;
use std::{collections::HashMap, str::FromStr};
use tokio;

async fn services<'a>(headers: HashMap<&'a str, &'a str>) -> Result<(), Error> {
    let url: &str = "https://api.ddog-gov.com/api/v2/services/definitions";
    let client = reqwest::Client::new();
    let mut req_headers = HeaderMap::new();
    for (key, value) in headers {
        let headername = HeaderName::from_str(key).expect("no parse headers");
        let headervalue = HeaderValue::from_str(value).expect("no parse values");
        req_headers.insert(headername, headervalue);
    }

    let response = client.get(url).headers(req_headers).send().await?;
    match response.status() {
        StatusCode::OK => {
            let result = response.json::<Value>().await?;
            if let Some(data) = result.get("data").and_then(|d| d.as_array()) {
                for entry in data {
                    if let Some(dd_service) = entry
                        .get("attributes")
                        .and_then(|a| a.get("schema"))
                        .and_then(|s| s.get("dd-service"))
                        .and_then(|ds| ds.as_str())
                    {
                        let df = DataFrame::new(vec![Series::new(
                            "FS APM LiST",
                            vec![dd_service.to_string()],
                        )]);
                        println!("{:?}", df)
                    }
                }
            }
        }
        StatusCode::BAD_REQUEST => {
            eprintln!("{}", StatusCode::BAD_REQUEST)
        }
        _ => {
            eprintln!("Unexpected Error: {}", response.status())
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let api_key: &str = match std::env::var("DD_API_KEY") {
        Ok(var) => Box::leak(var.into_boxed_str()),
        Err(_) => panic!("DD_API_KEY is not set"),
    };

    let app_key: &str = match std::env::var("DD_APP_KEY") {
        Ok(var) => Box::leak(var.into_boxed_str()),
        Err(_) => panic!("DD_APP_KEY is not set"),
    };

    let headers: HashMap<&str, &str> = vec![
        ("Accept", "application/json"),
        ("DD-API-KEY", api_key),
        ("DD-APPLICATION-KEY", app_key),
    ]
    .into_iter()
    .collect();

    match services(headers).await {
        Err(e) => eprintln!("Error: {}", e),
        _ => {}
    }
}
