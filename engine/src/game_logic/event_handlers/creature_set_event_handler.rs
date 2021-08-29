use crate::{
    game_logic::{event_handlers::EventHandler, events::CreatureSetEvent, EventDispatcher},
    game_state::GameState,
};
use async_trait::async_trait;

#[derive(Default)]
pub struct CreatureSetEventHandler;

#[async_trait]
impl EventHandler for CreatureSetEventHandler {
    type Event = CreatureSetEvent;

    async fn handle(
        &self,
        event: CreatureSetEvent,
        game_state: &mut GameState,
        _dispatcher: &mut EventDispatcher,
    ) {
        let target_position = event.target_position();
        let instance = event.take_card();
        game_state
            .board_mut()
            .set_creature_at_pos(target_position, instance);
    }
}
