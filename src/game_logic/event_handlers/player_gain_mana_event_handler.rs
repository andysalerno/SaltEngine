use crate::{
    game_logic::{
        event_handlers::EventHandler, events::EndTurnEvent, EventDispatcher, PlayerGainManaEvent,
        TurnStartEvent,
    },
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

        game_state.gain_mana(event.player_id(), event.mana_count());
    }
}
