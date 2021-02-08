use crate::{
    game_state::{UnitCardBoardInstanceId, UnitCardInstance},
    id::Id,
};

use super::{Event, GameEvent};

#[derive(Debug)]
pub struct AttackEvent {
    attacker: UnitCardBoardInstanceId,
    target: UnitCardBoardInstanceId,
}

impl AttackEvent {
    pub fn new(attacker: UnitCardBoardInstanceId, target: UnitCardBoardInstanceId) -> Self {
        Self { attacker, target }
    }

    pub fn attacker(&self) -> UnitCardBoardInstanceId {
        self.attacker
    }

    pub fn target(&self) -> UnitCardBoardInstanceId {
        self.target
    }
}

impl Event for AttackEvent {}

impl Into<GameEvent> for AttackEvent {
    fn into(self) -> GameEvent {
        GameEvent::Attack(self)
    }
}
