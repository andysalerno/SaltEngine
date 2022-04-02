use super::Event;
use protocol::entities::CreatureInstanceId;

#[derive(Debug, Clone)]
pub struct CreatureDestroyedEvent {
    creature_id: CreatureInstanceId,
}

impl CreatureDestroyedEvent {
    #[must_use]
    pub fn new(creature_id: CreatureInstanceId) -> Self {
        Self { creature_id }
    }

    #[must_use]
    pub fn creature_id(&self) -> CreatureInstanceId {
        self.creature_id
    }
}

impl Event for CreatureDestroyedEvent {}
