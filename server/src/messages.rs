use salt_engine::{
    game_logic::ClientGameEvent,
    game_state::{GameStatePlayerView, PlayerId},
    id::Id,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// A marker trait to indicate a type is a GameMessage.
pub trait GameMessage: Serialize + DeserializeOwned {}

#[derive(Serialize, Deserialize, Debug)]
pub enum FromClient {
    JoinGame,
    Ready,
    GameId(Id),
    ClientAction(ClientGameEvent),
}
impl GameMessage for FromClient {}

#[derive(Serialize, Deserialize, Debug)]
pub enum FromServer {
    Hello(PlayerId),
    GameStart { opponent_id: PlayerId },
    State(GameStatePlayerView),
    TurnStart,
    WaitingForAction(GameStatePlayerView),
}
impl GameMessage for FromServer {}
