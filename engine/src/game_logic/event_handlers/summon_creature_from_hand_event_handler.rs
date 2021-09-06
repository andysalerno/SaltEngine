use log::{debug, info};

use crate::{
    game_logic::{
        event_handlers::EventHandler,
        events::{CreatureSetEvent, PlayerSpendManaEvent, SummonCreatureFromHandEvent},
        EventDispatcher,
    },
    game_state::GameState,
};
use async_trait::async_trait;

#[derive(Default)]
pub struct SummonCreatureFromHandEventHandler;

#[async_trait]
impl EventHandler for SummonCreatureFromHandEventHandler {
    type Event = SummonCreatureFromHandEvent;

    async fn handle(
        &self,
        event: SummonCreatureFromHandEvent,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        let player_id = event.player_id();

        // Take the card out of the player's hand
        debug!("Taking card from player's hand.");
        let mut card_from_hand = game_state
            .hand_mut(player_id)
            .take_card(event.hand_card_id());

        // Spend the mana
        {
            let creature_name = card_from_hand.definition().title();
            let mana_amount = card_from_hand.definition().cost();
            info!("Player {:?} summons {}", player_id, creature_name);

            dispatcher
                .dispatch(
                    PlayerSpendManaEvent::new(player_id, mana_amount as u32),
                    game_state,
                )
                .await;
        }

        let pos = event.board_pos();

        // Perform the "upon summon"
        {
            let upon_summon = card_from_hand.definition().upon_summon();
            upon_summon
                .action(&mut card_from_hand, pos, game_state, dispatcher)
                .await;
        }

        // Set the card instance on the board
        {
            dispatcher
                .dispatch(
                    CreatureSetEvent::new(player_id, card_from_hand, pos),
                    game_state,
                )
                .await;
        }
    }
}
