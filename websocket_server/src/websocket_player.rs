use std::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

use engine::{ClientChannel, FromClient, FromServer};
use log::info;
use tungstenite::WebSocket;

pub(crate) struct WebSocketPlayer(Mutex<WebSocket<TcpStream>>);

impl WebSocketPlayer {
    pub fn new(ws: WebSocket<TcpStream>) -> Self {
        Self(Mutex::new(ws))
    }
}

impl ClientChannel for WebSocketPlayer {
    fn push_message(&self, message: FromServer) {
        let text = serde_json::to_string(&message).unwrap();
        info!("Sending message to player: {text}");
        let ws_message = tungstenite::Message::Text(text);
        self.0.lock().unwrap().write_message(ws_message).unwrap();
        info!("Sent.");
    }

    fn try_receive_message(&self) -> Option<FromClient> {
        info!("Starting receive from client...");
        let message = self
            .0
            .lock()
            .unwrap()
            .read_message()
            .ok()
            .map(|m| m.into_text().unwrap())
            .map(|m| serde_json::from_str(&m).unwrap());

        info!("Received from client: {message:?}");

        message
    }
}

pub(crate) fn accept_connections() -> (impl ClientChannel, impl ClientChannel) {
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
