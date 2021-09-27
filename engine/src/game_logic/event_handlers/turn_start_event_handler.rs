use log::info;

use crate::{
    game_logic::{
        event_handlers::EventHandler,
        events::{DrawCardEvent, PlayerGainManaEvent, TurnStartEvent},
        EventDispatcher,
    },
    game_state::GameState,
};
use async_trait::async_trait;

#[derive(Default)]
pub struct TurnStartHandler;

#[async_trait]
impl EventHandler for TurnStartHandler {
    type Event = TurnStartEvent;

    async fn handle(
        &self,
        _event: &TurnStartEvent,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        let player_id = game_state.cur_player_id();
        info!("Turn started for player {:?}", player_id);

        dispatcher
            .dispatch(PlayerGainManaEvent::new(player_id, 1), game_state)
            .await;

        game_state.refresh_player_mana(player_id);

        dispatcher
            .dispatch(DrawCardEvent::new(player_id), game_state)
            .await;

        let turn_start_actions = game_state
            .board()
            .player_side(player_id)
            .iter()
            .filter_map(|s| {
                s.maybe_creature()
                    .map(|c| (c.id(), c.definition().upon_turn_start()))
            })
            .collect::<Vec<_>>()
            .into_iter();

        for (id, action) in turn_start_actions {
            action.action(id, game_state, dispatcher).await;
        }
    }
}
