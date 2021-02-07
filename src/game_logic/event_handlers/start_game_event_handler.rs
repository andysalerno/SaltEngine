use crate::{
    game_logic::{
        event_handlers::EventHandler, CreatureDealsDamageEvent, DrawCardEvent, EventDispatcher,
        StartGameEvent,
    },
    game_state::GameState,
};

#[derive(Default)]
pub struct StartGameEventHandler;

const START_GAME_CARD_COUNT: usize = 5;

impl EventHandler for StartGameEventHandler {
    type Event = StartGameEvent;

    fn handle(
        &self,
        _event: StartGameEvent,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        let player_a_id = game_state.player_a_id();
        let player_b_id = game_state.player_b_id();

        println!(
            "Game start.\nPlayer A: {:?}\nPlayer B: {:?}",
            player_a_id, player_b_id
        );

        (0..START_GAME_CARD_COUNT).for_each(|_| {
            dispatcher.dispatch(DrawCardEvent::new(player_a_id), game_state);
            dispatcher.dispatch(DrawCardEvent::new(player_b_id), game_state);
        });
    }
}
