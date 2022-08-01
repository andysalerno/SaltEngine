use engine::GameState;
use log::info;

mod websocket_receiver;

fn main() {
    init_logger();

    let receiver = websocket_receiver::connect();

    // First, we expect a Hello message, with our player IDs.
    let (my_id, enemy_id) = match receiver.receive().expect("connection must be open.") {
        engine::FromServer::Event(_) => panic!("Expected Hello."),
        engine::FromServer::Hello(a, b) => (a, b),
    };

    let game_state = GameState::new(my_id, enemy_id);

    info!("I am: {my_id:?}. Enemy is: {enemy_id:?}");

    loop {
        info!("waiting for message...");
        let message = receiver.receive();
        info!("received message: {message:?}");
    }
}

fn init_logger() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.format_timestamp_millis();
    builder.init();
}
