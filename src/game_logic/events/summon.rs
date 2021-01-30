use crate::{game_logic::cards::UnitCardDefinition, game_state::board::BoardPos};

use super::Event;

#[derive(Debug)]
pub struct SummonCreatureEvent {
    definition: Box<dyn UnitCardDefinition>,
    target_position: BoardPos,
}

impl SummonCreatureEvent {
    pub fn new(definition: Box<dyn UnitCardDefinition>, target_position: BoardPos) -> Self {
        Self {
            definition,
            target_position,
        }
    }

    pub fn definition(&self) -> &Box<dyn UnitCardDefinition> {
        &self.definition
    }

    pub fn target_position(&self) -> BoardPos {
        self.target_position
    }
}

impl Event for SummonCreatureEvent {}
