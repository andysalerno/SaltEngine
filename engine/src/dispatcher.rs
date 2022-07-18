use crate::{
    event::{EventHandler, EventMessage, EventType},
    GameState,
};
use log::info;
use std::collections::HashMap;

pub struct Dispatcher {
    // event_stack: ...
    event_handler_mapping: HashMap<EventType, Box<dyn EventHandler>>,
}

impl Dispatcher {
    /// # Panics
    /// Panics if there is a problem registering handlers for events.
    #[must_use]
    pub fn new(handlers: Vec<Box<dyn EventHandler>>) -> Self {
        // consume the handlers and map them.
        let handlers_provided = handlers.len();

        let mut mapping = HashMap::new();

        for handler in handlers {
            let event_type = handler.event_type();
            info!("Registered handler for event type {event_type:?}");
            mapping.insert(event_type, handler);
        }

        assert!(
            handlers_provided == mapping.keys().len(),
            "Conflict in event handlers and handled types."
        );

        Self {
            event_handler_mapping: mapping,
        }
    }

    /// # Panics
    /// Panics if there is no valid handler for the event.
    pub fn dispatch(&mut self, event: &EventMessage, game_state: &mut GameState) {
        let event_type = event.event_type();

        let matching_handler = self
            .event_handler_mapping
            .get_mut(event_type)
            .unwrap_or_else(|| {
                panic!("no matching handler was found for event type {event_type:?}")
            });

        info!("Dispatching event {event:?}");

        matching_handler.handle(event, game_state);
    }
}
