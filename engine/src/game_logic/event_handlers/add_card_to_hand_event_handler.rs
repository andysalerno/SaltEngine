use log::info;

use crate::{
    game_logic::{
        event_handlers::EventHandler,
        events::{AddCardToHandClientEvent, AddCardToHandEvent, ClientEventView},
        EventDispatcher,
    },
    game_state::{GameState, MakePlayerView},
};
use async_trait::async_trait;

#[derive(Default)]
pub struct AddCardToHandEventHandler;

#[async_trait]
impl EventHandler for AddCardToHandEventHandler {
    type Event = AddCardToHandEvent;

    async fn handle(
        &self,
        event: AddCardToHandEvent,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        let player_id = event.player_id();
        let event_view: AddCardToHandClientEvent = event.player_view(player_id);
        let client_event = ClientEventView::AddCardToHand(event_view);

        game_state.hand_mut(player_id).add_card(event.take_card());

        info!(
            "Player {:?} adds a card to hand. Next hand size: {}",
            player_id,
            game_state.hand(player_id).len()
        );

        dispatcher
            .player_notifier(player_id)
            .notify(client_event)
            .await;
    }
}
