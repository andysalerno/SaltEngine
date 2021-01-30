use crate::{
    game_logic::cards::UnitCardDefinitionClone,
    game_logic::{event_handlers::EventHandler, events::SummonCreatureEvent},
    game_state::{GameState, UnitCardBoardInstance},
};

#[derive(Default)]
pub struct SummonCreatureEventHandler;

impl EventHandler for SummonCreatureEventHandler {
    type Event = SummonCreatureEvent;

    fn handle(&self, event: &SummonCreatureEvent, game_state: &mut GameState) {
        let instance = UnitCardBoardInstance::new((*event.definition()).clone_box());
        game_state
            .board_mut()
            .set_at(event.target_position(), instance);
    }
}
