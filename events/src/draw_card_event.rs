use engine::event::{Event, EventHandler, EventMessage, EventType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DrawCardEvent {}

impl Event for DrawCardEvent {
    fn event_type(&self) -> EventType {
        EventType::new("DrawCardEventHandler")
    }
}

pub struct DrawCardEventHandler {}

impl EventHandler for DrawCardEventHandler {
    fn event_type(&self) -> EventType {
        EventType::new(String::from("DrawCardEventHandler"))
    }

    fn handle(&mut self, event: &EventMessage) {
        let draw_card_event: DrawCardEvent = event.unpack();

        // ... do stuff
    }
}
