use log::info;
use protocol::from_server::{EntityAdded, Notification};

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
        _event: &StartGameEvent,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        let player_a_id = game_state.player_a_id();
        let player_b_id = game_state.player_b_id();

        info!(
            "Game start.\nPlayer A: {:?}\nPlayer B: {:?}.  Adding cards to hand.",
            player_a_id, player_b_id
        );

        // 1. Add entities for player heroes
        let player_a_hero = game_state.board().hero(player_a_id);
        let player_b_hero = game_state.board().hero(player_b_id);

        dispatcher
            .notify_players(Notification::EntityAdded(EntityAdded::new(
                player_a_hero.id(),
                player_a_hero.as_entity(),
            )))
            .await;

        dispatcher
            .notify_players(Notification::EntityAdded(EntityAdded::new(
                player_b_hero.id(),
                player_b_hero.as_entity(),
            )))
            .await;

        // 2. Players draw initial hand
        for _ in 0..START_GAME_CARD_COUNT {
            dispatcher
                .dispatch(DrawCardEvent::new(player_a_id), game_state)
                .await;
            dispatcher
                .dispatch(DrawCardEvent::new(player_b_id), game_state)
                .await;
        }
    }
}
