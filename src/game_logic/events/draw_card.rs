use crate::id::Id;

use super::{Event, GameEvent};

#[derive(Debug)]
pub struct DrawCardEvent {
    player_id: Id,
}

impl DrawCardEvent {
    pub fn new(player_id: Id) -> Self {
        Self { player_id }
    }

    pub fn player_id(&self) -> Id {
        self.player_id
    }
}

impl Into<GameEvent> for DrawCardEvent {
    fn into(self) -> GameEvent {
        GameEvent::DrawCard(self)
    }
}

impl Event for DrawCardEvent {}
