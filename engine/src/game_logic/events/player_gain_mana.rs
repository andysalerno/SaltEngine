use crate::game_state::PlayerId;

use super::{Event, GameEvent};

#[derive(Debug, Clone)]
pub struct PlayerGainManaEvent {
    player_id: PlayerId,
    gain_count: u32,
}

impl PlayerGainManaEvent {
    pub fn new(player_id: PlayerId, gain_count: u32) -> Self {
        Self {
            player_id,
            gain_count,
        }
    }

    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }

    pub fn gain_count(&self) -> u32 {
        self.gain_count
    }
}

impl From<PlayerGainManaEvent> for GameEvent {
    fn from(val: PlayerGainManaEvent) -> Self {
        GameEvent::GainMana(val)
    }
}

impl Event for PlayerGainManaEvent {
    fn maybe_client_event(&self) -> Option<super::ClientEventView> {
        Some(super::ClientEventView::PlayerGainMana(
            self.player_id,
            self.gain_count as usize,
        ))
    }
}
