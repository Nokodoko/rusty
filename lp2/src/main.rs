#![allow(dead_code)]
use dotenv::dotenv;

#[derive(Debug, Serialize)]
struct Response_report {
    status: String,
    next: String,
    data: Event[],
}

#[derive(Debug, Serialize)]
struct Event {
    time: String,
    username: String,
    ip_adderss: String,
    action :String,
    data: Date,
}

#[derive(Default, Serialize)]
struct Date {
    from: String,
    to: String,
}

#[derive(Debug, Serialize)]
struct Report_Request {
    cid: String,
    provhash: String,
    cmd: String,
    data: Date,
}

#[tokio::main]
async fn main() -> color_eyre::Result<()>{
    color_eyre::install()?;

    let answer = Response::default();
    let ask = Report_Request{
        cid: String::from("12133541"),
        provhash: String::from(key),
        cmd: String::from("reporting"),
        data: Date{
            from: String::from(),
            to: String::from()
        },
    }

    let client = reqwest::client::new();
    let req = client.get("https://lastpass.com/enterprise.php")
        .send(ask).await?
        println!(req.json(answer));

    Ok(())
}
