use crate::{
    game_logic::{
        event_handlers::EventHandler, events::EndTurnEvent, EventDispatcher, PlayerGainManaEvent,
        PlayerSpendManaEvent, TurnStartEvent,
    },
    game_state::GameState,
};

#[derive(Default)]
pub struct PlayerSpendManaEventHandler;

impl EventHandler for PlayerSpendManaEventHandler {
    type Event = PlayerSpendManaEvent;

    fn handle(
        &self,
        event: PlayerSpendManaEvent,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        println!(
            "Player {:?} spends {} mana.",
            event.player_id(),
            event.mana_count()
        );

        game_state.reduce_mana(event.player_id(), event.mana_count());
    }
}
