mod connection;
mod matchmaker;
pub mod messages;
mod play_game;
mod websocket_server;

fn main() {
    websocket_server::run().expect("server execution failed");
}
