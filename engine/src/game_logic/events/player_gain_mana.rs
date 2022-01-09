use protocol::entities::PlayerId;

use super::{Event, VisualEvent};
use crate::game_state::GameState;

#[derive(Debug, Clone)]
pub struct PlayerGainManaEvent {
    player_id: PlayerId,
    gain_count: u32,
}

impl PlayerGainManaEvent {
    #[must_use]
    pub fn new(player_id: PlayerId, gain_count: u32) -> Self {
        Self {
            player_id,
            gain_count,
        }
    }

    #[must_use]
    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }

    #[must_use]
    pub fn gain_count(&self) -> u32 {
        self.gain_count
    }
}

impl Event for PlayerGainManaEvent {
    fn maybe_client_event(
        &self,
        player_id: PlayerId,
        _game_state: &GameState,
    ) -> Option<VisualEvent> {
        // Some(ClientEventView::PlayerGainMana(
        //     self.player_id,
        //     self.gain_count as usize,
        // ))
        None
    }
}
