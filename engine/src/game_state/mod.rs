pub mod board;
mod card_instance;
mod deck;
mod game_state;
mod hand;
mod hero;
mod selector;

pub use card_instance::{InstanceState, UnitCardInstance, UnitCardInstanceId};
pub use deck::Deck;
pub use game_state::GameState;
pub use hand::Hand;
pub use selector::*;
use serde::{Deserialize, Serialize};

use crate::id::Id;

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerId(Id);

impl PlayerId {
    pub fn new() -> Self {
        Self(Id::new())
    }
}

#[cfg(test)]
pub use game_state::tests::*;
