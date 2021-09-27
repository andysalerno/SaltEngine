use crate::{
    game_logic::{event_handlers::EventHandler, events::CreatureSetEvent, EventDispatcher},
    game_state::GameState,
};
use async_trait::async_trait;
use log::info;

#[derive(Default)]
pub struct CreatureSetEventHandler;

#[async_trait]
impl EventHandler for CreatureSetEventHandler {
    type Event = CreatureSetEvent;

    async fn handle(
        &self,
        event: &CreatureSetEvent,
        game_state: &mut GameState,
        _dispatcher: &mut EventDispatcher,
    ) {
        let player_id = event.player_id();
        let target_position = event.target_position();

        let card_instance = game_state.board_mut().take_pre_summon();

        if card_instance.id() != event.card_id() {
            panic!(
                "Took a card from purgatory, but it wasn't the one in the summon event request."
            );
        }

        info!(
            "{} is set on pos {:?} by player {:?}",
            card_instance.definition().title(),
            target_position,
            player_id
        );

        game_state
            .board_mut()
            .set_creature_at_pos(target_position, card_instance);
    }
}
