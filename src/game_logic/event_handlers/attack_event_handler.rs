use crate::{
    game_logic::{event_handlers::EventHandler, events::*, EventDispatcher},
    game_state::GameState,
};

#[derive(Default)]
pub struct AttackEventHandler;

fn validate(event: &AttackEvent, game_state: &GameState) {
    let pos = game_state.position_with_creature(event.target());
    if game_state.is_pos_defended(pos) {
        panic!("Cannot attack defended pos {:?}", pos);
    }
}

impl EventHandler for AttackEventHandler {
    type Event = AttackEvent;

    fn handle(
        &self,
        event: AttackEvent,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        validate(&event, game_state);

        {
            let attacker_instance = game_state.creature_instance(event.attacker());
            let target_instance = game_state.creature_instance(event.target());
            let attack_amount = attacker_instance.attack() as usize;
            println!(
                "{} attacks {} for {} damage",
                attacker_instance.definition().title(),
                target_instance.definition().title(),
                attack_amount
            );
        }

        // 1. Attacker deals damage
        let attacker_attack_amount =
            game_state.creature_instance(event.attacker()).attack() as usize;
        let deal_damage_event =
            CreatureDealsDamageEvent::new(event.attacker(), event.target(), attacker_attack_amount);

        dispatcher.dispatch(deal_damage_event, game_state);

        // 2. Target deals damage
        let target_attack_amount = game_state.creature_instance(event.target()).attack() as usize;
        let deal_damage_event =
            CreatureDealsDamageEvent::new(event.target(), event.attacker(), target_attack_amount);

        dispatcher.dispatch(deal_damage_event, game_state);

        // 3. Target receives damage
        let take_damage_event =
            CreatureTakesDamageEvent::new(event.target(), attacker_attack_amount);

        dispatcher.dispatch(take_damage_event, game_state);

        // 4. Attacker receives damage
        let take_damage_event =
            CreatureTakesDamageEvent::new(event.attacker(), target_attack_amount);

        dispatcher.dispatch(take_damage_event, game_state);
    }
}
