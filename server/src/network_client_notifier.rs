use async_trait::async_trait;
use salt_engine::game_agent::game_agent::ClientNotifier;

pub(crate) struct NetworkClientNotifier {}

impl NetworkClientNotifier {
    pub fn new() -> Self {
        NetworkClientNotifier {}
    }
}

#[async_trait]
impl ClientNotifier for NetworkClientNotifier {
    async fn notify(&self, _event: salt_engine::game_logic::ClientEventView) {
        todo!()
    }
}
