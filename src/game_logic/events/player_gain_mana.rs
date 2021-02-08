use crate::{game_state::PlayerId, id::Id};

use super::{Event, GameEvent};

#[derive(Debug)]
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

impl Into<GameEvent> for PlayerGainManaEvent {
    fn into(self) -> GameEvent {
        GameEvent::GainMana(self)
    }
}

impl Event for PlayerGainManaEvent {}
