use crate::{
    game_logic::{event_handlers::EventHandler, DrawCardEvent, EventDispatcher, TurnStartEvent},
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
        dispatcher: &mut EventDispatcher,
    ) {
        println!("Turn started for player {:?}", game_state.cur_player_id());

        let draw_event = DrawCardEvent::new(game_state.cur_player_id());
        dispatcher.dispatch(draw_event, game_state);
    }
}
