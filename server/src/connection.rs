use async_tungstenite::{tungstenite::Message, WebSocketStream};
use futures::{SinkExt, StreamExt};
use serde::de::DeserializeOwned;
use smol::net::TcpStream;

use crate::messages::GameMessage;
use crate::Result;

/// A connection to a player.
#[derive(Debug)]
pub struct Connection {
    stream: WebSocketStream<TcpStream>,
}

// pub enum ConnectionError {
//     SendFailure(String),
// }

// impl From<(dyn std::error::Error + 'static)> for ConnectionError {
//     fn from(_: Box<dyn std::error::Error>) -> Self {
//         todo!()
//     }
// }

// impl<T> From<T> for ConnectionError
// where
//     T: std::error::Error,
// {
//     fn from(_: T) -> Self {
//         todo!()
//     }
// }

impl Connection {
    pub fn new(stream: WebSocketStream<TcpStream>) -> Self {
        Self { stream }
    }

    pub async fn send<M>(&mut self, message: M) -> Result<()>
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
