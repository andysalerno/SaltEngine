mod add_card_to_hand_event_handler;
mod attack_event_handler;
mod creature_deals_damage_handler;
mod creature_destroyed_handler;
mod creature_takes_damage_handler;
mod draw_card_event_handler;
mod end_turn_event_handler;
mod summon_event_handler;
mod turn_start_event_handler;

pub use attack_event_handler::AttackEventHandler;
pub use creature_deals_damage_handler::CreatureDealsDamageHandler;
pub use creature_destroyed_handler::CreatureDestroyedEventHandler;
pub use creature_takes_damage_handler::CreatureTakesDamageHandler;
pub use draw_card_event_handler::DrawCardEventHandler;
pub use end_turn_event_handler::EndTurnEventHandler;
pub use summon_event_handler::SummonCreatureEventHandler;
pub use turn_start_event_handler::TurnStartHandler;

use crate::game_state::GameState;

use super::{Event, EventDispatcher};

pub trait EventHandler {
    type Event: Event;
    fn handle(
        &self,
        event: Self::Event,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    );
}
