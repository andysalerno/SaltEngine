use crate::game_state::PlayerId;

use super::{Event, GameEvent};

#[derive(Debug, Clone)]
pub struct PlayerGainManaEvent {
    player_id: PlayerId,
    mana_count: u32,
}

impl PlayerGainManaEvent {
    pub fn new(player_id: PlayerId, mana_count: u32) -> Self {
        Self {
            player_id,
            mana_count,
        }
    }

    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }

    pub fn mana_count(&self) -> u32 {
        self.mana_count
    }
}

impl From<PlayerGainManaEvent> for GameEvent {
    fn from(val: PlayerGainManaEvent) -> Self {
        GameEvent::GainMana(val)
    }
}

impl Event for PlayerGainManaEvent {}
