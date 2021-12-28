use protocol::entities::UnitCardInstanceId;
use serde::{Deserialize, Serialize};

use super::Event;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttackEvent {
    attacker: UnitCardInstanceId,
    target: UnitCardInstanceId,
}

impl AttackEvent {
    #[must_use]
    pub fn new(attacker: UnitCardInstanceId, target: UnitCardInstanceId) -> Self {
        Self { attacker, target }
    }

    #[must_use]
    pub fn attacker(&self) -> UnitCardInstanceId {
        self.attacker
    }

    #[must_use]
    pub fn target(&self) -> UnitCardInstanceId {
        self.target
    }
}

impl Event for AttackEvent {}
