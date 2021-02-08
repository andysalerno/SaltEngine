use crate::{game_state::PlayerId, id::Id};

use super::{Event, GameEvent};

#[derive(Debug)]
pub struct PlayerSpendManaEvent {
    player_id: PlayerId,
    mana_count: u32,
}

impl PlayerSpendManaEvent {
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

impl Into<GameEvent> for PlayerSpendManaEvent {
    fn into(self) -> GameEvent {
        GameEvent::SpendMana(self)
    }
}

impl Event for PlayerSpendManaEvent {}
