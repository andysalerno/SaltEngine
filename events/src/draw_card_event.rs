use engine::{
    event::{Event, EventHandler, EventMessage, EventType},
    Dispatcher, GameState, PlayerId,
};
use log::info;
use serde::{Deserialize, Serialize};

const HANDLER_NAME: &str = "DrawCardEventHandler";

#[derive(Serialize, Deserialize, Debug)]
pub struct DrawCardEvent {
    player_id: PlayerId,
}

impl DrawCardEvent {
    pub fn new(player_id: PlayerId) -> Self {
        Self { player_id }
    }

    pub fn player_id(&self) -> PlayerId {
        self.player_id
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

    fn handle(&self, event: &EventMessage, _game_state: &mut GameState, _dispatcher: &Dispatcher) {
        let draw_card_event: DrawCardEvent = event.unpack();
        let player_id = draw_card_event.player_id();

        info!("Player {player_id:?} is drawing a card.");

        // ... do stuff
    }
}
