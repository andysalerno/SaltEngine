use crate::id::HasId;
use crate::{game_logic::Event, game_state::GameState};

pub trait GameAgent: HasId {
    fn get_action(&self, game_state: &GameState) -> Box<dyn Event>;
}
