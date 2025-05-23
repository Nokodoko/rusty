fn main() -> color_eyre::Result<()>{
    color_eyre::install()?;

    let client = reqwest::get("http://localhost:80");

    let resp = client.text();
    println!("{}", resp); 

    Ok(())
}
