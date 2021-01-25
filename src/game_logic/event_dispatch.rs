use crate::game_state::GameState;

use super::{
    event_handlers::{AttackEventHandler, EndTurnEventHandler, EventHandler},
    events::GameEvent,
};

/// Has the ugly, unforgiving job of detecting the concrete type of a `dyn Event`
/// and dispatching it to the proper handler.
pub struct EventDispatcher;

impl EventDispatcher {
    pub fn dispatch(event: GameEvent, game_state: &mut GameState) {
        match event {
            GameEvent::Attack(e) => AttackEventHandler::default().handle(&e, game_state),
            GameEvent::EndTurn(e) => EndTurnEventHandler::default().handle(&e, game_state),
        }
    }
}
