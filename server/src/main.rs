mod matchmaking_queue;
pub mod messages;
mod websocker_server;

fn main() {
    websocker_server::run().expect("server execution failed");
}
