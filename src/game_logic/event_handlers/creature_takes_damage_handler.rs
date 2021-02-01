use crate::{
    game_logic::{
        event_handlers::EventHandler, CreatureDestroyedEvent, CreatureTakesDamageEvent,
        EventDispatcher, GameEvent,
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
            .get_by_id(event.creature_id())
            .definition()
            .title();

        println!("{} takes {} damage", title, event.damage_amount());

        game_state
            .board_mut()
            .update_by_id(event.creature_id(), |c| {
                c.take_damage(event.damage_amount());
            });

        if game_state.get_by_id(event.creature_id()).health() <= 0 {
            dispatcher.dispatch(
                GameEvent::CreatureDestroyed(CreatureDestroyedEvent::new(event.creature_id())),
                game_state,
            );
        }
    }
}
