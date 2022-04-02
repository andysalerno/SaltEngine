use protocol::entities::CreatureInstanceId;
use serde::{Deserialize, Serialize};

use super::Event;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttackEvent {
    attacker: CreatureInstanceId,
    target: CreatureInstanceId,
}

impl AttackEvent {
    #[must_use]
    pub fn new(attacker: CreatureInstanceId, target: CreatureInstanceId) -> Self {
        Self { attacker, target }
    }

    #[must_use]
    pub fn attacker(&self) -> CreatureInstanceId {
        self.attacker
    }

    #[must_use]
    pub fn target(&self) -> CreatureInstanceId {
        self.target
    }
}

impl Event for AttackEvent {}
