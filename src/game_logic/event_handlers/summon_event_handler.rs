use crate::{
    game_logic::{event_handlers::EventHandler, events::SummonCreatureEvent},
    game_state::{GameState, UnitCardBoardInstance},
};

#[derive(Default)]
pub struct SummonCreatureEventHandler;

impl EventHandler for SummonCreatureEventHandler {
    type Event = SummonCreatureEvent;

    fn handle(&self, event: SummonCreatureEvent, game_state: &mut GameState) {
        let target_position = event.target_position();
        let instance = UnitCardBoardInstance::new(event.take_definition());
        game_state.board_mut().set_at(target_position, instance);
    }
}
