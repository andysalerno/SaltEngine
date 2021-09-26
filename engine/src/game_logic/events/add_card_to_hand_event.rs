

use crate::{
    game_state::{MakePlayerView, PlayerId, UnitCardInstance, UnitCardInstancePlayerView},
};
use serde::{Deserialize, Serialize};

use super::{Event, GameEvent};

#[derive(Debug)]
pub struct AddCardToHandEvent {
    player_id: PlayerId,
    card: UnitCardInstance,
}

impl AddCardToHandEvent {
    #[must_use] pub fn new(player_id: PlayerId, card: UnitCardInstance) -> Self {
        Self { player_id, card }
    }

    #[must_use] pub fn player_id(&self) -> PlayerId {
        self.player_id
    }

    #[must_use] pub fn take_card(self) -> UnitCardInstance {
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
    pub player_id: PlayerId,
    pub card: UnitCardInstancePlayerView,
}

impl<'a> MakePlayerView<'a> for AddCardToHandEvent {
    type TOut = AddCardToHandClientEvent;

    fn player_view(&'a self, player_viewing: PlayerId) -> <Self as MakePlayerView<'a>>::TOut {
        let card = self.card.player_view(player_viewing);

        AddCardToHandClientEvent {
            player_id: player_viewing,
            card,
        }
    }
}
