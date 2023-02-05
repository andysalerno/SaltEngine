use engine::{
    event::{Event, EventHandler, EventMessage, EventType},
    Dispatcher, FromServer, GameState, PlayerId,
};
use log::info;
use serde::{Deserialize, Serialize};

const HANDLER_NAME: &str = "GainManaEvent";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GainManaEvent {
    player_id: PlayerId,
    gain_count: i32,
}

impl GainManaEvent {
    pub fn new(player_id: PlayerId, gain_count: i32) -> Self {
        assert!(gain_count > 0);

        Self {
            player_id,
            gain_count,
        }
    }

    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }
}

impl Event for GainManaEvent {
    fn event_type() -> EventType {
        EventType::new(HANDLER_NAME)
    }
}

pub struct GainManaEventHandler;

impl GainManaEventHandler {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GainManaEventHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl EventHandler for GainManaEventHandler {
    fn event_type(&self) -> EventType {
        EventType::new(HANDLER_NAME)
    }

    fn handle(&self, event: &EventMessage, game_state: &mut GameState, _dispatcher: &Dispatcher) {
        let event: GainManaEvent = event.unpack();
        let gain_count = event.gain_count;
        let player_id = event.player_id();

        game_state.add_player_base_mana(player_id, gain_count);

        info!("Player gain mana: {player_id:?} count: {gain_count}");
    }
}
