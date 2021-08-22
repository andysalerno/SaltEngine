use crate::connection::Connection;
use crate::messages::{FromClient, FromServer};
use crate::network_client_notifier::NetworkClientNotifier;
use crate::network_prompter::NewtorkPrompter;
use async_trait::async_trait;
use log::info;
use salt_engine::game_agent::ClientNotifier;
use salt_engine::{
    game_agent::Prompter,
    game_logic::ClientActionEvent,
    game_runner::GameClient,
    game_state::{GameState, GameStatePlayerView, PlayerId},
};

pub(crate) struct NetworkGameClient {
    player_id: PlayerId,
    connection: Connection,
}

impl NetworkGameClient {
    pub(crate) fn new(player_id: PlayerId, connection: Connection) -> Self {
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

    async fn next_action(&mut self, game_state_view: GameStatePlayerView) -> ClientActionEvent {
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

    async fn make_notifier(&self) -> Box<dyn ClientNotifier> {
        Box::new(NetworkClientNotifier::new(self.connection.clone()))
    }
}
