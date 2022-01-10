use crate::connection::Connection;
use async_trait::async_trait;
use protocol::from_server::{FromServer, VisualEvent};
use salt_engine::game_agent::ClientNotifier;

pub(crate) struct NetworkClientNotifier {
    connection: Connection,
}

impl NetworkClientNotifier {
    pub fn new(connection: Connection) -> Self {
        NetworkClientNotifier { connection }
    }
}

#[async_trait]
impl ClientNotifier for NetworkClientNotifier {
    async fn notify(&self, event: VisualEvent) {
        self.connection
            .send(FromServer::VisualEvent(event))
            .await
            .expect("Failed to notify the client");
    }
}
