use crate::messages::{FromClient, FromServer};
use crate::{connection::Connection, Result};
use crate::{network_prompter::NewtorkPrompter, websocket_server::SharedContext};
use async_trait::async_trait;
use cards::*;
use futures::{join, try_join};
use log::info;
use salt_engine::{
    cards::UnitCardDefinition,
    game_agent::game_agent::Prompter,
    game_logic::ClientGameEvent,
    game_runner::{GameClient, GameRunner},
    game_state::{
        Deck, GameState, GameStatePlayerView, MakePlayerView, PlayerId, UnitCardInstance,
    },
};

struct NetworkGameClient {
    player_id: PlayerId,
    connection: Connection,
}

impl NetworkGameClient {
    fn new(player_id: PlayerId, connection: Connection) -> Self {
        Self {
            player_id,
            connection,
        }
    }
}

#[async_trait]
impl GameClient for NetworkGameClient {
    async fn on_turn_start(&mut self, _game_state: &GameState) {
        info!("Player controller: on turn start");

        self.connection
            .send(FromServer::TurnStart)
            .await
            .expect("failed to send turnstart");
    }

    async fn next_action(&mut self, game_state_view: GameStatePlayerView) -> ClientGameEvent {
        // Awaiting response from the client.

        let _ping = self
            .connection
            .send(FromServer::WaitingForAction(game_state_view))
            .await;
        info!("Waiting for the player's next action...");
        let from_client = self
            .connection
            .recv::<FromClient>()
            .await
            .expect("no response from the client.");
        info!("Action received from player.");

        match from_client {
            FromClient::ClientAction(e) => e,
            _ => panic!("Unexpected response from client; expected ClientGameEvent"),
        }
    }

    async fn make_prompter(&self) -> Box<dyn Prompter> {
        Box::new(NewtorkPrompter::new(self.connection.clone()))
    }

    async fn observe_state_update(&mut self, game_state_view: GameStatePlayerView) {
        self.connection
            .send(FromServer::State(game_state_view))
            .await
            .expect("Failed to send state update");
    }
}

pub(crate) async fn play_game(
    player_a_connection: Connection,
    player_a_id: PlayerId,
    player_b_connection: Connection,
    player_b_id: PlayerId,
    context: SharedContext,
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
        player_a_connection
            .send(FromServer::State(game_state.player_view(player_a_id)))
            .await?;

        player_b_connection
            .send(FromServer::State(game_state.player_view(player_b_id)))
            .await?;
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
