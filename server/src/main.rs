mod connection;
mod matchmaker;
pub mod messages;
mod play_game;
mod websocket_server;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() {
    websocket_server::run().expect("server execution failed");
}
