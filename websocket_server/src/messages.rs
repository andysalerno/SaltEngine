use protocol::{
    entities::{BoardPos, Id, PlayerId, UnitCardInstanceId},
    ClientAction, VisualEvent,
};
use salt_engine::game_state::GameStatePlayerView;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// A marker trait to indicate a type is a `GameMessage`.
pub trait GameMessage: Serialize + DeserializeOwned {}

/// Messages that can be sent from the game client to the game server.
#[derive(Serialize, Deserialize, Debug)]
pub enum FromClient {
    JoinGame,
    Ready,
    GameId(Id),
    ClientAction(ClientAction),
    PromptResponse(BoardPos),
}
impl GameMessage for FromClient {}

/// Messages that can be sent from the game server to the game client.
#[derive(Serialize, Deserialize, Debug)]
pub enum FromServer {
    /// Initial message from the server, providing the player with their ID for the match.
    Hello(PlayerId),

    /// Sent from the server to indicate the game has started, including the opponent's `PlayerId`.
    GameStart { opponent_id: PlayerId },

    /// A newly-updated gamestate after some action has occurred.
    State(GameStatePlayerView),

    /// Indicates to the client their turn has started.
    TurnStart,

    /// Indicates the player has drawn a card from their deck. Includes the card instance ID and the updated state that reflects the card draw.
    PlayerCardDrawnFromDeck(UnitCardInstanceId, GameStatePlayerView),

    /// Indicates the opponent has drawn a card from their own deck. Includes the updated game state that reflects the card draw.
    OpponentCardDrawnFromDeck(GameStatePlayerView),

    /// Hints to the client that the server is waiting for their action. Includes the current state.
    WaitingForAction(GameStatePlayerView),

    /// Indicates the client should prompt for some value (board slot, card, etc).
    Prompt(PromptMessage, GameStatePlayerView),

    /// A message from the server notifying the game client about some event.
    NotifyEvent(VisualEvent),
}
impl GameMessage for FromServer {}

/// A message requesting the client to prompt for a certain value and provide its result.
#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum PromptMessage {
    PromptSlot,
    PromptPlayerSlot,
    PromptOpponentSlot,
    PromptCreaturePos,
    PromptPlayerCreaturePos,
    PromptOpponentCreaturePos,
}
