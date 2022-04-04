// pub mod board;
// mod card_instance;
// mod deck;
mod creature_definition;
mod creature_instance;
mod game_state;
// mod hand;
// pub mod hero;
// pub mod selector;

// pub use card_instance::{
//     InstanceState, UnitCardInstance, UnitCardInstancePlayerView, UnitCardInstanceView,
// };
// pub use deck::Deck;
// pub use game_state::{player_view::GameStatePlayerView, GameState, GameStateView};
// pub use hand::{Hand, HandView};
// use protocol::entities::{IsEntity, PlayerId};
// pub use selector::iter_helpers::{IterAddons, IteratorAny};
// use serde::{de::DeserializeOwned, Deserialize, Serialize};

// #[derive(Debug, Eq, Hash, Clone, Copy, PartialEq, Serialize, Deserialize)]
// pub struct PlayerId(Id);

// impl PlayerId {
//     #[must_use]
//     pub fn new() -> Self {
//         Self(Id::new())
//     }
// }

// impl Default for PlayerId {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// / A marker trait, indicating that the implementor
// / is a player-friendly "view" of the associated type.
// trait PlayerView {
//     type TViewOf;
// }

// / Similar in concept to Clone,
// / but specifically with the intent of creating
// / a copy of T that is intended for viewing in a player's
// / client -- i.e., only containing info visible to the player,
// / and not info invisible to them (such as the content of the opponent's hand).
// pub trait MakePlayerView<'a, 'b>
// pub trait MakePlayerView<'a> {
//     type TOut: Serialize + DeserializeOwned + Sized + 'static;
//     fn player_view(&'a self, player_viewing: PlayerId) -> <Self as MakePlayerView<'a>>::TOut;
// }

// pub trait MakePlayerViewNew<'a> {
//     type TOut: IsEntity;
//     fn player_view_new(&'a self, player_viewing: PlayerId)
//         -> <Self as MakePlayerViewNew<'a>>::TOut;
// }

// #[cfg(test)]
// pub use game_state::tests::*;
