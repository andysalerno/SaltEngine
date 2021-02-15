use crate::game_state::{GameState, PlayerId};
use crate::{game_logic::GameEvent, game_state::board::BoardPos};

pub trait GameAgent {
    fn get_action(&self, game_state: &GameState) -> GameEvent;
    fn id(&self) -> PlayerId;

    /// Prompt the player for for any position on the board.
    fn prompt_pos(&self, game_state: &GameState) -> BoardPos;
}
