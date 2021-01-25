use crate::game_runner::GameDisplay;

pub struct ConsoleDisplay;

impl GameDisplay for ConsoleDisplay {
    fn display(&mut self, game_state: &crate::game_state::GameState) {
        todo!()
    }
}
