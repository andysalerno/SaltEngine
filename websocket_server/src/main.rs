use std::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

use engine::{event::EventHandler, ClientChannel, Dispatcher, FromClient, GameState, PlayerId};
use events::{DrawCardEventHandler, StartGameEvent, StartGameEventHandler};
use log::info;
use tungstenite::WebSocket;

struct WebSocketPlayer(Mutex<WebSocket<TcpStream>>);

impl WebSocketPlayer {
    fn new(ws: WebSocket<TcpStream>) -> Self {
        Self(Mutex::new(ws))
    }
}

impl ClientChannel for WebSocketPlayer {
    fn push_message(&self, message: &engine::event::EventMessage) {
        let text = serde_json::to_string(message).unwrap();
        info!("Sending message to player: {text}");
        let ws_message = tungstenite::Message::Text(text);
        self.0.lock().unwrap().write_message(ws_message).unwrap();
        info!("Sent.");
    }

    fn try_receive_message(&self) -> Option<FromClient> {
        None
    }
}

fn main() {
    init_logger();

    let (player_a, player_b) = accept_connections();

    info!("Both players connected. Starting game.");
    let handlers: Vec<Box<dyn EventHandler>> = vec![
        Box::new(DrawCardEventHandler::new()),
        Box::new(StartGameEventHandler::new()),
    ];
    let player_a = Box::new(player_a);
    let player_b = Box::new(player_b);
    let dispatcher = Dispatcher::new(handlers, player_a, player_b);

    let event = StartGameEvent::new();

    let mut game_state = GameState::new(PlayerId::new(), PlayerId::new());

    dispatcher.dispatch(&event.into(), &mut game_state);
}

fn accept_connections() -> (impl ClientChannel, impl ClientChannel) {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();

    info!("Waiting for first player to connect...");
    let player_a = tungstenite::accept(server.accept().unwrap().0).unwrap();

    info!("Waiting for second player to connect...");
    let player_b = tungstenite::accept(server.accept().unwrap().0).unwrap();

    (
        WebSocketPlayer::new(player_a),
        WebSocketPlayer::new(player_b),
    )
}

fn init_logger() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.format_timestamp_millis();
    builder.init();
}
