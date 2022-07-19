#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::missing_docs_in_private_items,
    dead_code,
    clippy::module_name_repetitions
)]

mod card;
mod deck;
mod dispatcher;
pub mod event;
mod game_state;
mod hand;
mod player_id;

pub use card::{Card, CardDefinition};
pub use dispatcher::Dispatcher;
pub use game_state::GameState;
pub use player_id::PlayerId;
