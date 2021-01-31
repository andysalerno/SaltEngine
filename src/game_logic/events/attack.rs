use crate::{game_state::UnitCardBoardInstance, id::Id};

use super::Event;

#[derive(Debug)]
pub struct AttackEvent {
    attacker: Id,
    target: Id,
}

impl AttackEvent {
    pub fn new(attacker: Id, target: Id) -> Self {
        Self { attacker, target }
    }

    pub fn attacker(&self) -> Id {
        self.attacker
    }

    pub fn target(&self) -> Id {
        self.target
    }
}

impl Event for AttackEvent {}
