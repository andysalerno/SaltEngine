use super::game_event::{AttackEvent, GameEvent};
use crate::game_state::GameState;

pub trait EventHandler<T: GameEvent> {
    fn ApplyEvent(event: T, state: GameState);
}
