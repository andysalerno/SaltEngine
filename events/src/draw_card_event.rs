pub use self::client_event::*;
pub use self::event::*;
pub use self::handler::*;

const HANDLER_NAME: &str = "DrawCardEventHandler";

/// Module containing the event for drawing a card.
mod event {
    use super::HANDLER_NAME;
    use engine::{
        event::{Event, EventType},
        PlayerId,
    };
    use serde::{Deserialize, Serialize};

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
    use crate::{draw_card_event::client_event, DrawCardEvent, HiddenInfo};
    use client_event::CardDrawnClientEvent;
    use engine::{
        event::{EventHandler, EventMessage, EventType},
        Dispatcher, FromServer, GameState,
    };
    use log::info;

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

        fn handle(
            &self,
            event: &EventMessage,
            game_state: &mut GameState,
            dispatcher: &Dispatcher,
        ) {
            let draw_card_event: DrawCardEvent = event.unpack();
            let player_id = draw_card_event.player_id();

            info!("Player {player_id:?} is drawing a card.");

            let deck = game_state.deck_mut(player_id);

            let card_drawn = if let Some(card_drawn) = deck.take_from_top() {
                card_drawn
            } else {
                info!("Player had no cards in deck left to draw.");
                return;
            };

            info!("Player drew: {card_drawn:?}");

            let hand = game_state.hand_mut(player_id);
            hand.add_to_right(card_drawn.clone());

            // Send outcome to player who drew.
            {
                let player_channel = dispatcher.player_channel(player_id);
                let client_event =
                    CardDrawnClientEvent::new(player_id, HiddenInfo::Visible(card_drawn));
                player_channel.send(FromServer::Event(client_event.into()));
            }

            // Send outcome to observing opponent.
            {
                let opponent_channel = dispatcher.opponent_channel(player_id);
                let client_event = CardDrawnClientEvent::new(player_id, HiddenInfo::Hidden);
                opponent_channel.send(FromServer::Event(client_event.into()));
            }
        }
    }
}

/// Module for the client event, allowing a client to observe the draw card event result.
mod client_event {
    use crate::HiddenInfo;
    use engine::{
        event::{Event, EventType},
        Card, CardDefinition, PlayerId,
    };
    use serde::{Deserialize, Serialize};

    /// The client event emitted when a card is drawn.
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct CardDrawnClientEvent {
        player_id: PlayerId,
        card_drawn: HiddenInfo<Card>,
    }

    impl CardDrawnClientEvent {
        pub fn new(player_id: PlayerId, card_drawn: HiddenInfo<Card>) -> Self {
            Self {
                player_id,
                card_drawn,
            }
        }

        pub fn player_id(&self) -> PlayerId {
            self.player_id
        }

        pub fn card_drawn(&self) -> &HiddenInfo<Card> {
            &self.card_drawn
        }
    }

    impl Event for CardDrawnClientEvent {
        fn event_type() -> engine::event::EventType {
            EventType::new("CardDrawnClientEvent")
        }
    }
}
