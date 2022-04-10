use super::Event;
use protocol::entities::CreatureInstanceId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddBuffToCardInstanceEvent {
    buff: BuiltBuff,
    recipient_id: CreatureInstanceId,
}

impl AddBuffToCardInstanceEvent {
    #[must_use]
    pub fn new(buff: BuiltBuff, recipient_id: CreatureInstanceId) -> Self {
        Self { buff, recipient_id }
    }

    #[must_use]
    pub fn buff(&self) -> &BuiltBuff {
        &self.buff
    }

    #[must_use]
    pub fn recipient(&self) -> CreatureInstanceId {
        self.recipient_id
    }
}

impl Event for AddBuffToCardInstanceEvent {}
