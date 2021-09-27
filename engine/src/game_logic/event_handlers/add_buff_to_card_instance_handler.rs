use crate::{
    game_logic::{
        event_handlers::EventHandler,
        events::{
            AddBuffToCardInstanceEvent, AddCardToHandClientEvent, AddCardToHandEvent,
            ClientEventView,
        },
        EventDispatcher,
    },
    game_state::{GameState, MakePlayerView},
};
use async_trait::async_trait;
use log::info;

#[derive(Default)]
pub struct AddBuffToCardInstanceHandler;

#[async_trait]
impl EventHandler for AddBuffToCardInstanceHandler {
    type Event = AddBuffToCardInstanceEvent;

    async fn handle(
        &self,
        event: &AddBuffToCardInstanceEvent,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
    }
}
