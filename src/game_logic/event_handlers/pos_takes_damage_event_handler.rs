use crate::{
    game_logic::{
        event_handlers::EventHandler, CreatureTakesDamageEvent, EventDispatcher,
        PosTakesDamageEvent,
    },
    game_state::GameState,
};

#[derive(Default)]
pub struct PosTakesDamageHandler;

impl EventHandler for PosTakesDamageHandler {
    type Event = PosTakesDamageEvent;

    fn handle(
        &self,
        event: PosTakesDamageEvent,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        println!(
            "Slot {:?} takes {} damage",
            event.pos(),
            event.damage_amount()
        );

        if let Some(creature_there) = game_state.creature_at_pos(event.pos()) {
            let damage_event =
                CreatureTakesDamageEvent::new(creature_there.id(), event.damage_amount());

            dispatcher.dispatch(damage_event, game_state);
        }
    }
}
