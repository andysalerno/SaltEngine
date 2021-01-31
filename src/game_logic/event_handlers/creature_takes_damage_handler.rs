use crate::{
    game_logic::{event_handlers::EventHandler, CreatureTakesDamageEvent, EventDispatcher},
    game_state::GameState,
};

#[derive(Default)]
pub struct CreatureTakesDamageHandler;

impl EventHandler for CreatureTakesDamageHandler {
    type Event = CreatureTakesDamageEvent;

    fn handle(
        &self,
        event: CreatureTakesDamageEvent,
        game_state: &mut GameState,
        _dispatcher: &mut EventDispatcher,
    ) {
        let title = game_state
            .get_by_id(event.creature_id())
            .definition()
            .title();

        println!("{} takes {} damage", title, event.damage_amount());
    }
}
