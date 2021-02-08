use crate::{game_logic::cards::UnitCardDefinition, game_state::board::BoardPos, id::Id};

use super::{Event, GameEvent};

#[derive(Debug)]
pub struct SummonCreatureFromHandEvent {
    player_id: Id,
    board_pos: BoardPos,
    creature_definition: Box<dyn UnitCardDefinition>,
}

impl SummonCreatureFromHandEvent {
    pub fn new(
        player_id: Id,
        board_pos: BoardPos,
        creature_definition: Box<dyn UnitCardDefinition>,
    ) -> Self {
        Self {
            player_id,
            board_pos,
            creature_definition,
        }
    }

    pub fn player_id(&self) -> Id {
        self.player_id
    }

    pub fn board_pos(&self) -> BoardPos {
        self.board_pos
    }

    pub fn take_definition(self) -> Box<dyn UnitCardDefinition> {
        self.creature_definition
    }

    pub fn definition(&self) -> &dyn UnitCardDefinition {
        self.creature_definition.as_ref()
    }
}

impl Event for SummonCreatureFromHandEvent {}

impl Into<GameEvent> for SummonCreatureFromHandEvent {
    fn into(self) -> GameEvent {
        GameEvent::SummonCreatureFromHand(self)
    }
}
