use crate::{
    game_logic::{event_handlers::EventHandler, EventDispatcher, TurnStartEvent},
    game_state::GameState,
};

#[derive(Default)]
pub struct TurnStartHandler;

impl EventHandler for TurnStartHandler {
    type Event = TurnStartEvent;

    fn handle(
        &self,
        _event: TurnStartEvent,
        game_state: &mut GameState,
        _dispatcher: &mut EventDispatcher,
    ) {
        println!("Turn started for player {:?}", game_state.cur_player_turn());
    }
}
