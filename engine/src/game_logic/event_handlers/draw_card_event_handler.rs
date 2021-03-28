use log::info;

use crate::{
    game_logic::{
        event_handlers::EventHandler, events::CreatureTakesDamageEvent, AddCardToHandEvent,
        DrawCardEvent, EventDispatcher,
    },
    game_state::GameState,
};

#[derive(Default)]
pub struct DrawCardEventHandler;

impl EventHandler for DrawCardEventHandler {
    type Event = DrawCardEvent;

    fn handle(
        &self,
        event: DrawCardEvent,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        let player_id = event.player_id();
        info!(
            "Player {:?} draws a card. Deck size before draw: {}",
            player_id,
            game_state.deck(player_id).len()
        );

        let card = game_state.draw_card(player_id);

        if let Some(card) = card {
            let add_to_hand_event = AddCardToHandEvent::new(player_id, card);
            dispatcher.dispatch(add_to_hand_event, game_state);
        } else {
            info!(
                "Player {:?} had no cards in deck, so drew nothing.",
                player_id
            );

            let hero_id = game_state.board().hero(player_id).id();
            let hero_damaged_event = CreatureTakesDamageEvent::new(hero_id, 1);
            dispatcher.dispatch(hero_damaged_event, game_state);
        }
    }
}
