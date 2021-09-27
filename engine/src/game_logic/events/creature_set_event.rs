use crate::game_state::{
    board::BoardPos, MakePlayerView, PlayerId, UnitCardInstance, UnitCardInstanceId,
    UnitCardInstancePlayerView,
};
use serde::{Deserialize, Serialize};

use super::{ClientEventView, Event, GameEvent};

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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreatureSetClientEvent {
    pub player_id: PlayerId,
    pub card_id: UnitCardInstanceId,
    pub pos: BoardPos,
}

impl Event for CreatureSetEvent {
    fn maybe_client_event(&self) -> Option<ClientEventView> {
        Some(ClientEventView::UnitSet(CreatureSetClientEvent {
            player_id: self.player_id,
            card_id: self.card_id,
            pos: self.target_position,
        }))
    }
}

impl From<CreatureSetEvent> for GameEvent {
    fn from(val: CreatureSetEvent) -> Self {
        GameEvent::Summon(val)
    }
}
