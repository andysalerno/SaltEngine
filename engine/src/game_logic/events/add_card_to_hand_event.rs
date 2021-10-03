use super::{ClientEventView, Event};
use crate::game_state::{
    board::BoardView, GameState, MakePlayerView, PlayerId, UnitCardInstanceId,
    UnitCardInstancePlayerView,
};
use serde::{Deserialize, Serialize};

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

    #[must_use]
    pub fn make_client_event(&self, game_state: &GameState) -> AddCardToHandClientEvent {
        let card = game_state
            .board()
            .creature_instance(self.card_id())
            .player_view(self.player_id);
        AddCardToHandClientEvent {
            player_id: self.player_id,
            card_id: self.card_id,
            card,
        }
    }
}

impl Event for AddCardToHandEvent {
    fn maybe_client_event(&self, game_state: &GameState) -> Option<ClientEventView> {
        let event = self.make_client_event(game_state);
        Some(ClientEventView::AddCardToHand(event))
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AddCardToHandClientEvent {
    pub player_id: PlayerId,
    pub card_id: UnitCardInstanceId,
    pub card: UnitCardInstancePlayerView,
}
