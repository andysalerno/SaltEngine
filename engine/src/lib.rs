#![deny(clippy::all, nonstandard_style, future_incompatible)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::needless_pass_by_value,
    clippy::module_name_repetitions,
    clippy::unused_self,
    clippy::cast_lossless,
    clippy::module_inception
)]

pub mod game_agent;
pub mod game_logic;
pub mod game_runner;
pub mod game_state;
pub mod id;

pub use game_logic::cards;
