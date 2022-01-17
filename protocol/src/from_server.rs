use crate::{
    entities::{Entity, Id, PlayerId},
    visual_events::*,
    GameMessage,
};
use serde::{Deserialize, Serialize};

/// Views of events that the server can send to clients.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VisualEvent {
    CardAddedToHand(CardAddedToHand),
    CreatureSetOnBoard(CreatureSetOnBoard),
    CreatureSummonedFromHand(CreatureSummonedFromHand),
    TurnEnded(TurnEnded),
    TurnStarted(TurnStarted),
    PlayerGainMana(PlayerGainMana),
    PlayerSpendMana(PlayerSpendMana),
    CreatureAttacksTarget(CreatureAttacksTarget),
}

/// A message from server to client that informs of an entity's new value.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntityUpdate {
    pub id: Id,
    pub property_names: Vec<String>,
    pub property_values: Vec<String>,
}

/// A message from server to client that informs of an entity's new value.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntityAdded {
    pub id: Id,
    pub entity: Entity,
}

/// Messages that can be sent from the game server to the game client.
#[derive(Serialize, Deserialize, Debug)]
pub enum FromServer {
    /// Initial message from the server, providing the player with their ID for the match.
    Hello {
        your_id: PlayerId,
        opponent_id: PlayerId,
    },

    /// Sent from the server to indicate the game has started, including the opponent's `PlayerId`.
    GameStart { opponent_id: PlayerId },

    /// Indicates to the client their turn has started.
    TurnStart,

    /// Hints to the client that the server is waiting for their action. Includes the current state.
    //WaitingForAction(GameStatePlayerView),
    WaitingForAction,

    /// Indicates the client should prompt for some value (board slot, card, etc).
    Prompt(PromptMessage),

    /// A message from the server notifying the game client about some event.
    Notification(Notification),
}
impl GameMessage for FromServer {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Notification {
    VisualEvent(VisualEvent),
    EntityUpdate(EntityUpdate),
    EntityAdded(EntityAdded),
}

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
