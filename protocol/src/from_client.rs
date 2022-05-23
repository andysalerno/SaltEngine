use crate::{
    client_actions::{Attack, EndTurn, SummonCreatureFromHand},
    entities::BoardPos,
    GameMessage,
};
use id::Id;
use serde::{Deserialize, Serialize};

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

/// Actions a client can send to the server for execution.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ClientAction {
    EndTurn(EndTurn),
    SummonCreatureFromHand(SummonCreatureFromHand),
    Attack(Attack),
    // DrawCard(DrawCardEvent), "draw card" is not an action a client can decide to do, it just happens
}

impl ClientAction {
    #[must_use]
    pub fn is_end_turn(&self) -> bool {
        matches!(self, ClientAction::EndTurn(_))
    }
}
