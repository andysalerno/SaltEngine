use crate::game_state::game_state::GameState;

use super::{Event, VisualEvent};
use protocol::{entities::PlayerId, visual_events::TurnStarted};

#[derive(Debug, Clone)]
pub struct TurnStartEvent(pub PlayerId);

impl Event for TurnStartEvent {
    fn maybe_client_event(
        &self,
        player_id: PlayerId,
        _game_state: &GameState,
    ) -> Option<VisualEvent> {
        Some(VisualEvent::TurnStarted(TurnStarted { player_id }))
    }

    fn validate(&self, game_state: &GameState) -> super::Result {
        if game_state.cur_player_turn() == self.0 {
            Ok(())
        } else {
            super::Result::Err("Turn start not valid for a different player's turn.".into())
        }
    }
}
