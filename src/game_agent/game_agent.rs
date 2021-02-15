use crate::game_state::{GameState, PlayerId};
use crate::{game_logic::GameEvent, game_state::board::BoardPos};

pub trait GameAgent {
    fn get_action(&self, game_state: &GameState) -> GameEvent;
    fn id(&self) -> PlayerId;
    fn make_prompter(&self) -> Box<dyn Prompter>;
}

pub trait Prompter: std::fmt::Debug {
    /// Prompt the player for for any position (slot) on the board.
    fn prompt_slot(&self, game_state: &GameState) -> BoardPos;

    /// Prompt the player for any position (slot) on the player's side of the board.
    fn prompt_player_slot(&self, game_state: &GameState) -> BoardPos;

    /// Prompt the player for any position (slot) on the opponent's side of the board.
    fn prompt_opponent_slot(&self, game_state: &GameState) -> BoardPos;

    /// Prompt the player for any slot in the board containing a creature.
    fn prompt_creature_pos(&self, game_state: &GameState) -> BoardPos;

    /// Prompt the player for a slot on their side of the board containing a creature.
    fn prompt_player_creature_pos(&self, game_state: &GameState) -> BoardPos;

    /// Prompt the player for a slot on the opponent's side of the board containing a creature.
    fn prompt_opponent_creature_pos(&self, game_state: &GameState) -> BoardPos;
}
