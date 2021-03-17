use async_tungstenite::tungstenite::Message;
use salt_engine::{game_state::PlayerId, id::Id};
use serde::{Deserialize, Serialize};

pub trait GameMessage {}

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

pub trait IntoJson {
    fn json(self) -> String;
}

impl<T> IntoJson for T
where
    T: GameMessage + Serialize,
{
    fn json(self) -> String {
        serde_json::to_string(&self).expect("failed to serialize")
    }
}

pub trait FromJson<T> {
    fn from_json(self) -> T;
}

impl<'a, T> FromJson<T> for &'a str
where
    T: GameMessage + Deserialize<'a>,
{
    fn from_json(self) -> T {
        serde_json::from_str(self).expect("failed to deserialize")
    }
}

impl<'a, T> FromJson<T> for &'a Message
where
    T: GameMessage + Deserialize<'a>,
{
    fn from_json(self) -> T {
        let s = self.to_text().expect("Expected a websocket text message");
        serde_json::from_str(s).expect("failed to deserialize")
    }
}
