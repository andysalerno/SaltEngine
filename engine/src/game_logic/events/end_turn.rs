use super::{Event, GameEvent};

#[derive(Debug)]
pub struct EndTurnEvent;

impl Event for EndTurnEvent {}

impl Into<GameEvent> for EndTurnEvent {
    fn into(self) -> GameEvent {
        GameEvent::EndTurn(self)
    }
}
