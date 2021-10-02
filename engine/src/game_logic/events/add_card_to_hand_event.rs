use crate::game_state::{MakePlayerView, PlayerId, UnitCardInstanceId};
use serde::{Deserialize, Serialize};

use super::{Event, GameEvent};

#[derive(Debug)]
pub struct AddCardToHandEvent {
    player_id: PlayerId,
    card_id: UnitCardInstanceId,
}

impl AddCardToHandEvent {
    #[must_use]
    pub fn new(player_id: PlayerId, card_id: UnitCardInstanceId) -> Self {
        Self { player_id, card_id }
    }

    #[must_use]
    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }

    #[must_use]
    pub fn card_id(&self) -> UnitCardInstanceId {
        self.card_id
    }
}

impl Event for AddCardToHandEvent {}

// impl From<AddCardToHandEvent> for GameEvent {
//     fn from(val: AddCardToHandEvent) -> Self {
//         GameEvent::AddCardToHand(val)
//     }
// }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AddCardToHandClientEvent {
    pub player_id: PlayerId,
    pub card_id: UnitCardInstanceId,
}

impl<'a> MakePlayerView<'a> for AddCardToHandEvent {
    type TOut = AddCardToHandClientEvent;

    fn player_view(&'a self, player_viewing: PlayerId) -> <Self as MakePlayerView<'a>>::TOut {
        AddCardToHandClientEvent {
            player_id: player_viewing,
            card_id: self.card_id,
        }
    }
}
