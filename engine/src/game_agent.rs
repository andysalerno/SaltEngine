use crate::game_logic::events::ClientEventView;
use crate::game_state::board::BoardPos;
use crate::game_state::GameStatePlayerView;
use async_trait::async_trait;

#[cfg(test)]
use mockall::{automock, predicate::str};

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

/// A trait for notifying game clients about game events so they can update their visual state.
/// Every `GameAgent` will be able to provide one of these.
#[async_trait]
pub trait ClientNotifier: Send + Sync {
    async fn notify(&self, event: ClientEventView);
}

impl std::fmt::Debug for dyn ClientNotifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ClientNotifier}}")
    }
}
