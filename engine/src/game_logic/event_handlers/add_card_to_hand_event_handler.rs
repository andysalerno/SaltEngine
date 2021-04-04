use log::info;

use crate::{
    game_logic::{event_handlers::EventHandler, AddCardToHandEvent, EventDispatcher},
    game_state::GameState,
};

#[derive(Default)]
pub struct AddCardToHandEventHandler;

impl EventHandler for AddCardToHandEventHandler {
    type Event = AddCardToHandEvent;

    fn handle(
        &self,
        event: AddCardToHandEvent,
        game_state: &mut GameState,
        _dispatcher: &mut EventDispatcher,
    ) {
        let player_id = event.player_id();

        game_state.hand_mut(player_id).add_card(event.take_card());

        info!(
            "Player {:?} adds a card to hand. Next hand size: {}",
            player_id,
            game_state.hand(player_id).len()
        );
    }
}
