pub mod client_actions;
pub mod client_event_views;
pub mod entities;
pub mod full_state;

use client_actions::*;
use client_event_views::*;
use serde::{Deserialize, Serialize};

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
pub enum ClientEventView {
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
