use crate::game_logic::GameEvent;
use crate::game_state::{GameState, PlayerId};

pub trait GameAgent {
    fn get_action(&self, game_state: &GameState) -> GameEvent;
    fn id(&self) -> PlayerId;
}
