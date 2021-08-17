use crate::{
    cards::player_view::UnitCardDefinitionPlayerView,
    game_state::{PlayerId, UnitCardInstance},
};
use serde::{Deserialize, Serialize};

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

impl From<AddCardToHandEvent> for GameEvent {
    fn from(val: AddCardToHandEvent) -> Self {
        GameEvent::AddCardToHand(val)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AddCardToHandClientEvent {
    player_id: PlayerId,
    card: UnitCardDefinitionPlayerView,
}
