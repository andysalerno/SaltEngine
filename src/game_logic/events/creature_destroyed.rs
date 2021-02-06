use crate::game_state::UnitCardBoardInstanceId;

use super::Event;

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
