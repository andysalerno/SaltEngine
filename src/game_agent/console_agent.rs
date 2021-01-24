use super::game_agent::GameAgent;
use crate::game_logic::game_event::GameEvent;
use crate::game_state::GameState;
use crate::id;
use crate::id::HasId;
use crate::id::Id;

pub struct ConsoleAgent {
    id: Id,
}

impl ConsoleAgent {
    pub fn new() -> Self {
        Self { id: id::new_id() }
    }
}

impl HasId for ConsoleAgent {
    fn id(&self) -> Id {
        self.id
    }
}

impl GameAgent for ConsoleAgent {
    fn get_action(&self, game_state: &GameState) -> Box<dyn GameEvent> {
        todo!()
    }
}
