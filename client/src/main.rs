use salt_engine::id::Id;
use server::messages::NewGameResponse;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let game_id = join_game().await?;

    Ok(())
}

async fn join_game() -> Result<Id, Box<dyn std::error::Error>> {
    let resp = reqwest::get("http://localhost:3030/newgame").await?;
    let resp = resp.json::<NewGameResponse>().await?;

    println!("Joining game: {:#?}", resp.game_id);
    Ok(resp.game_id)
}
