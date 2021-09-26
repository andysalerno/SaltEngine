use crate::game_state::PlayerId;
use serde::{Deserialize, Serialize};

use super::{Event, GameEvent};

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

impl From<DrawCardEvent> for GameEvent {
    fn from(val: DrawCardEvent) -> Self {
        GameEvent::DrawCard(val)
    }
}

impl Event for DrawCardEvent {}
