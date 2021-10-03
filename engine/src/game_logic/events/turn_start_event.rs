use super::{ClientEventView, Event};
use crate::game_state::{GameState, PlayerId};

#[derive(Debug, Clone)]
pub struct TurnStartEvent(pub PlayerId);

impl Event for TurnStartEvent {
    fn maybe_client_event(&self, _game_state: &GameState) -> Option<ClientEventView> {
        Some(ClientEventView::TurnStarted(self.0))
    }

    fn validate<'a, G>(&self, game_state: &'a G) -> super::Result
    where
        G: crate::game_state::GameStateView<'a>,
    {
        if game_state.cur_player_turn() == self.0 {
            Ok(())
        } else {
            super::Result::Err("Turn start not valid for a different player's turn.".into())
        }
    }
}
