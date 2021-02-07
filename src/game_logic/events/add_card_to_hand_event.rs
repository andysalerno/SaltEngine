use crate::{game_logic::cards::UnitCardDefinition, id::Id};

use super::{Event, GameEvent};

#[derive(Debug)]
pub struct AddCardToHandEvent {
    player_id: Id,
    card: Box<dyn UnitCardDefinition>,
}

impl AddCardToHandEvent {
    pub fn new(player_id: Id, card: Box<dyn UnitCardDefinition>) -> Self {
        Self { player_id, card }
    }

    pub fn player_id(&self) -> Id {
        self.player_id
    }

    pub fn take_card(self) -> Box<dyn UnitCardDefinition> {
        self.card
    }
}

impl Event for AddCardToHandEvent {}

impl Into<GameEvent> for AddCardToHandEvent {
    fn into(self) -> GameEvent {
        GameEvent::AddCardToHand(self)
    }
}
