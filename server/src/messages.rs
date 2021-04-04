use salt_engine::{
    game_logic::ClientGameEvent,
    game_state::{board::BoardPos, GameStatePlayerView, PlayerId},
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
    PromptResponse(BoardPos),
}
impl GameMessage for FromClient {}

#[derive(Serialize, Deserialize, Debug)]
pub enum FromServer {
    Hello(PlayerId),
    GameStart { opponent_id: PlayerId },
    State(GameStatePlayerView),
    TurnStart,
    WaitingForAction(GameStatePlayerView),
    Prompt(PromptMessage, GameStatePlayerView),
}
impl GameMessage for FromServer {}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum PromptMessage {
    PromptSlot,
    PromptPlayerSlot,
    PromptOpponentSlot,
    PromptCreaturePos,
    PromptPlayerCreaturePos,
    PromptOpponentCreaturePos,
}
