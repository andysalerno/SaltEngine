use crate::{
    game_logic::{
        event_handlers::EventHandler, events::EndTurnEvent, AddCardToHandEvent, DrawCardEvent,
        EventDispatcher, GameEvent, TurnStartEvent,
    },
    game_state::GameState,
};

#[derive(Default)]
pub struct AddCardToHandEventHandler;

impl EventHandler for AddCardToHandEventHandler {
    type Event = AddCardToHandEvent;

    fn handle(
        &self,
        _event: AddCardToHandEvent,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        let player_id = game_state.cur_player_id();
        println!("Player {:?} draws a card.", player_id);

        let _card = game_state.draw_card(player_id);

        dispatcher.dispatch(TurnStartEvent, game_state);
    }
}
