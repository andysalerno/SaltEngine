use crate::game_state::GameState;
use crate::{game_logic::GameEvent, id::HasId};

pub trait GameAgent: HasId {
    fn get_action(&self, game_state: &GameState) -> GameEvent;
}
