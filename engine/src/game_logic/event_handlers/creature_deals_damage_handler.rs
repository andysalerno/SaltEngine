use log::info;

use crate::{
    game_logic::{event_handlers::EventHandler, events::CreatureDealsDamageEvent, EventDispatcher},
    game_state::board::BoardView,
    game_state::GameState,
};
use async_trait::async_trait;

#[derive(Default)]
pub struct CreatureDealsDamageHandler;

#[async_trait]
impl EventHandler for CreatureDealsDamageHandler {
    type Event = CreatureDealsDamageEvent;

    async fn handle(
        &self,
        event: CreatureDealsDamageEvent,
        game_state: &mut GameState,
        _dispatcher: &mut EventDispatcher,
    ) {
        let title = game_state
            .board()
            .creature_instance(event.creature_id())
            .definition()
            .title();

        info!("{} deals {} damage", title, event.damage_amount());
    }
}
