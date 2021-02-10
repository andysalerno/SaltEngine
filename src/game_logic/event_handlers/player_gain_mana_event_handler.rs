use crate::{
    game_logic::{event_handlers::EventHandler, EventDispatcher, PlayerGainManaEvent},
    game_state::GameState,
};

#[derive(Default)]
pub struct PlayerGainManaEventHandler;

impl EventHandler for PlayerGainManaEventHandler {
    type Event = PlayerGainManaEvent;

    fn handle(
        &self,
        event: PlayerGainManaEvent,
        game_state: &mut GameState,
        _dispatcher: &mut EventDispatcher,
    ) {
        println!(
            "Player {:?} gains {} mana.",
            event.player_id(),
            event.mana_count()
        );

        game_state.raise_mana_limit(event.player_id(), event.mana_count());
    }
}
