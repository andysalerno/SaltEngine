use crate::{
    event::{EventHandler, EventMessage, EventType},
    game_client::ClientChannel,
    GameState,
};
use log::info;
use std::collections::HashMap;

pub struct Dispatcher {
    // event_stack: ...
    event_handler_mapping: HashMap<EventType, Box<dyn EventHandler>>,
    player_a: Box<dyn ClientChannel>,
    player_b: Box<dyn ClientChannel>,
}

impl Dispatcher {
    /// # Panics
    /// Panics if there is a problem registering handlers for events.
    #[must_use]
    pub fn new(
        handlers: Vec<Box<dyn EventHandler>>,
        player_a: Box<dyn ClientChannel>,
        player_b: Box<dyn ClientChannel>,
    ) -> Self {
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
            player_a,
            player_b,
        }
    }

    /// # Panics
    /// Panics if there is no valid handler for the event.
    pub fn dispatch(&self, event: &EventMessage, game_state: &mut GameState) {
        let event_type = event.event_type();

        let matching_handler = self
            .event_handler_mapping
            .get(event_type)
            .unwrap_or_else(|| {
                panic!("no matching handler was found for event type {event_type:?}")
            });

        info!("Dispatching event {event:?}");

        self.player_a.push_message(event);
        self.player_b.push_message(event);

        matching_handler.handle(event, game_state, self);
    }

    #[must_use]
    pub fn player_a(&self) -> &dyn ClientChannel {
        self.player_a.as_ref()
    }

    #[must_use]
    pub fn player_b(&self) -> &dyn ClientChannel {
        self.player_b.as_ref()
    }
}
