use crate::game_logic::events::Event;
use crate::{
    game_logic::{event_handlers::EventHandler, EventDispatcher, PlayerSpendManaEvent},
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
        _dispatcher: &mut EventDispatcher,
    ) {
        if let Err(e) = event.validate(game_state) {
            panic!("Event failed validation: {}", e);
        }

        let player_id = event.player_id();

        println!("Player {:?} spends {} mana.", player_id, event.mana_count());

        let cur_mana = game_state.player_mana(player_id);

        assert!(
            event.mana_count() <= cur_mana,
            "Player does not have enough mana."
        );

        game_state.reduce_mana(event.player_id(), event.mana_count());
    }
}
