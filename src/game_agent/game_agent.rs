use crate::game_state::{GameState, PlayerId};
use crate::{game_logic::GameEvent, id::Id};

pub trait GameAgent {
    fn get_action(&self, game_state: &GameState) -> GameEvent;
    fn id(&self) -> PlayerId;
}
