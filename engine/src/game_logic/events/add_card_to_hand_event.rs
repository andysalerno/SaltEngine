use super::{Event, VisualEvent};
use crate::game_state::{GameState, MakePlayerView, UnitCardInstancePlayerView};
use protocol::entities::{CreatureInstanceId, PlayerId};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct AddCardToHandEvent {
    player_id: PlayerId,
    card_id: CreatureInstanceId,
}

impl AddCardToHandEvent {
    #[must_use]
    pub fn new(player_id: PlayerId, card_id: CreatureInstanceId) -> Self {
        Self { player_id, card_id }
    }

    #[must_use]
    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }

    #[must_use]
    pub fn card_id(&self) -> CreatureInstanceId {
        self.card_id
    }
}

impl Event for AddCardToHandEvent {
    fn maybe_client_event(
        &self,
        player_id: PlayerId,
        game_state: &GameState,
    ) -> Option<VisualEvent> {
        let card = if self.player_id == player_id {
            let card = game_state
                .board()
                .creature_instance_all(self.card_id())
                .player_view(self.player_id);

            Some(card)
        } else {
            None
        };

        let event = AddCardToHandClientEvent {
            player_id: self.player_id,
            card_id: self.card_id,
            card,
        };
        // Some(ClientEventView::AddCardToHand(event))
        None
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AddCardToHandClientEvent {
    pub player_id: PlayerId,
    pub card_id: CreatureInstanceId,
    pub card: Option<UnitCardInstancePlayerView>,
}
