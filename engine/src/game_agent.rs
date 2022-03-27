use std::sync::Arc;

use async_trait::async_trait;

#[cfg(test)]
use mockall::{automock, predicate::str};
use protocol::{
    entities::BoardPos,
    from_client::ClientAction,
    from_server::{Notification, VisualEvent},
};

use crate::game_state::GameState;

/// A trait that defines the interaction between the GameRunner
/// and the client.
/// The GameRunner is the rules engine, and it will use the
/// GameClient for each player client to alert that client
/// to events, and to receive input from the player client.
#[async_trait]
pub trait GameClient: Send + Sync {
    async fn on_turn_start(&mut self, game_state: &GameState);

    // rename to "receive_input"
    async fn next_action(&mut self) -> ClientAction;

    // make "notify"
    async fn make_prompter(&self) -> Arc<dyn Prompter>;
    async fn make_notifier(&self) -> Arc<dyn ClientNotifier>;
}

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
    async fn notify(&self, event: Notification);
}

impl std::fmt::Debug for dyn ClientNotifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ClientNotifier}}")
    }
}

#[cfg(test)]
pub mod tests {
    use super::{ClientNotifier, Prompter};
    use async_trait::async_trait;
    use mockall::mock;
    use protocol::{entities::BoardPos, from_server::Notification};

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
        async fn notify(&self, _event: Notification) {
            // Do nothing for the stub
        }
    }
}
