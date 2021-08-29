pub mod board;
mod card_instance;
mod deck;
mod game_state;
mod hand;
mod hero;
mod selector;

pub use card_instance::{
    InstanceState, UnitCardInstance, UnitCardInstanceId, UnitCardInstancePlayerView,
    UnitCardInstanceView,
};
pub use deck::Deck;
pub use game_state::{player_view::GameStatePlayerView, GameState, GameStateView};
pub use hand::{Hand, HandView};
pub use selector::iter_helpers::{IterAddons, IteratorAny};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::id::Id;

#[derive(Debug, Eq, Hash, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PlayerId(Id);

impl PlayerId {
    pub fn new() -> Self {
        Self(Id::new())
    }
}

/// A marker trait, indicating that the implementor
/// is a player-friendly "view" of the associated type.
// trait PlayerView {
//     type TViewOf;
// }

/// Similar in concept to Clone,
/// but specifically with the intent of creating
/// a copy of T that is intended for viewing in a player's
/// client -- i.e., only containing info visible to the player,
/// and not info invisible to them (such as the content of the opponent's hand).
// pub trait MakePlayerView<'a, 'b>
pub trait MakePlayerView<'a> // where
//     'b: 'a,
{
    type TOut: Serialize + DeserializeOwned + Sized + 'static;
    fn player_view(&'a self, player_viewing: PlayerId) -> <Self as MakePlayerView<'a>>::TOut;
}

#[cfg(test)]
pub use game_state::tests::*;
