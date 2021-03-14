use crate::{
    game_logic::{event_handlers::EventHandler, CreatureHealedEvent, EventDispatcher},
    game_state::GameState,
};

#[derive(Default)]
pub struct CreatureHealedEventHandler;

impl EventHandler for CreatureHealedEventHandler {
    type Event = CreatureHealedEvent;

    fn handle(
        &self,
        event: CreatureHealedEvent,
        game_state: &mut GameState,
        _dispatcher: &mut EventDispatcher,
    ) {
        let healed_creature = game_state
            .board_mut()
            .creature_instance_mut(event.creature_id());

        let starting_health = healed_creature.health();
        let max_health = healed_creature.definition().health();

        let new_health = std::cmp::min(max_health, starting_health + event.heal_amount() as i32);

        println!(
            "{} heals {} health, from {} to {}.",
            healed_creature.definition().title(),
            event.heal_amount(),
            starting_health,
            new_health
        );

        healed_creature.receive_heal(event.heal_amount());
    }
}
