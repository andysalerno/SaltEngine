use crate::{
    game_logic::{event_handlers::EventHandler, events::EndTurnEvent},
    game_state::GameState,
};

#[derive(Default)]
pub struct EndTurnEventHandler;

impl EventHandler for EndTurnEventHandler {
    type Event = EndTurnEvent;

    fn handle(&self, event: &EndTurnEvent, game_state: &mut GameState) {
        todo!()
    }
}
