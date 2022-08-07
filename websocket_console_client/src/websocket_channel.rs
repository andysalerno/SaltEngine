use engine::{FromClient, FromServer, MessageChannel};
use log::info;
use std::{net::TcpStream, sync::Mutex};
use tungstenite::{stream::MaybeTlsStream, WebSocket};

struct WebSocketChannel(Mutex<WebSocket<MaybeTlsStream<TcpStream>>>);

impl MessageChannel for WebSocketChannel {
    type Receive = FromServer;
    type Send = FromClient;

    fn send(&self, from_client: FromClient) {
        let message = tungstenite::Message::Text(serde_json::to_string(&from_client).unwrap());
        self.0.lock().unwrap().write_message(message).unwrap();
    }

    fn try_receive(&self) -> Option<Self::Receive> {
        let message = self.0.lock().unwrap().read_message().ok()?;

        let text = match message {
            tungstenite::Message::Text(s) => s,
            _ => panic!("unknown message type"),
        };

        let from_server: FromServer = serde_json::from_str(&text).ok()?;

        Some(from_server)
    }
}

impl WebSocketChannel {
    pub fn new(ws: WebSocket<MaybeTlsStream<TcpStream>>) -> Self {
        Self(Mutex::new(ws))
    }
}

pub(crate) fn connect() -> impl MessageChannel<Send = FromClient, Receive = FromServer> {
    info!("Connecting...");
    let socket = loop {
        match tungstenite::connect("ws://localhost:9001") {
            Ok((socket, _)) => break socket,
            _ => continue,
        }
    };
    info!("Connected.");

    WebSocketChannel::new(socket)
}
