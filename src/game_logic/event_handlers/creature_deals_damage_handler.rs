use crate::{
    game_logic::{event_handlers::EventHandler, CreatureDealsDamageEvent, EventDispatcher},
    game_state::GameState,
};

#[derive(Default)]
pub struct CreatureDealsDamageHandler;

impl EventHandler for CreatureDealsDamageHandler {
    type Event = CreatureDealsDamageEvent;

    fn handle(
        &self,
        event: CreatureDealsDamageEvent,
        game_state: &mut GameState,
        _dispatcher: &mut EventDispatcher,
    ) {
        let title = game_state
            .creature_instance(event.creature_id())
            .definition()
            .title();

        println!("{} deals {} damage", title, event.damage_amount());
    }
}
