use log::info;
use protocol::{
    entities::{BoardPos, Entity, EntityPosition, PlayerHero},
    from_server::{EntityAdded, Notification},
};

use crate::{
    game_logic::{
        event_dispatch::EventDispatcher,
        event_handlers::EventHandler,
        events::{DrawCardEvent, StartGameEvent, TurnStartEvent},
    },
    game_state::game_state::GameState,
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
            "Game start. Player A: {:?} Player B: {:?}. Adding cards to hand.",
            player_a_id, player_b_id
        );

        // 1. Add entities for player heroes
        // let player_a_hero = PlayerHero::new(player_a_id);
        // let player_a_hero_entity: Entity = player_a_hero.into();
        // let player_a_hero_board_pos = BoardPos::hero_pos(player_a_id);
        // let player_a_hero_pos = EntityPosition::BoardPos(player_a_hero_board_pos);
        // game_state
        //     .board_mut()
        //     .set_creature_at_pos(player_a_hero_board_pos, player_a_hero);

        // let player_b_hero = HeroDefinition.make_instance();
        // let player_b_hero_entity = player_b_hero.as_entity();
        // let player_b_hero_board_pos = BoardPos::hero_pos(player_b_id);
        // let player_b_hero_pos = EntityPosition::BoardPos(player_b_hero_board_pos);
        // game_state
        //     .board_mut()
        //     .set_creature_at_pos(BoardPos::hero_pos(player_b_id), player_b_hero);

        // dispatcher
        //     .notify_players(Notification::EntityAdded(EntityAdded::new(
        //         player_a_hero_entity.id(),
        //         player_a_hero_entity,
        //         player_a_hero_pos,
        //     )))
        //     .await;

        // dispatcher
        //     .notify_players(Notification::EntityAdded(EntityAdded::new(
        //         player_b_hero_entity.id(),
        //         player_b_hero_entity,
        //         player_b_hero_pos,
        //     )))
        //     .await;

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
