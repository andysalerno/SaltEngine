use server::messages::NewGameResponse;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://localhost:3030/newgame").await?;
    println!("{:?}", resp);

    let resp = resp.json::<NewGameResponse>().await?;
    println!("{:#?}", resp);
    Ok(())
}
