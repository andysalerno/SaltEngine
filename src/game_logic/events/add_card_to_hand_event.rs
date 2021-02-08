use crate::{
    game_state::{PlayerId, UnitCardInstance},
    id::Id,
};

use super::{Event, GameEvent};

#[derive(Debug)]
pub struct AddCardToHandEvent {
    player_id: PlayerId,
    card: UnitCardInstance,
}

impl AddCardToHandEvent {
    pub fn new(player_id: PlayerId, card: UnitCardInstance) -> Self {
        Self { player_id, card }
    }

    pub fn player_id(&self) -> PlayerId {
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
