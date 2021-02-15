pub mod board;
mod card_instance;
mod deck;
mod game_state;
mod hand;

pub use card_instance::{InstanceState, UnitCardInstance, UnitCardInstanceId};
pub use deck::Deck;
pub use game_state::GameState;
pub use hand::Hand;

use crate::id::Id;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PlayerId(Id);

impl PlayerId {
    pub fn new() -> Self {
        Self(Id::new())
    }
}
