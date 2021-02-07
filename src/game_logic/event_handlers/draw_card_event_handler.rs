use crate::{
    game_logic::{
        event_handlers::EventHandler, events::EndTurnEvent, DrawCardEvent, EventDispatcher,
        GameEvent, TurnStartEvent,
    },
    game_state::GameState,
};

#[derive(Default)]
pub struct DrawCardEventHandler;

impl EventHandler for DrawCardEventHandler {
    type Event = DrawCardEvent;

    fn handle(
        &self,
        _event: DrawCardEvent,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        let player_id = game_state.cur_player_turn();
        println!("Player {:?} draws a card.", player_id);

        let card = game_state.draw_card(player_id);

        dispatcher.dispatch(TurnStartEvent, game_state);
    }
}
