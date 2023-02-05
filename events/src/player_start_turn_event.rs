use engine::{
    event::{Event, EventHandler, EventMessage, EventType},
    Dispatcher, FromServer, GameState, PlayerId,
};
use log::info;
use serde::{Deserialize, Serialize};

const HANDLER_NAME: &str = "PlayerStartTurnEvent";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerStartTurnEvent {
    player_id: PlayerId,
    starting_mana: i32,
}

impl PlayerStartTurnEvent {
    pub fn new(player_id: PlayerId, starting_mana: i32) -> Self {
        Self {
            player_id,
            starting_mana,
        }
    }

    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }
}

impl Event for PlayerStartTurnEvent {
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

    fn handle(&self, event: &EventMessage, _game_state: &mut GameState, dispatcher: &Dispatcher) {
        let event: PlayerStartTurnEvent = event.unpack();
        let player_id = event.player_id();
        info!("Player turn start: {player_id:?}");

        dispatcher
            .player_a_channel()
            .send(FromServer::Event(event.clone().into()));
        dispatcher
            .player_b_channel()
            .send(FromServer::Event(event.into()));
    }
}
