use crate::{
    game_logic::{
        event_handlers::EventHandler, CreatureSetEvent, Event, EventDispatcher,
        PlayerSpendManaEvent, SummonCreatureFromHandEvent,
    },
    game_state::GameState,
};

#[derive(Default)]
pub struct SummonCreatureFromHandEventHandler;

impl EventHandler for SummonCreatureFromHandEventHandler {
    type Event = SummonCreatureFromHandEvent;

    fn handle(
        &self,
        event: SummonCreatureFromHandEvent,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        if let Err(e) = event.validate(game_state) {
            panic!("Event failed validation: {}", e);
        }

        let player_id = event.player_id();

        // Take the card out of the player's hand
        let mut card_from_hand = game_state
            .hand_mut(player_id)
            .take_card(event.hand_card_id());

        // Spend the mana
        {
            let creature_name = card_from_hand.definition().title();
            let mana_amount = card_from_hand.definition().cost();
            println!("Player {:?} summons {}", player_id, creature_name);

            dispatcher.dispatch(
                PlayerSpendManaEvent::new(player_id, mana_amount as u32),
                game_state,
            );
        }

        let pos = event.board_pos();

        // Perform the "upon summon"
        {
            let upon_summon = card_from_hand.definition().upon_summon();
            (upon_summon)(&mut card_from_hand, pos, game_state, dispatcher);
        }

        // Set the card instance on the board
        {
            dispatcher.dispatch(
                CreatureSetEvent::new(player_id, card_from_hand, pos),
                game_state,
            );
        }
    }
}
