use crate::{
    game_logic::{
        event_handlers::EventHandler, CreatureDestroyedEvent, CreatureTakesDamageEvent,
        EventDispatcher,
    },
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
        dispatcher: &mut EventDispatcher,
    ) {
        let title = game_state
            .board()
            .creature_instance(event.creature_id())
            .definition()
            .title();

        println!("{} takes {} damage", title, event.damage_amount());

        game_state.update_by_id(event.creature_id(), |c| {
            c.take_damage(event.damage_amount());
        });

        let card_trigger = game_state
            .board()
            .creature_instance(event.creature_id())
            .definition()
            .upon_receive_damage();

        (card_trigger)(event.creature_id(), game_state, dispatcher);

        if game_state
            .board()
            .creature_instance(event.creature_id())
            .health()
            <= 0
        {
            dispatcher.dispatch(CreatureDestroyedEvent::new(event.creature_id()), game_state);
        }
    }
}
