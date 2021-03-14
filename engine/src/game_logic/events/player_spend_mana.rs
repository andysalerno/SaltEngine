use crate::game_state::PlayerId;

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

impl Event for PlayerSpendManaEvent {
    fn validate(&self, game_state: &crate::game_state::GameState) -> super::Result {
        let mana_count = game_state.player_mana(self.player_id());

        if mana_count >= self.mana_count() {
            Ok(())
        } else {
            Err(format!(
                "Player {:?} only has {} mana, but tried to spend {} mana.",
                self.player_id(),
                mana_count,
                self.mana_count()
            )
            .into())
        }
    }
}
