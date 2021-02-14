use crate::{
    game_logic::{event_handlers::EventHandler, EventDispatcher, PromptPlayerEvent},
    game_state::GameState,
};

#[derive(Default)]
pub struct PromptPlayerEventHandler;

impl EventHandler for PromptPlayerEventHandler {
    type Event = PromptPlayerEvent;

    fn handle(
        &self,
        _event: PromptPlayerEvent,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
    }
}
