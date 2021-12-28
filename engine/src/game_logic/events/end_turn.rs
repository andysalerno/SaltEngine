use crate::game_state::GameState;

use super::{ClientEventView, Event};
use protocol::entities::PlayerId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EndTurnEvent(pub PlayerId);

impl Event for EndTurnEvent {
    fn validate<'a, G>(&self, game_state: &'a G) -> super::Result
    where
        G: crate::game_state::GameStateView<'a>,
    {
        if game_state.cur_player_turn() == self.0 {
            Ok(())
        } else {
            super::Result::Err("Cannot end a different player's turn.".into())
        }
    }

    fn maybe_client_event(
        &self,
        player_id: PlayerId,
        _game_state: &GameState,
    ) -> Option<ClientEventView> {
        None
        // Some(ClientEventView::TurnEnded(self.0))
    }
}
