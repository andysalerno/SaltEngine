use std::sync::Arc;

use crate::connection::Connection;
use crate::network_client_notifier::NetworkClientNotifier;
use crate::network_prompter::NewtorkPrompter;
use async_trait::async_trait;
use log::info;
use protocol::entities::PlayerId;
use protocol::from_client::{ClientAction, FromClient};
use protocol::from_server::FromServer;
use salt_engine::game_agent::{ClientNotifier, GameClient};
use salt_engine::{game_agent::Prompter, game_state::GameState};

pub(crate) struct NetworkGameClient {
    player_id: PlayerId,
    connection: Connection,
    notifier: NetworkClientNotifier,
}

impl NetworkGameClient {
    pub(crate) fn new(player_id: PlayerId, connection: Connection) -> Self {
        Self {
            player_id,
            connection: connection.clone(),
            notifier: NetworkClientNotifier::new(connection),
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

    async fn make_prompter(&self) -> Arc<dyn Prompter> {
        Arc::new(NewtorkPrompter::new(self.connection.clone()))
    }

    async fn make_notifier(&self) -> Arc<dyn ClientNotifier> {
        Arc::new(NetworkClientNotifier::new(self.connection.clone()))
    }
}
