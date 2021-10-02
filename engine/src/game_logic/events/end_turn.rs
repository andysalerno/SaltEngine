use crate::game_state::PlayerId;

use super::{Event, GameEvent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EndTurnEvent(pub PlayerId);

impl Event for EndTurnEvent {
    fn validate<'a, G>(&self, game_state: &'a G) -> super::Result
    where
        G: crate::game_state::GameStateView<'a>,
    {
        if game_state.cur_player_turn() != self.0 {
            super::Result::Err("Cannot end a different player's turn.".into())
        } else {
            Ok(())
        }
    }

    fn maybe_client_event(&self) -> Option<super::ClientEventView> {
        Some(super::ClientEventView::TurnEnded(self.0))
    }
}

// impl From<EndTurnEvent> for GameEvent {
//     fn from(val: EndTurnEvent) -> Self {
//         GameEvent::EndTurn(val)
//     }
// }
