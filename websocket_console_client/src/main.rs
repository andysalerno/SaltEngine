use engine::{
    event::{Event, EventHandler, EventType},
    FromClient, GameState,
};
use events::PlayerStartTurnEvent;
use log::info;

mod websocket_receiver;

fn main() {
    init_logger();

    let receiver = websocket_receiver::connect();

    let handlers: Vec<Box<dyn EventHandler>> = Vec::new();

    // First, we expect a Hello message, with our player IDs.
    let (my_id, enemy_id) = match receiver.receive().expect("connection must be open.") {
        engine::FromServer::Event(_) => panic!("Expected Hello."),
        engine::FromServer::Hello(a, b) => (a, b),
    };

    let game_state = GameState::new(my_id, enemy_id);

    info!("I am: {my_id:?}. Enemy is: {enemy_id:?}");

    loop {
        info!("waiting for message...");
        let message = receiver.receive().unwrap();
        info!("received message: {message:?}");

        let event = match message {
            engine::FromServer::Event(e) => e,
            _ => panic!("unexpected FromServer message"),
        };

        if event.event_type() == &PlayerStartTurnEvent::et() {
            let event: PlayerStartTurnEvent = event.unpack();
            if event.player_id() == my_id {
                info!("My turn has started!");

                info!("Sending end turn action...");
                let my_action = player_next_action();
                receiver.send(my_action);
                info!("Sent.");
            } else {
                info!("Enemy turn has started.")
            }
        }
    }
}

fn enemey_take_turn() {}

fn player_next_action() -> FromClient {
    FromClient::EndTurn
}

fn init_logger() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.format_timestamp_millis();
    builder.init();
}
