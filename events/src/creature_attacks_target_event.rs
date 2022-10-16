pub use self::event::*;
pub use self::handler::*;

const HANDLER_NAME: &str = "CreatureAttacksTargetEventHandler";

/// Module containing the event for drawing a card.
mod event {
    use super::HANDLER_NAME;
    use engine::{
        event::{Event, EventType},
        PlayerId,
    };
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreatureAttacksTargetEvent {
        player_id: PlayerId,
    }

    impl CreatureAttacksTargetEvent {
        pub fn new(player_id: PlayerId) -> Self {
            Self { player_id }
        }

        pub fn player_id(&self) -> PlayerId {
            self.player_id
        }
    }

    impl Event for CreatureAttacksTargetEvent {
        // fn event_type(&self) -> EventType {
        //     EventType::new(HANDLER_NAME)
        // }

        fn event_type() -> EventType {
            EventType::new(HANDLER_NAME)
        }
    }
}

/// Module containing the handler for the draw card event.
mod handler {
    use super::HANDLER_NAME;
    use crate::{creature_attacks_target_event::CreatureAttacksTargetEvent, HiddenInfo};
    use engine::{
        event::{EventHandler, EventMessage, EventType},
        Dispatcher, FromServer, GameState,
    };
    use log::info;

    pub struct CreatureAttacksTargetEvenetHandler;

    impl CreatureAttacksTargetEvenetHandler {
        pub fn new() -> Self {
            Self
        }
    }

    impl Default for CreatureAttacksTargetEvenetHandler {
        fn default() -> Self {
            Self::new()
        }
    }

    impl EventHandler for CreatureAttacksTargetEvenetHandler {
        fn event_type(&self) -> EventType {
            EventType::new(HANDLER_NAME)
        }

        fn handle(
            &self,
            event: &EventMessage,
            game_state: &mut GameState,
            dispatcher: &Dispatcher,
        ) {
            let draw_card_event: CreatureAttacksTargetEvent = event.unpack();
            let player_id = draw_card_event.player_id();
        }
    }
}
