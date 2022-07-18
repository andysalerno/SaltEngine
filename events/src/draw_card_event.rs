use engine::{
    event::{Event, EventHandler, EventMessage, EventType},
    GameState,
};
use log::info;
use serde::{Deserialize, Serialize};

const HANDLER_NAME: &str = "DrawCardEventHandler";

#[derive(Serialize, Deserialize, Debug)]
pub struct DrawCardEvent {}

impl DrawCardEvent {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for DrawCardEvent {
    fn default() -> Self {
        Self::new()
    }
}

impl Event for DrawCardEvent {
    fn event_type(&self) -> EventType {
        EventType::new(HANDLER_NAME)
    }
}

pub struct DrawCardEventHandler;

impl DrawCardEventHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DrawCardEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl EventHandler for DrawCardEventHandler {
    fn event_type(&self) -> EventType {
        EventType::new(HANDLER_NAME)
    }

    fn handle(&mut self, event: &EventMessage, _game_state: &mut GameState) {
        let _draw_card_event: DrawCardEvent = event.unpack();

        info!("Player is drawing a card.");

        // ... do stuff
    }
}
