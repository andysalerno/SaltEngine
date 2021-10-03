use super::{Event};
use crate::{game_logic::buff::BuiltBuff, game_state::UnitCardInstanceId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddBuffToCardInstanceEvent {
    buff: BuiltBuff,
    recipient_id: UnitCardInstanceId,
}

impl AddBuffToCardInstanceEvent {
    #[must_use]
    pub fn new(buff: BuiltBuff, recipient_id: UnitCardInstanceId) -> Self {
        Self { buff, recipient_id }
    }

    #[must_use]
    pub fn buff(&self) -> &BuiltBuff {
        &self.buff
    }

    #[must_use]
    pub fn recipient(&self) -> UnitCardInstanceId {
        self.recipient_id
    }
}

impl Event for AddBuffToCardInstanceEvent {}
