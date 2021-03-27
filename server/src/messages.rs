use async_tungstenite::{tungstenite::Message, WebSocketStream};
use futures::{AsyncRead, AsyncWrite, SinkExt, StreamExt};
use salt_engine::{
    game_state::{GameStatePlayerView, PlayerId},
    id::Id,
};
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
    Session(GameSession),
    State(GameStatePlayerView),
}
impl GameMessage for FromServer {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSession {
    pub session_id: Id,
    pub player_a_id: PlayerId,
    pub player_b_id: PlayerId,
}
