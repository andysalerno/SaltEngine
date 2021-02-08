use crate::{
    game_logic::{
        event_handlers::EventHandler, CreatureSetEvent, EventDispatcher, PlayerSpendManaEvent,
        SummonCreatureFromHandEvent,
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
        let player_id = event.player_id();
        let hand_card_id = event.hand_card_id();

        let card_from_hand = game_state.hand_mut(player_id).take_card(hand_card_id);

        let creature_name = card_from_hand.definition().title();
        let mana_amount = card_from_hand.definition().cost();
        println!("Player {:?} summons {}", player_id, creature_name);

        dispatcher.dispatch(
            PlayerSpendManaEvent::new(player_id, mana_amount as u32),
            game_state,
        );

        let pos = event.board_pos();

        dispatcher.dispatch(
            CreatureSetEvent::new(player_id, card_from_hand, pos),
            game_state,
        );
    }
}
