use async_tungstenite::{tungstenite::Message, WebSocketStream};
use futures::{AsyncRead, AsyncWrite, SinkExt, StreamExt};
use salt_engine::{game_state::PlayerId, id::Id};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait GameMessage: Serialize + DeserializeOwned {}

#[derive(Serialize, Deserialize, Debug)]
pub enum FromClient {
    JoinGame,
    GameId(Id),
}
impl GameMessage for FromClient {}

#[derive(Serialize, Deserialize, Debug)]
pub enum FromServer {
    Hello(PlayerId),
    GameId(Id),
}
impl GameMessage for FromServer {}

pub struct Connection<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    stream: WebSocketStream<S>,
}

impl<S> Connection<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    pub fn new(stream: WebSocketStream<S>) -> Self {
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

    pub async fn recv<'a, T>(&mut self) -> Option<T>
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
