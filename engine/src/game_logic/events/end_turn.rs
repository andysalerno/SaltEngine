use super::{Event, GameEvent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EndTurnEvent;

impl Event for EndTurnEvent {}

impl Into<GameEvent> for EndTurnEvent {
    fn into(self) -> GameEvent {
        GameEvent::EndTurn(self)
    }
}
