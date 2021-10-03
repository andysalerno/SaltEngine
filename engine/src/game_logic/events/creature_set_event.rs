use crate::game_state::{
    board::{BoardPos, BoardView},
    GameState, MakePlayerView, PlayerId, UnitCardInstanceId,
    UnitCardInstancePlayerView,
};
use serde::{Deserialize, Serialize};

use super::{ClientEventView, Event};

#[derive(Debug)]
pub struct CreatureSetEvent {
    player_id: PlayerId,
    card_id: UnitCardInstanceId,
    target_position: BoardPos,
}

impl CreatureSetEvent {
    #[must_use]
    pub fn new(
        player_id: PlayerId,
        card_id: UnitCardInstanceId,
        target_position: BoardPos,
    ) -> Self {
        Self {
            player_id,
            card_id,
            target_position,
        }
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
    pub fn target_position(&self) -> BoardPos {
        self.target_position
    }

    pub fn make_client_event(&self, game_state: &GameState) -> CreatureSetClientEvent {
        let view = game_state
            .board()
            .creature_instance(self.card_id())
            .player_view(self.player_id); // todo: this is wrong, since both players get the owning player's view

        CreatureSetClientEvent {
            player_id: self.player_id,
            card_id: self.card_id,
            pos: self.target_position,
            card: view,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreatureSetClientEvent {
    pub player_id: PlayerId,
    pub card_id: UnitCardInstanceId,
    pub pos: BoardPos,
    pub card: UnitCardInstancePlayerView,
}

impl Event for CreatureSetEvent {
    fn maybe_client_event(&self, game_state: &GameState) -> Option<ClientEventView> {
        let event = self.make_client_event(game_state);
        Some(ClientEventView::UnitSet(event))
    }
}
