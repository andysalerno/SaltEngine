use engine::{event::EventHandler, Dispatcher, GameState, PlayerId};
use events::{DrawCardEventHandler, StartGameEvent, StartGameEventHandler};
use log::info;

mod websocket_player;

fn main() {
    init_logger();

    let (player_a, player_b) = websocket_player::accept_connections();

    info!("Both players connected. Starting game.");
    let dispatcher = {
        let handlers: Vec<Box<dyn EventHandler>> = vec![
            Box::new(DrawCardEventHandler::new()),
            Box::new(StartGameEventHandler::new()),
        ];
        let player_a = Box::new(player_a);
        let player_b = Box::new(player_b);
        Dispatcher::new(handlers, player_a, player_b)
    };

    let event = StartGameEvent::new();

    let mut game_state = GameState::new(PlayerId::new(), PlayerId::new());

    dispatcher.dispatch(&event.into(), &mut game_state);
}

fn init_logger() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.format_timestamp_millis();
    builder.init();
}
