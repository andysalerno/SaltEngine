use env_logger::Env;

mod connection;
mod matchmaker;
pub mod messages;
mod network_prompter;
mod play_game;
mod websocket_server;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    websocket_server::run().expect("server execution failed");
}
