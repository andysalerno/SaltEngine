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

pub use add_card_to_hand_event::AddCardToHandEvent;
pub use attack::AttackEvent;
pub use creature_deals_damage_event::CreatureDealsDamageEvent;
pub use creature_destroyed::CreatureDestroyedEvent;
pub use creature_healed_event::CreatureHealedEvent;
pub use creature_set_event::CreatureSetEvent;
pub use creature_takes_damage_event::CreatureTakesDamageEvent;
pub use draw_card::DrawCardEvent;
pub use end_turn::EndTurnEvent;
pub use player_gain_mana::PlayerGainManaEvent;
pub use player_spend_mana::PlayerSpendManaEvent;
pub use pos_takes_damage_event::PosTakesDamageEvent;
use serde::{Deserialize, Serialize};
pub use start_game_event::StartGameEvent;
pub use summon_creature_from_hand_event::SummonCreatureFromHandEvent;
pub use turn_start_event::TurnStartEvent;

use crate::game_state::GameStateView;

use self::add_card_to_hand_event::AddCardToHandClientEvent;

pub type Result = std::result::Result<(), Box<dyn std::error::Error>>;

pub trait Event: Into<GameEvent> {
    fn validate<'a, G>(&self, _game_state: &'a G) -> Result
    where
        G: GameStateView<'a>,
    {
        Ok(())
    }

    fn maybe_client_event(&self) -> Option<ClientEventView> {
        None
    }
}

/// All possible game events.
#[derive(Debug)]
pub enum GameEvent {
    Attack(AttackEvent),
    EndTurn(EndTurnEvent),
    Summon(CreatureSetEvent),
    CreatureDealsDamage(CreatureDealsDamageEvent),
    CreatureTakesDamage(CreatureTakesDamageEvent),
    CreatureDestroyed(CreatureDestroyedEvent),
    TurnStart(TurnStartEvent),
    DrawCard(DrawCardEvent),
    AddCardToHand(AddCardToHandEvent),
    StartGame(StartGameEvent),
    GainMana(PlayerGainManaEvent),
    SpendMana(PlayerSpendManaEvent),
    SummonCreatureFromHand(SummonCreatureFromHandEvent),
    PosTakesDamage(PosTakesDamageEvent),
    CreatureHealed(CreatureHealedEvent),
}

impl GameEvent {
    pub fn is_end_turn(&self) -> bool {
        matches!(self, GameEvent::EndTurn(_))
    }

    pub fn maybe_client_event(&self) -> Option<ClientEventView> {
        match self {
            GameEvent::AddCardToHand(e) => e.maybe_client_event(),
            _ => None,
        }
    }
}

/// The subset of game events that clients can
/// provide the server over the course of the game.
/// For example, a client can legally provide a TurnEnd event
/// (they are allowed to end their own turn), but a client cannot
/// directly provide a CreatureDestroyed event.
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
    pub fn is_end_turn(&self) -> bool {
        matches!(self, ClientActionEvent::EndTurn(_))
    }
}

impl Event for GameEvent {
    fn validate<'a, G>(&self, game_state: &'a G) -> Result
    where
        G: GameStateView<'a>,
    {
        match self {
            GameEvent::Attack(e) => e.validate(game_state),
            GameEvent::EndTurn(e) => e.validate(game_state),
            GameEvent::Summon(e) => e.validate(game_state),
            GameEvent::CreatureDealsDamage(e) => e.validate(game_state),
            GameEvent::CreatureTakesDamage(e) => e.validate(game_state),
            GameEvent::CreatureDestroyed(e) => e.validate(game_state),
            GameEvent::TurnStart(e) => e.validate(game_state),
            GameEvent::DrawCard(e) => e.validate(game_state),
            GameEvent::AddCardToHand(e) => e.validate(game_state),
            GameEvent::StartGame(e) => e.validate(game_state),
            GameEvent::GainMana(e) => e.validate(game_state),
            GameEvent::SpendMana(e) => e.validate(game_state),
            GameEvent::SummonCreatureFromHand(e) => e.validate(game_state),
            GameEvent::PosTakesDamage(e) => e.validate(game_state),
            GameEvent::CreatureHealed(e) => e.validate(game_state),
        }
    }
}
