use std::collections::VecDeque;

use crate::{
    game_logic::{
        event_handlers::EventHandler, events::EndTurnEvent, EventDispatcher, TurnStartEvent,
    },
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
        dispatcher: &mut EventDispatcher,
    ) {
        println!("Player {:?} ends turn", game_state.cur_player_id());

        // Trigger 'upon turn end' events
        game_state
            .board()
            .creatures_iter()
            .map(|c| (c.id(), c.definition().upon_turn_end()))
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(id, action)| {
                (action)(id, game_state, dispatcher);
            });

        game_state.set_next_player_turn();

        dispatcher.dispatch(TurnStartEvent, game_state);
    }
}
