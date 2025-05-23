use core::panic;
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::StatusCode;
use reqwest::{header::HeaderMap, Client, Error};
use serde_json::{self, Value};
use std::collections::HashMap;
use std::fmt::Result;
use std::io;
use std::process::Stdio;
use std::result::Result as StdResult;
use std::str::FromStr;
use tokio;

async fn fzf(list: Vec<String>) -> io::Result<String> {
    let fzf: &str = "fzf";
    let fzf_process = std::process::Command::new(&fzf)
        .stdin(Stdio::from(list))
        .stdout(Stdio::piped())
        .spawn()?;
    if let Some(mut stdin) = fzf_process.stdin.take() {
        for item in list {
            writeln!(stdin, "{}", item)?;
        }
    }
    let output = fzf_process.wait_with_output()?;
    if output.status.success() {
        let output_str = String::from_utf8(output.stdout)
            .unwrap_or_else(|_| String::from("").trim().to_string());
        Ok(output_str)
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "failed to capture fzf output",
        ))
    }
}

fn role_name_json_parser(result: &Value) -> Result<String, Box<dyn std::error::Error>> {
    let mut role_names = Vec::new();
    if let Some(data) = result.get("data").and_then(|d| d.as_array()) {
        for entry in data {
            if let Some(name) = entry
                .get("attributes")
                .and_then(|n| n.get("name"))
                .and_then(|n| n.as_str())
            {
                //println!("{}", name.to_string());
                role_names.push(name.to_string())
            }
        }
    }
    if role_names.is_empty() {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "no role names found".into(),
        )))
    } else {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(fzf(role_names))
            .map_err(|e| e.into())
    }
}

async fn role_names<'a>(headers: HashMap<&'a str, &'a str>) -> Result<(), Error> {
    let url: &str = "https://api.ddog-gov.com/api/v2/roles";
    let client = Client::new();
    let mut req_headers = HeaderMap::new();
    for (k, v) in headers {
        let headername = HeaderName::from_str(k).expect("invalid header name");
        let headrvalue = HeaderValue::from_str(v).expect("invalide header value");
        req_headers.insert(headername, headrvalue);
    }

    let response = client.get(url).headers(req_headers).send().await?;
    match response.status() {
        StatusCode::OK => {
            let result = response.json::<Value>().await?;
            println!("{:?}", role_name_json_parser(&result));
        }
        StatusCode::UNAUTHORIZED => {
            eprintln!("Unauthorized api call, ensure you have the correct credentials");
            return Err(Error::from(response.error_for_status().unwrap_err()));
        }
        _ => {
            eprintln!("Error with api call: {}", response.status());
            return Err(reqwest::Error::from(
                response.error_for_status().unwrap_err(),
            ));
        }
    };
    Ok(())
}

#[tokio::main]
async fn main() {
    let api_key: &'static str = match std::env::var("DD_API_KEY") {
        Ok(val) => Box::leak(val.into_boxed_str()),
        Err(e) => panic!("DD_API_KEY is not set: {}", e),
    };

    let app_key: &'static str = match std::env::var("DD_APP_KEY") {
        Ok(val) => Box::leak(val.into_boxed_str()),
        Err(e) => panic!("DD_APP_KEY is not set: {}", e),
    };

    let dd_headers: HashMap<&str, &str> = vec![
        ("Accept", "application/json"),
        ("DD-API-KEY", api_key),
        ("DD-APPLICATION-KEY", app_key),
    ]
    .into_iter()
    .collect();

    match role_names(dd_headers).await {
        Err(e) => panic!("Error running role_names api: {}", e),
        _ => {}
    }
}
