use crate::{
    game_logic::{event_handlers::EventHandler, events::CreatureSetEvent, EventDispatcher},
    game_state::{GameState, UnitCardInstance},
};

#[derive(Default)]
pub struct CreatureSetEventHandler;

impl EventHandler for CreatureSetEventHandler {
    type Event = CreatureSetEvent;

    fn handle(
        &self,
        event: CreatureSetEvent,
        game_state: &mut GameState,
        _dispatcher: &mut EventDispatcher,
    ) {
        let target_position = event.target_position();
        let instance = event.take_card();
        game_state.board_mut().set_at(target_position, instance);
    }
}
