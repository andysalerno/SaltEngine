use crate::game_state::UnitCardInstanceId;

use super::{Event, GameEvent};

#[derive(Debug, Clone)]
pub struct CreatureDestroyedEvent {
    creature_id: UnitCardInstanceId,
}

impl CreatureDestroyedEvent {
    #[must_use] pub fn new(creature_id: UnitCardInstanceId) -> Self {
        Self { creature_id }
    }

    #[must_use] pub fn creature_id(&self) -> UnitCardInstanceId {
        self.creature_id
    }
}

impl Event for CreatureDestroyedEvent {}

impl From<CreatureDestroyedEvent> for GameEvent {
    fn from(val: CreatureDestroyedEvent) -> Self {
        GameEvent::CreatureDestroyed(val)
    }
}
