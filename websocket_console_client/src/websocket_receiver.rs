use std::{net::TcpStream, sync::Mutex};

use engine::{event::EventMessage, FromServer};
use log::info;
use tungstenite::{stream::MaybeTlsStream, WebSocket};

pub(crate) struct WebSocketReceiver(Mutex<WebSocket<MaybeTlsStream<TcpStream>>>);

impl WebSocketReceiver {
    pub fn new(ws: WebSocket<MaybeTlsStream<TcpStream>>) -> Self {
        Self(Mutex::new(ws))
    }

    pub fn receive(&self) -> Option<FromServer> {
        let message = self.0.lock().unwrap().read_message().ok()?;

        let text = match message {
            tungstenite::Message::Text(s) => s,
            _ => panic!("unknown message type"),
        };

        let from_server: FromServer = serde_json::from_str(&text).ok()?;

        Some(from_server)
    }
}

pub(crate) fn connect() -> WebSocketReceiver {
    info!("Connecting...");
    let mut socket = loop {
        match tungstenite::connect("ws://localhost:9001") {
            Ok((socket, _)) => break socket,
            _ => continue,
        }
    };
    info!("Connected.");

    let receiver = WebSocketReceiver::new(socket);

    receiver
}
