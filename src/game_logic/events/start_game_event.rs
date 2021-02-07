use super::{Event, GameEvent};

#[derive(Debug)]
pub struct StartGameEvent;

impl Event for StartGameEvent {}

impl Into<GameEvent> for StartGameEvent {
    fn into(self) -> GameEvent {
        GameEvent::StartGame(self)
    }
}
