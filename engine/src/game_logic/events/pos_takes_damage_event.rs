use crate::game_state::board::BoardPos;

use super::Event;

#[derive(Debug, Clone)]
pub struct PosTakesDamageEvent {
    pos: BoardPos,
    damage_amount: usize,
}

impl PosTakesDamageEvent {
    #[must_use]
    pub fn new(pos: BoardPos, damage_amount: usize) -> Self {
        Self { pos, damage_amount }
    }

    #[must_use]
    pub fn pos(&self) -> BoardPos {
        self.pos
    }

    #[must_use]
    pub fn damage_amount(&self) -> usize {
        self.damage_amount
    }
}

impl Event for PosTakesDamageEvent {}
