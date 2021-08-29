use log::info;

use crate::{
    game_logic::{event_handlers::EventHandler, EventDispatcher, PlayerGainManaEvent},
    game_state::GameState,
};
use async_trait::async_trait;

#[derive(Default)]
pub struct PlayerGainManaEventHandler;

#[async_trait]
impl EventHandler for PlayerGainManaEventHandler {
    type Event = PlayerGainManaEvent;

    async fn handle(
        &self,
        event: PlayerGainManaEvent,
        game_state: &mut GameState,
        _dispatcher: &mut EventDispatcher,
    ) {
        info!(
            "Player {:?} gains {} mana.",
            event.player_id(),
            event.mana_count()
        );

        game_state.raise_mana_limit(event.player_id(), event.mana_count());
    }
}
