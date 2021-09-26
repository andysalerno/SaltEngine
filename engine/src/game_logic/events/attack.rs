use crate::game_state::UnitCardInstanceId;
use serde::{Deserialize, Serialize};

use super::{Event, GameEvent};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttackEvent {
    attacker: UnitCardInstanceId,
    target: UnitCardInstanceId,
}

impl AttackEvent {
    #[must_use] pub fn new(attacker: UnitCardInstanceId, target: UnitCardInstanceId) -> Self {
        Self { attacker, target }
    }

    #[must_use] pub fn attacker(&self) -> UnitCardInstanceId {
        self.attacker
    }

    #[must_use] pub fn target(&self) -> UnitCardInstanceId {
        self.target
    }
}

impl Event for AttackEvent {}

impl From<AttackEvent> for GameEvent {
    fn from(val: AttackEvent) -> Self {
        GameEvent::Attack(val)
    }
}
