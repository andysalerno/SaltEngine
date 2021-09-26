use std::sync::Arc;

use async_tungstenite::{tungstenite::Message, WebSocketStream};
use futures::{SinkExt, StreamExt};
use log::debug;
use serde::de::DeserializeOwned;
use smol::{lock::Mutex, net::TcpStream};

use crate::messages::GameMessage;
use crate::Result;

/// A connection to a player.
#[derive(Debug, Clone)]
pub struct Connection {
    stream: Arc<Mutex<WebSocketStream<TcpStream>>>,
}

impl Connection {
    #[must_use]
    pub fn new(stream: WebSocketStream<TcpStream>) -> Self {
        Self {
            stream: Arc::new(Mutex::new(stream)),
        }
    }

    pub async fn send<M>(&self, message: M) -> Result<()>
    where
        M: GameMessage,
    {
        let json = serde_json::to_string(&message)?;
        debug!("Sending raw json: {}", json);
        self.stream.lock().await.send(Message::Text(json)).await?;
        Ok(())
    }

    pub async fn recv<T>(&self) -> Option<T>
    where
        T: GameMessage + DeserializeOwned,
    {
        let response = self
            .stream
            .lock()
            .await
            .next()
            .await
            .expect("Connection died")
            .ok()?;

        let s = response
            .to_text()
            .expect("Expected a websocket text message");

        debug!("Received raw json response: {}", s);

        let t: T = serde_json::from_str(s).expect("failed to deserialize");

        Some(t)
    }
}
