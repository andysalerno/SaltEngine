use engine::{
    event::{Event, EventHandler, EventMessage, EventType},
    Dispatcher, GameState, PlayerId,
};
use log::info;
use serde::{Deserialize, Serialize};

const HANDLER_NAME: &str = "PlayerStartTurnEvent";

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerStartTurnEvent {
    player_id: PlayerId,
}

impl PlayerStartTurnEvent {
    pub fn new(player_id: PlayerId) -> Self {
        Self { player_id }
    }

    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }
}

impl Event for PlayerStartTurnEvent {
    // fn event_type(&self) -> EventType {
    //     EventType::new(HANDLER_NAME)
    // }

    fn event_type() -> EventType {
        EventType::new(HANDLER_NAME)
    }
}

pub struct PlayerStartTurnEventHandler;

impl PlayerStartTurnEventHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PlayerStartTurnEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl EventHandler for PlayerStartTurnEventHandler {
    fn event_type(&self) -> EventType {
        EventType::new(HANDLER_NAME)
    }

    fn handle(&self, event: &EventMessage, _game_state: &mut GameState, _dispatcher: &Dispatcher) {
        let event: PlayerStartTurnEvent = event.unpack();
        let player_id = event.player_id();
        info!("Player turn start: {player_id:?}");
    }
}
