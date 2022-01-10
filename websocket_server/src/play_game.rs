use crate::network_game_client::NetworkGameClient;
use crate::websocket_server::SharedContext;
use crate::{connection::Connection, Result};
use cards::*;
use futures::{join, try_join};
use log::info;
use protocol::entities::PlayerId;
use protocol::from_client::FromClient;
use protocol::from_server::FromServer;
use salt_engine::{
    cards::UnitCardDefinition,
    game_runner::GameRunner,
    game_state::{Deck, GameState, MakePlayerView, UnitCardInstance},
};

/// Plays a game to completion.
/// Requires a `Connection` for each player,
/// each player's `PlayerId`, and a `SharedContext`.
pub(crate) async fn play_game(
    player_a_connection: Connection,
    player_a_id: PlayerId,
    player_b_connection: Connection,
    player_b_id: PlayerId,
    _context: SharedContext,
) -> Result<()> {
    // Expect a Ready from both players
    {
        let task_a = player_a_connection.recv::<FromClient>();
        let task_b = player_b_connection.recv::<FromClient>();

        let (resp_a, resp_b) = join!(task_a, task_b);

        match resp_a {
            Some(FromClient::Ready) => info!("Received Ready message from player a."),
            _ => panic!("Expected Ready from client"),
        }

        match resp_b {
            Some(FromClient::Ready) => info!("Received Ready message from player b."),
            _ => panic!("Expected Ready from client"),
        }
    }

    // Send GameStart { opponent_id } to both players
    {
        let task_a = player_a_connection.send(FromServer::GameStart {
            opponent_id: player_b_id,
        });

        let task_b = player_b_connection.send(FromServer::GameStart {
            opponent_id: player_a_id,
        });

        try_join!(task_a, task_b)?;
    }

    let player_a_deck = get_deck();
    let player_b_deck = get_deck();

    let game_state =
        GameState::initial_state(player_a_id, player_a_deck, player_b_id, player_b_deck);

    // Send the initial gamestate to both players
    {
        // player_a_connection
        //     .send(FromServer::State(game_state.player_view(player_a_id)))
        //     .await?;

        // player_b_connection
        //     .send(FromServer::State(game_state.player_view(player_b_id)))
        //     .await?;
    }

    let player_a_runner = Box::new(NetworkGameClient::new(player_a_id, player_a_connection));
    let player_b_runner = Box::new(NetworkGameClient::new(player_b_id, player_b_connection));
    let runner = GameRunner::new(player_a_runner, player_b_runner, game_state);
    runner.run_game().await;

    info!(
        "[play_game] Game with player {:?} and player {:?} has ended.",
        player_a_id, player_b_id
    );
    Ok(())
}

fn get_deck() -> Deck {
    let mut deck = {
        let cards: Vec<UnitCardInstance> = (0..8)
            .flat_map(|_| {
                let cards = vec![
                    RicketyCannon.make_instance(),
                    Pawn.make_instance(),
                    EmotionalSupportDog.make_instance(),
                    ReallyBigRock.make_instance(),
                    AttackDog.make_instance(),
                    SleepingDog.make_instance(),
                    PopcornVendor.make_instance(),
                    PriestOfTheLowland.make_instance(),
                    FraidyCat.make_instance(),
                    OutdoorCat.make_instance(),
                    IndoorCat.make_instance(),
                ];

                cards
            })
            .collect();

        Deck::new(cards)
    };

    deck.shuffle();

    deck
}
