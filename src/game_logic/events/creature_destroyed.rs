use crate::{
    game_state::UnitCardBoardInstanceId,
};

use super::{Event, GameEvent};

#[derive(Debug)]
pub struct CreatureDestroyedEvent {
    creature_id: UnitCardBoardInstanceId,
}

impl CreatureDestroyedEvent {
    pub fn new(creature_id: UnitCardBoardInstanceId) -> Self {
        Self { creature_id }
    }

    pub fn creature_id(&self) -> UnitCardBoardInstanceId {
        self.creature_id
    }
}

impl Event for CreatureDestroyedEvent {}

impl Into<GameEvent> for CreatureDestroyedEvent {
    fn into(self) -> GameEvent {
        GameEvent::CreatureDestroyed(self)
    }
}
