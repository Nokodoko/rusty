use serde::Serialize;

#[derive(Debug, Serialize)]
struct Event {
    time: String,
    username: String,
    ip_address: String,
    action: String,
    data: Date,
}

#[derive(Debug, Serialize)]
struct Date {
    from: String,
    to: String,
}

#[derive(Debug, Serialize)]
struct ResponseReport {
    status: String,
    next: String,
    data: Event,
}

#[derive(Debug)]
struct ReportRequest {
    cid: String,
    provhash: String,
    data: Date,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()>{
    color_eyre::install()?;


    Ok(())
}
