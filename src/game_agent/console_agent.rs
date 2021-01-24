use super::game_agent::GameAgent;
use crate::game_logic::game_event::GameEvent;
use crate::game_state::GameState;

pub struct ConsoleAgent;

impl GameAgent for ConsoleAgent {
    fn get_action(game_state: &GameState) -> Box<dyn GameEvent> {
        todo!()
    }
}
