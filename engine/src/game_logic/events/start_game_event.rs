use super::{Event, GameEvent};

#[derive(Debug, Clone)]
pub struct StartGameEvent;

impl Event for StartGameEvent {}

// impl From<StartGameEvent> for GameEvent {
//     fn from(val: StartGameEvent) -> Self {
//         GameEvent::StartGame(val)
//     }
// }
