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
