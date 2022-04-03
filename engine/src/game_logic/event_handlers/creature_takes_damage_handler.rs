use log::info;
use protocol::{
    entities::{CreatureDefinition, CreatureInstance, Id, IsEntity},
    from_server::EntityUpdate,
};

use crate::{
    game_logic::{
        event_handlers::EventHandler,
        events::{CreatureDestroyedEvent, CreatureTakesDamageEvent},
        EventDispatcher,
    },
    game_state::board::BoardView,
    game_state::GameState,
};
use async_trait::async_trait;

#[derive(Default)]
pub struct CreatureTakesDamageHandler;

#[async_trait]
impl EventHandler for CreatureTakesDamageHandler {
    type Event = CreatureTakesDamageEvent;

    async fn handle(
        &self,
        event: &CreatureTakesDamageEvent,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        let creature_instance = game_state.board().creature_instance(event.creature_id());
        let title = creature_instance.definition().title();

        info!("{} takes {} damage", title, event.damage_amount());

        let entity_update = EntityUpdate {
            id: Id::new(),
            entity_type_id: CreatureInstance::type_id(),
            property_names: vec!["testing".into()],
            property_values: vec!["testing".into()],
        };

        game_state.update_by_id(event.creature_id(), |c| {
            c.take_damage(event.damage_amount());
        });

        let card_trigger = game_state
            .board()
            .creature_instance(event.creature_id())
            .definition()
            .upon_receive_damage();

        card_trigger
            .action(event.creature_id(), game_state, dispatcher)
            .await;

        if game_state
            .board()
            .creature_instance(event.creature_id())
            .health()
            <= 0
        {
            dispatcher
                .dispatch(CreatureDestroyedEvent::new(event.creature_id()), game_state)
                .await;
        }
    }
}
