use crate::{
    game_logic::{event_handlers::EventHandler, events::EndTurnEvent, EventDispatcher},
    game_state::GameState,
};

#[derive(Default)]
pub struct EndTurnEventHandler;

impl EventHandler for EndTurnEventHandler {
    type Event = EndTurnEvent;

    fn handle(
        &self,
        _event: EndTurnEvent,
        game_state: &mut GameState,
        _dispatcher: &mut EventDispatcher,
    ) {
        println!("Player {:?} ends turn", game_state.cur_player_turn());
        game_state.set_next_player_turn();
    }
}
