use super::{Event, GameEvent};

#[derive(Debug)]
pub struct TurnStartEvent;

impl Event for TurnStartEvent {}

impl Into<GameEvent> for TurnStartEvent {
    fn into(self) -> GameEvent {
        GameEvent::TurnStart(self)
    }
}
