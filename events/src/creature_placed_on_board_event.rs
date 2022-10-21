pub use self::event::*;
pub use self::handler::*;

const HANDLER_NAME: &str = "CreaturePlacedOnBoardEventHandler";

mod event {
    use super::HANDLER_NAME;
    use engine::{
        event::{Event, EventType},
        Card, GamePos, PlayerId,
    };
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CreaturePlacedOnBoardEvent {
        player_id: PlayerId,
        card: Card,
        placed_at: GamePos,
    }

    impl CreaturePlacedOnBoardEvent {
        pub fn new(player_id: PlayerId, card: Card, placed_at: GamePos) -> Self {
            Self {
                player_id,
                card,
                placed_at,
            }
        }

        pub fn player_id(&self) -> PlayerId {
            self.player_id
        }

        pub fn card(&self) -> &Card {
            &self.card
        }

        pub fn take_card(self) -> Card {
            self.card
        }

        pub fn placed_at(&self) -> GamePos {
            self.placed_at
        }
    }

    impl Event for CreaturePlacedOnBoardEvent {
        fn event_type() -> EventType {
            EventType::new(HANDLER_NAME)
        }
    }
}

mod handler {
    use super::HANDLER_NAME;
    use crate::CreaturePlacedOnBoardEvent;
    use engine::{
        event::{EventHandler, EventMessage, EventType},
        Dispatcher, FromServer, GameState,
    };
    use log::{error, info};

    pub struct CreaturePlacedOnBoardEventHandler;

    impl CreaturePlacedOnBoardEventHandler {
        pub fn new() -> Self {
            Self
        }
    }

    impl Default for CreaturePlacedOnBoardEventHandler {
        fn default() -> Self {
            Self::new()
        }
    }

    impl EventHandler for CreaturePlacedOnBoardEventHandler {
        fn event_type(&self) -> EventType {
            EventType::new(HANDLER_NAME)
        }

        fn handle(
            &self,
            event: &EventMessage,
            game_state: &mut GameState,
            dispatcher: &Dispatcher,
        ) {
            let creature_placed_event: CreaturePlacedOnBoardEvent = event.unpack();
            let pos = creature_placed_event.placed_at();
            let card = creature_placed_event.take_card();

            if game_state.card_at_pos(pos).is_some() {
                let msg = format!("Cannot place creature at {pos:?} because another creature already occupies it.");
                error!("{msg}");
                panic!("{msg}");
            }

            {
                let title = card.definition().title();
                info!("Creature {title} placed at {pos:?}");
            }

            game_state.set_card_at_pos(pos, card);

            dispatcher
                .player_a_channel()
                .send(FromServer::Event(event.clone()));

            dispatcher
                .player_b_channel()
                .send(FromServer::Event(event.clone()));
        }
    }
}
