use std::borrow::Borrow;

use crate::{
    cards::player_view::UnitCardDefinitionPlayerView,
    game_state::{MakePlayerView, PlayerId, UnitCardInstance},
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

impl<'a> MakePlayerView<'a> for AddCardToHandEvent {
    type TOut = AddCardToHandClientEvent;

    fn player_view(&'a self, player_viewing: PlayerId) -> <Self as MakePlayerView<'a>>::TOut {
        // todo!()
        let card = self.card.player_view(player_viewing);
        let definition_view = card.definition().clone();

        AddCardToHandClientEvent {
            player_id: player_viewing,
            card: definition_view,
        }
    }
}

// impl<T: Borrow<AddCardToHandEvent> + 'static> From<T> for AddCardToHandClientEvent {
// impl From<&'static AddCardToHandEvent> for AddCardToHandClientEvent {
//     fn from(event: &'static AddCardToHandEvent) -> Self {
//         //let event = event.borrow();
//         let definition = event.card.definition();
//         let definition_view = definition.player_view(event.player_id);

//         AddCardToHandClientEvent {
//             player_id: event.player_id,
//             card: definition_view,
//         }
//     }
// }
