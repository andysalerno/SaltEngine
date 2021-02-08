use crate::game_state::UnitCardInstanceId;

use super::{Event, GameEvent};

#[derive(Debug)]
pub struct CreatureDestroyedEvent {
    creature_id: UnitCardInstanceId,
}

impl CreatureDestroyedEvent {
    pub fn new(creature_id: UnitCardInstanceId) -> Self {
        Self { creature_id }
    }

    pub fn creature_id(&self) -> UnitCardInstanceId {
        self.creature_id
    }
}

impl Event for CreatureDestroyedEvent {}

impl Into<GameEvent> for CreatureDestroyedEvent {
    fn into(self) -> GameEvent {
        GameEvent::CreatureDestroyed(self)
    }
}
