use crate::{
    game_logic::{event_handlers::EventHandler, events::AttackEvent},
    game_state::GameState,
};

#[derive(Default)]
pub struct AttackEventHandler;

impl EventHandler for AttackEventHandler {
    type Event = AttackEvent;

    fn handle(&self, event: &AttackEvent, game_state: &mut GameState) {
        todo!()
    }
}
