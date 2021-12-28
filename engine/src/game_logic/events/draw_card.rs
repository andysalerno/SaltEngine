use super::Event;
use protocol::entities::PlayerId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawCardEvent {
    player_id: PlayerId,
}

impl DrawCardEvent {
    #[must_use]
    pub fn new(player_id: PlayerId) -> Self {
        Self { player_id }
    }

    #[must_use]
    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }
}

impl Event for DrawCardEvent {}
