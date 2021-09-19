use log::info;

use crate::{
    game_logic::{
        event_handlers::EventHandler,
        events::{DrawCardEvent, StartGameEvent, TurnStartEvent},
        EventDispatcher,
    },
    game_state::GameState,
};
use async_trait::async_trait;

#[derive(Default)]
pub struct StartGameEventHandler;

const START_GAME_CARD_COUNT: usize = 5;

#[async_trait]
impl EventHandler for StartGameEventHandler {
    type Event = StartGameEvent;

    async fn handle(
        &self,
        _event: StartGameEvent,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        let player_a_id = game_state.player_a_id();
        let player_b_id = game_state.player_b_id();

        info!(
            "Game start.\nPlayer A: {:?}\nPlayer B: {:?}",
            player_a_id, player_b_id
        );

        for _ in 0..START_GAME_CARD_COUNT {
            dispatcher
                .dispatch(DrawCardEvent::new(player_a_id), game_state)
                .await;
            dispatcher
                .dispatch(DrawCardEvent::new(player_b_id), game_state)
                .await;
        }

        let first_turn_player = game_state.cur_player_id();

        dispatcher
            .dispatch(TurnStartEvent(first_turn_player), game_state)
            .await;
    }
}
