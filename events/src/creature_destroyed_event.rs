pub use self::event::*;
pub use self::handler::*;

const HANDLER_NAME: &str = "CreatureDestroyedEvent";

/// Module containing the event for drawing a card.
mod event {
    use super::HANDLER_NAME;
    use engine::{
        event::{Event, EventType},
        CardId,
    };
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreatureDestroyedEvent {
        creature_destroyed: CardId,
    }

    impl CreatureDestroyedEvent {
        pub fn new(creature_destroyed: CardId) -> Self {
            Self { creature_destroyed }
        }

        pub fn creature_destroyed(&self) -> CardId {
            self.creature_destroyed
        }
    }

    impl Event for CreatureDestroyedEvent {
        fn event_type() -> EventType {
            EventType::new(HANDLER_NAME)
        }
    }
}

/// Module containing the handler for the draw card event.
mod handler {
    use super::HANDLER_NAME;
    use crate::creature_destroyed_event::CreatureDestroyedEvent;
    use engine::{
        event::{EventHandler, EventMessage, EventType},
        Dispatcher, FromServer, GameState,
    };
    use log::info;

    pub struct CreatureDestroyedEventHandler;

    impl CreatureDestroyedEventHandler {
        pub fn new() -> Self {
            Self
        }
    }

    impl Default for CreatureDestroyedEventHandler {
        fn default() -> Self {
            Self::new()
        }
    }

    impl EventHandler for CreatureDestroyedEventHandler {
        fn event_type(&self) -> EventType {
            EventType::new(HANDLER_NAME)
        }

        fn handle(
            &self,
            event: &EventMessage,
            _game_state: &mut GameState,
            dispatcher: &Dispatcher,
        ) {
            dispatcher
                .player_a_channel()
                .send(FromServer::Event(event.clone()));

            dispatcher
                .player_b_channel()
                .send(FromServer::Event(event.clone()));
        }
    }
}
