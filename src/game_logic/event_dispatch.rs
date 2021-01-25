use crate::game_state::GameState;

use super::{
    event::EventHandler,
    event_handlers::{AttackEventHandler, EndTurnEventHandler},
    events::{AttackEvent, EndTurnEvent},
    is::Downcast,
    Event,
};

pub struct EventDispatcher;

impl EventDispatcher {
    pub fn dispatch(event: Box<dyn Event>, game_state: &mut GameState) {
        let any = event.as_any();
        if let Some(event) = any.downcast_ref::<AttackEvent>() {
            AttackEventHandler::default().handle(event, game_state);
        } else if let Some(event) = any.downcast_ref::<EndTurnEvent>() {
            EndTurnEventHandler::default().handle(event, game_state);
        }
    }
}
