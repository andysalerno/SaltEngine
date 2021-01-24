use super::game_event::{AttackEvent, GameEvent};
use crate::game_state::GameState;

pub trait EventHandler<T: GameEvent> {
    fn apply_event(event: T, state: GameState);
}

struct EventDispatcher;

impl EventDispatcher {
    fn dispatch(event: Box<dyn GameEvent>) {}
}
