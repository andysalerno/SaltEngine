use crate::{
    game_logic::{event_handlers::EventHandler, events::AddCardToHandEvent, EventDispatcher},
    game_state::GameState,
};
use async_trait::async_trait;
use log::info;
use protocol::from_server::Notification;

#[derive(Default)]
pub struct AddCardToHandEventHandler;

#[async_trait]
impl EventHandler for AddCardToHandEventHandler {
    type Event = AddCardToHandEvent;

    async fn handle(
        &self,
        event: &AddCardToHandEvent,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        let player_id = event.player_id();
        let card = game_state
            .board_mut()
            .take_tracked_pending_card(event.card_id())
            .expect("Expected to find the tracked pending card");

        game_state.hand_mut(event.player_id()).add_card(card);

        let notifier = dispatcher.player_notifier(player_id);
        //let notification = Notification::
        let added_card_entity = todo!();
        let notification = Notification::EntityAdded(added_card_entity);
        notifier.notify(notification).await;

        info!(
            "Player {:?} adds a card to hand. Next hand size: {}",
            player_id,
            game_state.hand(player_id).len()
        );
    }
}
