use super::{Event, GameEvent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EndTurnEvent;

impl Event for EndTurnEvent {}

impl From<EndTurnEvent> for GameEvent {
    fn from(val: EndTurnEvent) -> Self {
        GameEvent::EndTurn(val)
    }
}
