use crate::game_logic::game_event::GameEvent;
use crate::game_state::GameState;

pub trait GameAgent {
    fn get_action(game_state: &GameState) -> Box<dyn GameEvent>;
}
