use crate::{connection::Connection, messages::FromServer};
use async_trait::async_trait;
use protocol::VisualEvent;
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
            .send(FromServer::NotifyEvent(event))
            .await
            .expect("Failed to notify the client");
    }
}
