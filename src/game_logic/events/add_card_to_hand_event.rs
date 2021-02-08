use crate::{game_state::UnitCardInstance, id::Id};

use super::{Event, GameEvent};

#[derive(Debug)]
pub struct AddCardToHandEvent {
    player_id: Id,
    card: UnitCardInstance,
}

impl AddCardToHandEvent {
    pub fn new(player_id: Id, card: UnitCardInstance) -> Self {
        Self { player_id, card }
    }

    pub fn player_id(&self) -> Id {
        self.player_id
    }

    pub fn take_card(self) -> UnitCardInstance {
        self.card
    }
}

impl Event for AddCardToHandEvent {}

impl Into<GameEvent> for AddCardToHandEvent {
    fn into(self) -> GameEvent {
        GameEvent::AddCardToHand(self)
    }
}
