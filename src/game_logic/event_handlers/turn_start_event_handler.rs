use crate::{
    game_logic::{
        event_handlers::EventHandler, DrawCardEvent, EventDispatcher, PlayerGainManaEvent,
        TurnStartEvent,
    },
    game_state::GameState,
};

#[derive(Default)]
pub struct TurnStartHandler;

impl EventHandler for TurnStartHandler {
    type Event = TurnStartEvent;

    fn handle(
        &self,
        _event: TurnStartEvent,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        let player_id = game_state.cur_player_id();
        println!("Turn started for player {:?}", player_id);

        dispatcher.dispatch(PlayerGainManaEvent::new(player_id, 1), game_state);

        game_state.refresh_player_mana(player_id);

        dispatcher.dispatch(DrawCardEvent::new(player_id), game_state);
    }
}
