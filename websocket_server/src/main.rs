#![deny(clippy::all, nonstandard_style, future_incompatible)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::needless_pass_by_value,
    clippy::unused_self,
    clippy::cast_lossless
)]
use env_logger::Env;

pub mod connection;
mod matchmaker;
mod network_client_notifier;
mod network_game_client;
mod network_prompter;
mod play_game;
pub mod websocket_server;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    websocket_server::run().expect("server execution failed");
}
