use std::convert::TryInto;

use crate::{
    game_logic::{event_handlers::EventHandler, events::*, EventDispatcher},
    game_state::GameState,
};

#[derive(Default)]
pub struct AttackEventHandler;

impl EventHandler for AttackEventHandler {
    type Event = AttackEvent;

    fn handle(
        &self,
        event: AttackEvent,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        let attacker_instance = game_state.get_by_id(event.attacker());
        let target_instance = game_state.get_by_id(event.target());
        let attack_amount = attacker_instance.attack() as usize;
        println!(
            "{} attacks {} for {} damage",
            attacker_instance.definition().title(),
            target_instance.definition().title(),
            attack_amount
        );

        let deal_damage_event = GameEvent::CreatureDealsDamage(CreatureDealsDamageEvent::new(
            event.attacker(),
            event.target(),
            attack_amount,
        ));

        dispatcher.dispatch(deal_damage_event, game_state);

        let take_damage_event = GameEvent::CreatureTakesDamage(CreatureTakesDamageEvent::new(
            event.target(),
            event.attacker(),
            attack_amount,
        ));

        dispatcher.dispatch(take_damage_event, game_state);
    }
}
