#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::missing_docs_in_private_items,
    dead_code,
    clippy::module_name_repetitions
)]

mod card;
pub mod deck;
mod dispatcher;
pub mod event;
mod game_client;
mod game_state;
mod hand;
mod player_id;

pub use card::{Card, CardDefinition, CardId};
pub use dispatcher::{ClientChannel, Dispatcher};
// pub use game_client::{ClientChannel, MessageChannel, FromClient, FromServer};
pub use game_client::{FromClient, FromServer, MessageChannel};
pub use game_state::GameState;
pub use player_id::PlayerId;
