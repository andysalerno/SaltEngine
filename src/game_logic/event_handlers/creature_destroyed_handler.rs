use crate::{
    game_logic::{event_handlers::EventHandler, events::*, EventDispatcher},
    game_state::GameState,
};

#[derive(Default)]
pub struct CreatureDestroyedEventHandler;

impl EventHandler for CreatureDestroyedEventHandler {
    type Event = CreatureDestroyedEvent;

    fn handle(
        &self,
        event: CreatureDestroyedEvent,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        let pos = game_state.position_with_creature(event.creature_id());
        let mut creature_instance = game_state
            .board_mut()
            .take_creature_by_id(event.creature_id());

        println!(
            "{} was destroyed (instance id: {:?})",
            creature_instance.definition().title(),
            creature_instance.id()
        );

        let upon_death = creature_instance.definition().upon_death();
        (upon_death)(&mut creature_instance, pos, game_state, dispatcher);
    }
}
