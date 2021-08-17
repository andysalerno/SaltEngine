use super::{Event, GameEvent};

#[derive(Debug, Clone)]
pub struct TurnStartEvent;

impl Event for TurnStartEvent {}

impl From<TurnStartEvent> for GameEvent {
    fn from(val: TurnStartEvent) -> Self {
        GameEvent::TurnStart(val)
    }
}
