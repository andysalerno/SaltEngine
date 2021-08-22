use crate::Result;
use crate::{connection::Connection, matchmaker::MatchMaker};
use crate::{messages::FromServer, play_game::play_game};
use log::info;
use salt_engine::game_state::PlayerId;
use smol::net::TcpListener;
use std::sync::Arc;

const SERVER_ADDRESS: &str = "localhost:9000";
pub type SharedContext = Arc<GlobalContext>;

#[derive(Debug, Default)]
pub struct GlobalContext {
    pub matchmaker: MatchMaker,
}

async fn queue_player_and_play(connection: Connection, context: SharedContext) -> Result<()> {
    let (player_a_connection, player_b_connection) =
        match context.matchmaker.match_player(connection).await {
            Some((a, b)) => (a, b),
            _ => return Ok(()),
        };

    let player_a_id = PlayerId::new();
    player_a_connection
        .send(FromServer::Hello(player_a_id))
        .await?;

    let player_b_id = PlayerId::new();
    player_b_connection
        .send(FromServer::Hello(player_b_id))
        .await?;

    play_game(
        player_a_connection,
        player_a_id,
        player_b_connection,
        player_b_id,
        context,
    )
    .await?;

    Ok(())
}

pub fn run() -> Result<()> {
    smol::block_on(async {
        info!("Listening on {}", SERVER_ADDRESS);
        let listener = TcpListener::bind(SERVER_ADDRESS).await?;

        let context: SharedContext = Arc::new(GlobalContext::default());

        while let Ok((stream, addr)) = listener.accept().await {
            info!("New connection from {}", addr);

            let ws_stream = async_tungstenite::accept_async(stream).await?;
            let connection = Connection::new(ws_stream);

            let context = context.clone();

            smol::spawn(async move {
                let result = queue_player_and_play(connection, context).await;

                match result {
                    Err(e) => eprintln!("{}", e),
                    _ => {}
                }
            })
            .detach();
        }

        Ok(())
    })
}
