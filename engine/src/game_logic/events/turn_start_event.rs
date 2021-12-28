use super::{ClientEventView, Event};
use crate::game_state::GameState;
use protocol::{client_event_views::TurnStarted, entities::PlayerId};

#[derive(Debug, Clone)]
pub struct TurnStartEvent(pub PlayerId);

impl Event for TurnStartEvent {
    fn maybe_client_event(
        &self,
        player_id: PlayerId,
        _game_state: &GameState,
    ) -> Option<ClientEventView> {
        Some(ClientEventView::TurnStarted(TurnStarted { player_id }))
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
