use crate::connection::Connection;
use crate::network_client_notifier::NetworkClientNotifier;
use crate::network_prompter::NewtorkPrompter;
use async_trait::async_trait;
use log::info;
use protocol::entities::PlayerId;
use protocol::from_client::{ClientAction, FromClient};
use protocol::from_server::FromServer;
use salt_engine::game_agent::ClientNotifier;
use salt_engine::{game_agent::Prompter, game_runner::GameClient, game_state::GameState};

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

    // async fn next_action(&mut self, game_state_view: GameStatePlayerView) -> ClientAction {
    async fn next_action(&mut self) -> ClientAction {
        // Awaiting response from the client.

        let _ping = self.connection.send(FromServer::WaitingForAction).await;
        info!("Waiting for the player's next action...");
        let from_client = self
            .connection
            .recv::<FromClient>()
            .await
            .expect("no response from the client.");

        let action = match from_client {
            FromClient::ClientAction(e) => e,
            _ => panic!("Unexpected response from client; expected ClientGameEvent"),
        };

        info!("Action received from player: {:?}", action);

        action
    }

    async fn make_prompter(&self) -> Box<dyn Prompter> {
        Box::new(NewtorkPrompter::new(self.connection.clone()))
    }

    async fn make_notifier(&self) -> Box<dyn ClientNotifier> {
        Box::new(NetworkClientNotifier::new(self.connection.clone()))
    }
}
