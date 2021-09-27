use log::info;

use crate::{
    game_logic::{event_handlers::EventHandler, events::CreatureDestroyedEvent, EventDispatcher},
    game_state::board::BoardView,
    game_state::GameState,
};
use async_trait::async_trait;

#[derive(Default)]
pub struct CreatureDestroyedEventHandler;

#[async_trait]
impl EventHandler for CreatureDestroyedEventHandler {
    type Event = CreatureDestroyedEvent;

    async fn handle(
        &self,
        event: &CreatureDestroyedEvent,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        let pos = game_state.board().pos_with_creature(event.creature_id());
        let creature_instance = game_state
            .board_mut()
            .take_creature_by_id(event.creature_id());

        let title = creature_instance.definition().title().to_string();
        let creature_id = creature_instance.id();
        let upon_death = creature_instance.definition().upon_death();

        info!("{} was destroyed (instance id: {:?})", title, creature_id);

        game_state.board_mut().add_to_graveyard(creature_instance);

        info!(
            "{} was added to the graveyard (instance id: {:?})",
            title, creature_id
        );

        upon_death
            .action(creature_id, pos, game_state, dispatcher)
            .await;
    }
}
