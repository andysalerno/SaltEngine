use crate::game_state::{GameState, GameStatePlayerView, PlayerId};
use crate::{game_logic::GameEvent, game_state::board::BoardPos};

#[cfg(test)]
use mockall::{automock, predicate::*};

pub trait GameAgent {
    fn get_action(&self, game_state: &GameStatePlayerView) -> GameEvent;
    fn id(&self) -> PlayerId;
    fn make_prompter(&self) -> Box<dyn Prompter>;
}

#[cfg_attr(test, automock)]
pub trait Prompter {
    /// Prompt the player for for any position (slot) on the board.
    fn prompt_slot(&self, game_state: &GameStatePlayerView) -> BoardPos;

    /// Prompt the player for any position (slot) on the player's side of the board.
    fn prompt_player_slot(&self, game_state: &GameStatePlayerView) -> BoardPos;

    /// Prompt the player for any position (slot) on the opponent's side of the board.
    fn prompt_opponent_slot(&self, game_state: &GameStatePlayerView) -> BoardPos;

    /// Prompt the player for any slot in the board containing a creature.
    fn prompt_creature_pos(&self, game_state: &GameStatePlayerView) -> BoardPos;

    /// Prompt the player for a slot on their side of the board containing a creature.
    fn prompt_player_creature_pos(&self, game_state: &GameStatePlayerView) -> BoardPos;

    /// Prompt the player for a slot on the opponent's side of the board containing a creature.
    fn prompt_opponent_creature_pos(&self, game_state: &GameStatePlayerView) -> BoardPos;
}

impl std::fmt::Debug for dyn Prompter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{Prompter}}")
    }
}
