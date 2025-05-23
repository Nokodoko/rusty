
#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let client = reqwest::get("http://httpbin.org:80").await?;

    let resp = client.text().await?;
    println!("{}", resp); 


    Ok(())
}
