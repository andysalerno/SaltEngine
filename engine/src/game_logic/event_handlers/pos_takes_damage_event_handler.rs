use log::info;

use crate::{
    game_logic::{
        event_handlers::EventHandler,
        events::{CreatureTakesDamageEvent, PosTakesDamageEvent},
        EventDispatcher,
    },
    game_state::board::BoardView,
    game_state::GameState,
};
use async_trait::async_trait;

#[derive(Default)]
pub struct PosTakesDamageHandler;

#[async_trait]
impl EventHandler for PosTakesDamageHandler {
    type Event = PosTakesDamageEvent;

    async fn handle(
        &self,
        event: &PosTakesDamageEvent,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        info!(
            "Slot {:?} takes {} damage",
            event.pos(),
            event.damage_amount()
        );

        if let Some(creature_there) = game_state.board().creature_at_pos(event.pos()) {
            let damage_event =
                CreatureTakesDamageEvent::new(creature_there.id(), event.damage_amount());

            dispatcher.dispatch(damage_event, game_state).await;
        }
    }
}
