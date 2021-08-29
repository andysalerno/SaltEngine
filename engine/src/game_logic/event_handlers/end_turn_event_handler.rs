use log::info;

use crate::{
    game_logic::{
        event_handlers::EventHandler, events::EndTurnEvent, EventDispatcher, TurnStartEvent,
    },
    game_state::{GameState, GameStateView, IterAddons},
};
use async_trait::async_trait;

#[derive(Default)]
pub struct EndTurnEventHandler;

#[async_trait]
impl EventHandler for EndTurnEventHandler {
    type Event = EndTurnEvent;

    async fn handle(
        &self,
        _event: EndTurnEvent,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        info!("Player {:?} ends turn", game_state.cur_player_id());

        // Trigger 'upon turn end' events
        game_state
            .iter()
            .creatures()
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
