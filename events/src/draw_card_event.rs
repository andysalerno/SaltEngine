use engine::{
    event::{Event, EventHandler, EventMessage, EventType},
    CardDefinition, Dispatcher, GameState, PlayerId,
};
use log::info;
use serde::{Deserialize, Serialize};

use crate::HiddenInfo;

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
    // fn event_type(&self) -> EventType {
    //     EventType::new(HANDLER_NAME)
    // }

    fn event_type() -> EventType {
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

    fn handle(&self, event: &EventMessage, game_state: &mut GameState, _dispatcher: &Dispatcher) {
        let draw_card_event: DrawCardEvent = event.unpack();
        let player_id = draw_card_event.player_id();

        info!("Player {player_id:?} is drawing a card.");

        let deck = game_state.deck_mut(player_id);

        if let Some(drew_card) = deck.take_from_top() {
            let hand = game_state.hand_mut(player_id);
            info!("Player drew: {drew_card:?}");
            hand.add_to_right(drew_card);
        } else {
            info!("Player had no cards in deck left to draw.");
        }

        // ... do stuff
    }
}
