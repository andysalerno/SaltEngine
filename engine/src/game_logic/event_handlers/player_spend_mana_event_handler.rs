use log::info;

use crate::{
    game_logic::{event_handlers::EventHandler, events::PlayerSpendManaEvent, EventDispatcher},
    game_state::{GameState, GameStateView},
};
use async_trait::async_trait;

#[derive(Default)]
pub struct PlayerSpendManaEventHandler;

#[async_trait]
impl EventHandler for PlayerSpendManaEventHandler {
    type Event = PlayerSpendManaEvent;

    async fn handle(
        &self,
        event: PlayerSpendManaEvent,
        game_state: &mut GameState,
        _dispatcher: &mut EventDispatcher,
    ) {
        let player_id = event.player_id();

        info!("Player {:?} spends {} mana.", player_id, event.mana_count());

        let cur_mana = game_state.player_mana(player_id);

        assert!(
            event.mana_count() <= cur_mana,
            "Player does not have enough mana."
        );

        game_state.reduce_mana(event.player_id(), event.mana_count());
    }
}
