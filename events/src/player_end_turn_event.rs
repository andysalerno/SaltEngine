use engine::{
    event::{Event, EventHandler, EventMessage, EventType},
    Dispatcher, FromServer, GameState, PlayerId,
};
use log::info;
use serde::{Deserialize, Serialize};

const HANDLER_NAME: &str = "PlayerEndTurnEvent";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PlayerEndTurnEvent {
    player_id: PlayerId,
}

impl PlayerEndTurnEvent {
    pub fn new(player_id: PlayerId) -> Self {
        Self { player_id }
    }

    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }
}

impl Event for PlayerEndTurnEvent {
    // fn event_type(&self) -> EventType {
    //     EventType::new(HANDLER_NAME)
    // }

    fn event_type() -> EventType {
        EventType::new(HANDLER_NAME)
    }
}

pub struct PlayerEndTurnEventHandler;

impl PlayerEndTurnEventHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PlayerEndTurnEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl EventHandler for PlayerEndTurnEventHandler {
    fn event_type(&self) -> EventType {
        EventType::new(HANDLER_NAME)
    }

    fn handle(&self, event: &EventMessage, game_state: &mut GameState, dispatcher: &Dispatcher) {
        let event: PlayerEndTurnEvent = event.unpack();
        let player_id = event.player_id();
        info!("Player turn end: {player_id:?}");

        game_state.set_next_cur_player_turn();

        dispatcher
            .player_a_channel()
            .send(FromServer::Event(event.clone().into()));
        dispatcher
            .player_b_channel()
            .send(FromServer::Event(event.into()));
    }
}
