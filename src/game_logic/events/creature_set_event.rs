use crate::{
    game_logic::cards::UnitCardDefinition,
    game_state::{board::BoardPos, UnitCardInstance},
    id::Id,
};

use super::{Event, GameEvent};

#[derive(Debug)]
pub struct CreatureSetEvent {
    player_id: Id,
    card: UnitCardInstance,
    target_position: BoardPos,
}

impl CreatureSetEvent {
    pub fn new(player_id: Id, card: UnitCardInstance, target_position: BoardPos) -> Self {
        Self {
            player_id,
            card,
            target_position,
        }
    }

    pub fn player_id(&self) -> Id {
        self.player_id
    }

    pub fn card(&self) -> &UnitCardInstance {
        &self.card
    }

    pub fn take_card(self) -> UnitCardInstance {
        self.card
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
