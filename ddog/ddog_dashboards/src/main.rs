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
use std::{collections::HashMap, str::FromStr, vec};
use tokio;

async fn dashboards<'a>(headers: HashMap<&'a str, &'a str>) -> Result<(), Error> {
    let url = "https://api.ddog-gov.com/api/v1/dashboard";
    let client = reqwest::Client::new();
    let mut req_headers = HeaderMap::new();
    for (k, v) in headers {
        let headername = HeaderName::from_str(k).expect("invalid header name");
        let headervalue = HeaderValue::from_str(v).expect("invalid header value");
        req_headers.insert(headername, headervalue);
    }

    let response = client.get(url).headers(req_headers).send().await?;
    if response.status() != StatusCode::OK {
        eprintln!("Error: {}", response.status());
    } else {
        let result = response.json::<Value>().await?;
        //dashboards[]|.title
        let mut df = DataFrame::new(vec![]);
        if let Some(dashboards) = result.get("dashboards").and_then(|d| d.as_array()) {
            for entry in dashboards {
                if let Some(title) = entry.get("title") {}
            }
        }
    }
    Ok(())
}
/*
        let mut df = DataFrame::new(vec![]); // Define DataFrame outside the loop
        if let Some(dashboards) = result.get("dashboards").and_then(|d| d.as_array()) {
            for entry in dashboards {
                if let Some(title) = entry.get("title") {
                    df.add_column("FS DASHBOARDS", Series::new("title", vec![title.to_string()]));
                }
            }
            println!("{:?}", &df);
*/

#[tokio::main]
async fn main() {
    let api_key: &str = match std::env::var("DD_API_KEY") {
        Ok(val) => Box::leak(val.into_boxed_str()),
        Err(_) => panic!("DD_API_KEY is not set"),
    };

    let app_key: &str = match std::env::var("DD_APP_KEY") {
        Ok(val) => Box::leak(val.into_boxed_str()),
        Err(_) => panic!("DD_APP_KEY is not set"),
    };

    let headers: HashMap<&str, &str> = vec![
        ("Accept", "application/json"),
        ("DD-API-KEY", api_key),
        ("DD-APPLICATION-KEY", app_key),
    ]
    .into_iter()
    .collect();

    match dashboards(headers).await {
        Err(e) => eprintln!("{}", e),
        _ => {}
    }
}
