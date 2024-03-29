use crate::game_state::game_state::GameState;

use super::{Event, VisualEvent};
use protocol::entities::PlayerId;

#[derive(Debug, Clone)]
pub struct PlayerSpendManaEvent {
    player_id: PlayerId,
    mana_count: u32,
}

impl PlayerSpendManaEvent {
    #[must_use]
    pub fn new(player_id: PlayerId, mana_count: u32) -> Self {
        Self {
            player_id,
            mana_count,
        }
    }

    #[must_use]
    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }

    #[must_use]
    pub fn mana_count(&self) -> u32 {
        self.mana_count
    }
}

impl Event for PlayerSpendManaEvent {
    // fn validate(&self, game_state: &GameState) -> super::Result {
    //     let mana_count = game_state.player_mana(self.player_id());

    //     if mana_count >= self.mana_count() {
    //         Ok(())
    //     } else {
    //         Err(format!(
    //             "Player {:?} only has {} mana, but tried to spend {} mana.",
    //             self.player_id(),
    //             mana_count,
    //             self.mana_count()
    //         )
    //         .into())
    //     }
    // }

    // fn maybe_client_event(
    //     &self,
    //     player_id: PlayerId,
    //     _game_state: &GameState,
    // ) -> Option<VisualEvent> {
    //     None
    //     // Some(ClientEventView::PlayerSpendMana {
    //     //     player_id: self.player_id,
    //     //     spent_mana_count: self.mana_count as usize,
    //     // })
    // }
}
