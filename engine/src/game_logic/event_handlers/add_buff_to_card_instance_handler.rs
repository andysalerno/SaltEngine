use crate::{
    game_logic::{
        event_handlers::EventHandler, events::AddBuffToCardInstanceEvent, EventDispatcher,
    },
    game_state::GameState,
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
        _dispatcher: &mut EventDispatcher,
    ) {
        let card_instance = game_state
            .board_mut()
            .creature_instance_mut(event.recipient());

        card_instance.add_buff(Box::new(event.buff().clone()));

        info!("Added buff to {}", card_instance.definition().title());
    }
}
