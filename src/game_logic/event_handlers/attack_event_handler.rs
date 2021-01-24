use super::super::event_handler::EventHandler;
use super::super::game_event::AttackEvent;
use crate::game_state::GameState;

pub struct AttackEventHandler;

impl EventHandler<AttackEvent> for AttackEventHandler {
    fn apply_event(event: AttackEvent, game_state: GameState) {
        todo!()
    }
}
