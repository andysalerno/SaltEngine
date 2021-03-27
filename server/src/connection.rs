use async_tungstenite::{tungstenite::Message, WebSocketStream};
use futures::{SinkExt, StreamExt};
use serde::de::DeserializeOwned;
use smol::net::TcpStream;

use crate::messages::GameMessage;

/// A connection to a player.
#[derive(Debug)]
pub struct Connection {
    stream: WebSocketStream<TcpStream>,
}

impl Connection {
    pub fn new(stream: WebSocketStream<TcpStream>) -> Self {
        Self { stream }
    }

    pub async fn send<M>(&mut self, message: M) -> Result<(), Box<dyn std::error::Error>>
    where
        M: GameMessage,
    {
        let json = serde_json::to_string(&message)?;
        self.stream.send(Message::Text(json)).await?;
        Ok(())
    }

    pub async fn recv<T>(&mut self) -> Option<T>
    where
        T: GameMessage + DeserializeOwned,
    {
        let response = self.stream.next().await.expect("Connection died").ok()?;

        let s = response
            .to_text()
            .expect("Expected a websocket text message");

        let t: T = serde_json::from_str(s).expect("failed to deserialize");

        Some(t)
    }
}
