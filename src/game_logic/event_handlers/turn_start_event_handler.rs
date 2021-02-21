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

        game_state
            .player_side(player_id)
            .filter_map(|s| {
                s.maybe_creature()
                    .map(|c| (c.id(), c.definition().upon_turn_start()))
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(id, action)| (action)(id, game_state, dispatcher));
    }
}
