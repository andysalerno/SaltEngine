mod add_buff_to_card_instance_event;
mod add_card_to_hand_event;
mod attack;
mod creature_deals_damage_event;
mod creature_destroyed;
mod creature_healed_event;
mod creature_set_event;
mod creature_takes_damage_event;
mod draw_card;
mod end_turn;
mod player_gain_mana;
mod player_spend_mana;
mod pos_takes_damage_event;
mod start_game_event;
mod summon_creature_from_hand_event;
mod turn_start_event;

use std::fmt::Debug;

pub use add_buff_to_card_instance_event::AddBuffToCardInstanceEvent;
pub use add_card_to_hand_event::{AddCardToHandClientEvent, AddCardToHandEvent};
pub use attack::AttackEvent;
pub use creature_deals_damage_event::CreatureDealsDamageEvent;
pub use creature_destroyed::CreatureDestroyedEvent;
pub use creature_healed_event::CreatureHealedEvent;
pub use creature_set_event::*;
pub use creature_takes_damage_event::CreatureTakesDamageEvent;
pub use draw_card::DrawCardEvent;
pub use end_turn::EndTurnEvent;
pub use player_gain_mana::PlayerGainManaEvent;
pub use player_spend_mana::PlayerSpendManaEvent;
pub use pos_takes_damage_event::PosTakesDamageEvent;
use serde::{Deserialize, Serialize};
pub use start_game_event::StartGameEvent;
pub use summon_creature_from_hand_event::{
    SummonCreatureFromHandClientEvent, SummonCreatureFromHandEvent,
};
pub use turn_start_event::TurnStartEvent;

use crate::game_state::{GameState, GameStateView, PlayerId};
use enum_dispatch::enum_dispatch;

pub type Result = std::result::Result<(), Box<dyn std::error::Error>>;

#[enum_dispatch(GameEvent)]
pub trait Event: Debug {
    fn validate<'a, G>(&self, _game_state: &'a G) -> Result
    where
        G: GameStateView<'a>,
    {
        Ok(())
    }

    fn maybe_client_event(&self, _game_state: &GameState) -> Option<ClientEventView> {
        None
    }
}

/// All possible game events.
#[enum_dispatch]
pub enum GameEvent {
    AttackEvent,
    EndTurnEvent,
    CreatureSetEvent,
    AddBuffToCardInstanceEvent,
    CreatureDealsDamageEvent,
    CreatureTakesDamageEvent,
    CreatureDestroyedEvent,
    TurnStartEvent,
    DrawCardEvent,
    AddCardToHandEvent,
    StartGameEvent,
    PlayerGainManaEvent,
    PlayerSpendManaEvent,
    SummonCreatureFromHandEvent,
    PosTakesDamageEvent,
    CreatureHealedEvent,
}

impl Debug for GameEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AttackEvent(e) => e.fmt(f),
            Self::EndTurnEvent(e) => e.fmt(f),
            Self::CreatureSetEvent(e) => e.fmt(f),
            Self::AddBuffToCardInstanceEvent(e) => e.fmt(f),
            Self::CreatureDealsDamageEvent(e) => e.fmt(f),
            Self::CreatureTakesDamageEvent(e) => e.fmt(f),
            Self::CreatureDestroyedEvent(e) => e.fmt(f),
            Self::TurnStartEvent(e) => e.fmt(f),
            Self::DrawCardEvent(e) => e.fmt(f),
            Self::AddCardToHandEvent(e) => e.fmt(f),
            Self::StartGameEvent(e) => e.fmt(f),
            Self::PlayerGainManaEvent(e) => e.fmt(f),
            Self::PlayerSpendManaEvent(e) => e.fmt(f),
            Self::SummonCreatureFromHandEvent(e) => e.fmt(f),
            Self::PosTakesDamageEvent(e) => e.fmt(f),
            Self::CreatureHealedEvent(e) => e.fmt(f),
        }
    }
}

impl GameEvent {
    #[must_use]
    pub fn is_end_turn(&self) -> bool {
        matches!(self, GameEvent::EndTurnEvent(_))
    }
}

/// The subset of game events that clients can
/// provide the server over the course of the game.
/// For example, a client can legally provide a `TurnEnd` event
/// (they are allowed to end their own turn), but a client cannot
/// directly provide a `CreatureDestroyed` event.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ClientActionEvent {
    EndTurn(EndTurnEvent),
    SummonCreatureFromHand(SummonCreatureFromHandEvent),
    Attack(AttackEvent),
    DrawCard(DrawCardEvent),
}

/// Views of events that can be sent to clients.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ClientEventView {
    AddCardToHand(AddCardToHandClientEvent),
    UnitSet(CreatureSetClientEvent),
    SummonCreatureFromHand(SummonCreatureFromHandClientEvent),
    TurnEnded(PlayerId),
    TurnStarted(PlayerId),
    PlayerGainMana(PlayerId, usize),
    PlayerSpendMana {
        player_id: PlayerId,
        spent_mana_count: usize,
    },
}

impl From<ClientActionEvent> for GameEvent {
    fn from(e: ClientActionEvent) -> Self {
        match e {
            ClientActionEvent::EndTurn(e) => e.into(),
            ClientActionEvent::SummonCreatureFromHand(e) => e.into(),
            ClientActionEvent::Attack(e) => e.into(),
            ClientActionEvent::DrawCard(e) => e.into(),
        }
    }
}

impl ClientActionEvent {
    #[must_use]
    pub fn is_end_turn(&self) -> bool {
        matches!(self, ClientActionEvent::EndTurn(_))
    }
}
