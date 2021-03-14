use crate::game_state::PlayerId;

use super::{Event, GameEvent};

#[derive(Debug)]
pub struct DrawCardEvent {
    player_id: PlayerId,
}

impl DrawCardEvent {
    pub fn new(player_id: PlayerId) -> Self {
        Self { player_id }
    }

    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }
}

impl Into<GameEvent> for DrawCardEvent {
    fn into(self) -> GameEvent {
        GameEvent::DrawCard(self)
    }
}

impl Event for DrawCardEvent {}
