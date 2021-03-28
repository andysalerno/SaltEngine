use crate::messages::{FromClient, FromServer};
use crate::websocket_server::SharedContext;
use crate::{connection::Connection, Result};
use futures::{join, try_join};
use salt_engine::{
    cards::*,
    game_logic::GameEvent,
    game_runner::{GameRunnerHandler, GameRunnerZ},
    game_state::{Deck, GameState, GameStateView, MakePlayerView, PlayerId, UnitCardInstance},
};

struct PlayerGameRunner {
    player_id: PlayerId,
    connection: Connection,
}

impl PlayerGameRunner {
    fn new(player_id: PlayerId, connection: Connection) -> Self {
        Self {
            player_id,
            connection,
        }
    }
}

impl GameRunnerHandler for PlayerGameRunner {
    fn on_turn_start(&self, game_state: &GameState) {
        todo!()
        // let player_turn_connection = {
        //     if whose_turn == game_state.player_a_id() {
        //         player_a_connection
        //     } else if whose_turn == game_state.player_b_id() {
        //         player_b_connection
        //     } else {
        //         panic!("Unknown player ID")
        //     }
        // };

        // // Send the TurnStart message.
        // player_turn_connection
        //     .send(FromServer::TurnStart)
        //     .await
        //     .expect("Sending TurnStart failed.");
    }

    fn next_action(&self) -> GameEvent {
        todo!()
    }
}

pub(crate) async fn play_game(
    mut player_a_connection: Connection,
    player_a_id: PlayerId,
    mut player_b_connection: Connection,
    player_b_id: PlayerId,
    context: SharedContext,
) -> Result<()> {
    // Expect a Ready from both players
    {
        let task_a = player_a_connection.recv::<FromClient>();
        let task_b = player_b_connection.recv::<FromClient>();

        let (resp_a, resp_b) = join!(task_a, task_b);

        match resp_a {
            Some(FromClient::Ready) => println!("Received Ready message from player a."),
            _ => panic!("Expected Ready from client"),
        }

        match resp_b {
            Some(FromClient::Ready) => println!("Received Ready message from player b."),
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
        player_a_connection
            .send(FromServer::State(game_state.player_view(player_a_id)))
            .await?;

        player_b_connection
            .send(FromServer::State(game_state.player_view(player_b_id)))
            .await?;
    }

    let player_a_runner = Box::new(PlayerGameRunner::new(player_a_id, player_a_connection));
    let player_b_runner = Box::new(PlayerGameRunner::new(player_b_id, player_b_connection));
    let runner = GameRunnerZ::new(player_a_runner, player_b_runner, game_state);
    runner.run_game().await;

    //game_loop(game_state, player_a_connection, player_b_connection).await;

    println!(
        "Game with player {:?} and player {:?} has ended.",
        player_a_id, player_b_id
    );
    Ok(())
}

async fn game_loop(
    mut game_state: GameState,
    mut player_a_connection: Connection,
    mut player_b_connection: Connection,
) {
    loop {
        if game_state.is_game_over() {
            return;
        }

        let whose_turn = game_state.cur_player_turn();
        player_take_turn(
            &mut game_state,
            whose_turn,
            &mut player_a_connection,
            &mut player_b_connection,
        )
        .await;
    }
}

async fn player_take_turn(
    game_state: &mut GameState,
    whose_turn: PlayerId,
    player_a_connection: &mut Connection,
    player_b_connection: &mut Connection,
) {
    println!("Player {:?} starts their turn.", whose_turn);

    let player_turn_connection = {
        if whose_turn == game_state.player_a_id() {
            player_a_connection
        } else if whose_turn == game_state.player_b_id() {
            player_b_connection
        } else {
            panic!("Unknown player ID")
        }
    };

    // Send the TurnStart message.
    player_turn_connection
        .send(FromServer::TurnStart)
        .await
        .expect("Sending TurnStart failed.");
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
