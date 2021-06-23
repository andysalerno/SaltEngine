use crate::game_state::board::BoardPos;
use crate::{
    game_logic::ClientGameEvent,
    game_state::{GameStatePlayerView, PlayerId},
};

#[cfg(test)]
use mockall::{automock, predicate::*};

/// A trait representing an player agent that can decide what
/// action to take for a given game state.
pub trait GameAgent {
    fn get_action(&self, game_state: &GameStatePlayerView) -> ClientGameEvent;
    fn id(&self) -> PlayerId;
    fn make_prompter(&self) -> Box<dyn Prompter>;

    fn observe_state_update(&self, _game_state: GameStatePlayerView) {
        // no implementation by default
    }
}

#[cfg_attr(test, automock)]
pub trait Prompter: Send + Sync {
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
