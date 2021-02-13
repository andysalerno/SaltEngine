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
        let creature_instance = game_state.get_by_id(event.creature_id());

        println!(
            "{} was destroyed (instance id: {:?})",
            creature_instance.definition().title(),
            creature_instance.id()
        );

        let upon_death_event = creature_instance
            .definition()
            .upon_death(event.creature_id(), game_state);

        game_state.board_mut().remove_by_id(event.creature_id());

        upon_death_event
            .into_iter()
            .for_each(|e| dispatcher.dispatch(e, game_state));
    }
}
