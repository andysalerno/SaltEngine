use engine::{
    event::{Event, EventHandler, EventMessage, EventType},
    Dispatcher, GameState,
};
use log::info;
use serde::{Deserialize, Serialize};

use crate::DrawCardEvent;

const HANDLER_NAME: &str = "StartGameEventHandler";
const INITIAL_CARD_DRAW_COUNT: usize = 6;

/// An event that triggers the start of the game.
#[derive(Serialize, Deserialize, Debug)]
pub struct StartGameEvent {}

impl StartGameEvent {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for StartGameEvent {
    fn default() -> Self {
        Self::new()
    }
}

impl Event for StartGameEvent {
    // fn event_type(&self) -> EventType {
    //     EventType::new(HANDLER_NAME)
    // }

    fn event_type() -> EventType {
        EventType::new(HANDLER_NAME)
    }
}

pub struct StartGameEventHandler;

impl StartGameEventHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StartGameEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl EventHandler for StartGameEventHandler {
    fn event_type(&self) -> EventType {
        EventType::new(HANDLER_NAME)
    }

    fn handle(&self, event: &EventMessage, game_state: &mut GameState, dispatcher: &Dispatcher) {
        let _: StartGameEvent = event.unpack();

        info!("Begin phase: players draw cards.");

        // Both players draw cards.
        for _ in 0..INITIAL_CARD_DRAW_COUNT {
            let draw_card_event = EventMessage::from(DrawCardEvent::new(game_state.player_id_a()));
            dispatcher.dispatch(&draw_card_event, game_state);

            let draw_card_event = EventMessage::from(DrawCardEvent::new(game_state.player_id_b()));
            dispatcher.dispatch(&draw_card_event, game_state);
        }

        info!("End phase: players draw cards.");

        // ... do stuff
    }
}
