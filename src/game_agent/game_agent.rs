use crate::game_logic::game_event::GameEvent;
use crate::game_state::GameState;
use crate::id::HasId;

pub trait GameAgent: HasId {
    fn get_action(&self, game_state: &GameState) -> Box<dyn GameEvent>;
}
