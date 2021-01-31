mod attack_event_handler;
mod end_turn_event_handler;
mod summon_event_handler;

pub use attack_event_handler::AttackEventHandler;
pub use end_turn_event_handler::EndTurnEventHandler;
pub use summon_event_handler::SummonCreatureEventHandler;

use crate::game_state::GameState;

use super::Event;

pub trait EventHandler {
    type Event: Event;
    fn handle(&self, event: Self::Event, game_state: &mut GameState);
}
