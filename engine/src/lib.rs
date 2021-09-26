#![deny(clippy::all, nonstandard_style, future_incompatible)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::needless_pass_by_value,
    clippy::module_name_repetitions,
    clippy::unused_self,
    clippy::cast_lossless,
    clippy::module_inception,
    clippy::missing_panics_doc,
    clippy::similar_names,
    dead_code
)]

pub mod game_agent;
pub mod game_logic;
pub mod game_runner;
pub mod game_state;
pub mod id;

pub use game_logic::cards;
