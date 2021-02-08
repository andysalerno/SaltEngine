use crate::{game_logic::cards::UnitCardDefinition, game_state::board::BoardPos, id::Id};

use super::{Event, GameEvent};

#[derive(Debug)]
pub struct CreatureSetEvent {
    player_id: Id,
    definition: Box<dyn UnitCardDefinition>,
    target_position: BoardPos,
}

impl CreatureSetEvent {
    pub fn new(
        player_id: Id,
        definition: Box<dyn UnitCardDefinition>,
        target_position: BoardPos,
    ) -> Self {
        Self {
            player_id,
            definition,
            target_position,
        }
    }

    pub fn player_id(&self) -> Id {
        self.player_id
    }

    pub fn definition(&self) -> &Box<dyn UnitCardDefinition> {
        &self.definition
    }

    pub fn take_definition(self) -> Box<dyn UnitCardDefinition> {
        self.definition
    }

    pub fn target_position(&self) -> BoardPos {
        self.target_position
    }
}

impl Event for CreatureSetEvent {}

impl Into<GameEvent> for CreatureSetEvent {
    fn into(self) -> GameEvent {
        GameEvent::Summon(self)
    }
}
