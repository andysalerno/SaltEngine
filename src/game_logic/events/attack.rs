use crate::{
    game_state::{UnitCardInstance, UnitCardInstanceId},
    id::Id,
};

use super::{Event, GameEvent};

#[derive(Debug)]
pub struct AttackEvent {
    attacker: UnitCardInstanceId,
    target: UnitCardInstanceId,
}

impl AttackEvent {
    pub fn new(attacker: UnitCardInstanceId, target: UnitCardInstanceId) -> Self {
        Self { attacker, target }
    }

    pub fn attacker(&self) -> UnitCardInstanceId {
        self.attacker
    }

    pub fn target(&self) -> UnitCardInstanceId {
        self.target
    }
}

impl Event for AttackEvent {}

impl Into<GameEvent> for AttackEvent {
    fn into(self) -> GameEvent {
        GameEvent::Attack(self)
    }
}
