use crate::{
    game_logic::{
        event_handlers::EventHandler, events::EndTurnEvent, AddCardToHandEvent, DrawCardEvent,
        EventDispatcher, GameEvent, TurnStartEvent,
    },
    game_state::GameState,
};

#[derive(Default)]
pub struct DrawCardEventHandler;

impl EventHandler for DrawCardEventHandler {
    type Event = DrawCardEvent;

    fn handle(
        &self,
        _event: DrawCardEvent,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        let player_id = game_state.cur_player_id();
        println!("Player {:?} draws a card.", player_id);

        let card = game_state.draw_card(player_id);

        if let Some(card) = card {
            let add_to_hand_event = AddCardToHandEvent::new(player_id, card);
            dispatcher.dispatch(add_to_hand_event, game_state);
        } else {
            println!(
                "Player {:?} had no cards in deck, so drew nothing.",
                player_id
            );
        }
    }
}
