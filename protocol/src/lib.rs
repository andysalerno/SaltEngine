pub mod client_actions;
pub mod entities;
pub mod full_state;
pub mod visual_events;

use client_actions::*;
use serde::{Deserialize, Serialize};
use visual_events::*;

/// Actions a client can send to the server for execution.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ClientAction {
    EndTurn(EndTurn),
    SummonCreatureFromHand(SummonCreatureFromHand),
    Attack(Attack),
    // DrawCard(DrawCardEvent), "draw card" is not an action a client can decide to do, it just happens
}

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
}

impl ClientAction {
    #[must_use]
    pub fn is_end_turn(&self) -> bool {
        matches!(self, ClientAction::EndTurn(_))
    }
}
