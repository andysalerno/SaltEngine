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
use protocol::{entities::PlayerId, from_client::ClientAction, from_server::VisualEvent};
pub use start_game_event::StartGameEvent;
pub use summon_creature_from_hand_event::{
    CreatureSummonedFromHandEvent, SummonCreatureFromHandClientEvent,
};
pub use turn_start_event::TurnStartEvent;

use crate::game_state::{GameState, GameStateView};
use enum_dispatch::enum_dispatch;

pub type Result = std::result::Result<(), Box<dyn std::error::Error>>;

/// A marker trait defining an event in the game.
#[enum_dispatch(GameEvent)]
pub trait Event: Debug {
    /// Returns a result indicating if the game event is valid given the current game state.
    fn validate<'a, G>(&self, _game_state: &'a G) -> Result
    where
        G: GameStateView<'a>,
    {
        Ok(())
    }

    /// Returns a `ClientEventView` representation of this event, or `None` if there isn't one.
    fn maybe_client_event(
        &self,
        _player_id: PlayerId,
        _game_state: &GameState,
    ) -> Option<VisualEvent> {
        None
    }
}

/// All possible game events.
/// This is an enum-dispatched implentation of the trait `Event`.
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
    CreatureSummonedFromHandEvent,
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
            Self::CreatureSummonedFromHandEvent(e) => e.fmt(f),
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

/// This implementation is responsible for converting the client-provided actions
/// to the `GameEvent`s that the engine will execute.
impl From<ClientAction> for GameEvent {
    fn from(e: ClientAction) -> Self {
        match e {
            ClientAction::EndTurn(e) => EndTurnEvent(e.player_id).into(),
            ClientAction::SummonCreatureFromHand(e) => {
                CreatureSummonedFromHandEvent::new(e.player_id, e.board_pos, e.card_id).into()
            }
            ClientAction::Attack(e) => AttackEvent::new(e.attacker, e.target).into(),
            // ClientAction::DrawCard(e) => e.into(),
        }
    }
}
