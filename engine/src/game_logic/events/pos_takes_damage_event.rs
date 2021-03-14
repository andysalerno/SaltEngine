use crate::game_state::board::BoardPos;

use super::{Event, GameEvent};

#[derive(Debug)]
pub struct PosTakesDamageEvent {
    pos: BoardPos,
    damage_amount: usize,
}

impl PosTakesDamageEvent {
    pub fn new(pos: BoardPos, damage_amount: usize) -> Self {
        Self { pos, damage_amount }
    }

    pub fn pos(&self) -> BoardPos {
        self.pos
    }

    pub fn damage_amount(&self) -> usize {
        self.damage_amount
    }
}

impl Event for PosTakesDamageEvent {}

impl Into<GameEvent> for PosTakesDamageEvent {
    fn into(self) -> GameEvent {
        GameEvent::PosTakesDamage(self)
    }
}
