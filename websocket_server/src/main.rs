use std::net::TcpListener;

use engine::{event::EventHandler, ClientChannel, Dispatcher, FromClient, GameState, PlayerId};
use events::{DrawCardEventHandler, StartGameEvent, StartGameEventHandler};
use log::info;

struct WebSocketPlayer;

impl ClientChannel for WebSocketPlayer {
    fn push_message(&self, _message: &engine::event::EventMessage) {
        //
    }

    fn try_receive_message(&self) -> Option<FromClient> {
        None
    }
}

fn main() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.format_timestamp_millis();
    builder.init();

    accept_connections();

    let handlers: Vec<Box<dyn EventHandler>> = vec![
        Box::new(DrawCardEventHandler::new()),
        Box::new(StartGameEventHandler::new()),
    ];
    let player_a = Box::new(WebSocketPlayer);
    let player_b = Box::new(WebSocketPlayer);
    let dispatcher = Dispatcher::new(handlers, player_a, player_b);

    let event = StartGameEvent::new();

    let mut game_state = GameState::new(PlayerId::new(), PlayerId::new());

    dispatcher.dispatch(&event.into(), &mut game_state);
}

fn accept_connections() {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();

    info!("Waiting for first player to connect...");
    let player_a = tungstenite::accept(server.accept().unwrap().0);

    info!("Waiting for second player to connect...");
    let player_b = tungstenite::accept(server.accept().unwrap().0);

    // for stream in server.incoming() {
    //     std::thread::spawn(move || {
    //         let mut websocket = tungstenite::accept(stream.unwrap()).unwrap();
    //         loop {
    //             let msg = websocket.read_message().unwrap();

    //             // We do not want to send back ping/pong messages.
    //             if msg.is_binary() || msg.is_text() {
    //                 websocket.write_message(msg).unwrap();
    //             }
    //         }
    //     });
    // }
}
