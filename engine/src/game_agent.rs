use crate::game_state::GameStatePlayerView;
use async_trait::async_trait;

#[cfg(test)]
use mockall::{automock, predicate::str};
use protocol::{entities::BoardPos, from_server::VisualEvent};

#[cfg_attr(test, automock)]
pub trait Prompter: Send + Sync {
    /// Prompt the player for for any position (slot) on the board.
    fn prompt_slot(&self) -> BoardPos;

    /// Prompt the player for any position (slot) on the player's side of the board.
    fn prompt_player_slot(&self) -> BoardPos;

    /// Prompt the player for any position (slot) on the opponent's side of the board.
    fn prompt_opponent_slot(&self) -> BoardPos;

    /// Prompt the player for any slot in the board containing a creature.
    fn prompt_creature_pos(&self) -> BoardPos;

    /// Prompt the player for a slot on their side of the board containing a creature.
    fn prompt_player_creature_pos(&self) -> BoardPos;

    /// Prompt the player for a slot on the opponent's side of the board containing a creature.
    fn prompt_opponent_creature_pos(&self) -> BoardPos;
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
    async fn notify(&self, event: VisualEvent);
}

impl std::fmt::Debug for dyn ClientNotifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ClientNotifier}}")
    }
}

#[cfg(test)]
pub mod tests {
    use super::{ClientNotifier, Prompter};
    use crate::game_state::GameStatePlayerView;
    use async_trait::async_trait;
    use mockall::mock;
    use protocol::{entities::BoardPos, from_server::VisualEvent};

    mock! {
        pub(crate) TestPrompter {}
        impl Prompter for TestPrompter {
            fn prompt_slot(&self) -> BoardPos;
            fn prompt_player_slot(&self) -> BoardPos;
            fn prompt_opponent_slot(&self) -> BoardPos;
            fn prompt_creature_pos(&self) -> BoardPos;
            fn prompt_player_creature_pos(&self) -> BoardPos;
            fn prompt_opponent_creature_pos(&self) -> BoardPos;
        }
    }

    pub(crate) struct StubNotifier;

    #[async_trait]
    impl ClientNotifier for StubNotifier {
        async fn notify(&self, _event: VisualEvent) {
            // Do nothing for the stub
        }
    }
}
