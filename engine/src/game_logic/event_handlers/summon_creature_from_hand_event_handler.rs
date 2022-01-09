use log::{debug, info};

use crate::{
    game_logic::{
        event_handlers::EventHandler,
        events::{CreatureSetEvent, CreatureSummonedFromHandEvent, PlayerSpendManaEvent},
        EventDispatcher,
    },
    game_state::GameState,
};
use async_trait::async_trait;

#[derive(Default)]
pub struct SummonCreatureFromHandEventHandler;

#[async_trait]
impl EventHandler for SummonCreatureFromHandEventHandler {
    type Event = CreatureSummonedFromHandEvent;

    async fn handle(
        &self,
        event: &CreatureSummonedFromHandEvent,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        let player_id = event.player_id();

        // Take the card out of the player's hand
        debug!("Taking card from player's hand.");
        let card_from_hand = game_state
            .hand_mut(player_id)
            .take_card(event.hand_card_id());

        {
            let creature_name = card_from_hand.definition().title();
            info!("Player summons {} ({:?})", creature_name, player_id);
        }

        let mana_amount = card_from_hand.definition().cost();
        let upon_summon = card_from_hand.definition().upon_summon();
        let card_instance_id = card_from_hand.id();

        game_state.board_mut().track_pending_card(card_from_hand);

        // Spend the mana
        {
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
            upon_summon
                .action(card_instance_id, pos, game_state, dispatcher)
                .await;
        }

        // Set the card instance on the board
        {
            dispatcher
                .dispatch(
                    CreatureSetEvent::new(player_id, card_instance_id, pos),
                    game_state,
                )
                .await;
        }
    }
}
