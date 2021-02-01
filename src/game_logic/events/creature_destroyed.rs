use crate::id::Id;

use super::Event;

#[derive(Debug)]
pub struct CreatureDestroyedEvent {
    creature_id: Id,
}

impl CreatureDestroyedEvent {
    pub fn new(creature_id: Id) -> Self {
        Self { creature_id }
    }

    pub fn creature_id(&self) -> Id {
        self.creature_id
    }
}

impl Event for CreatureDestroyedEvent {}
