use crate::{
    game_logic::{
        event_handlers::EventHandler, CreatureSetEvent, EventDispatcher, PlayerSpendManaEvent,
        SummonCreatureFromHandEvent,
    },
    game_state::GameState,
};

#[derive(Default)]
pub struct SummonCreatureFromHandEventHandler;

impl EventHandler for SummonCreatureFromHandEventHandler {
    type Event = SummonCreatureFromHandEvent;

    fn handle(
        &self,
        event: SummonCreatureFromHandEvent,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        let player_id = event.player_id();
        let creature_name = event.definition().title();
        let mana_amount = event.definition().cost();
        println!("Player {:?} summons {}", player_id, creature_name);

        dispatcher.dispatch(
            PlayerSpendManaEvent::new(player_id, mana_amount as u32),
            game_state,
        );

        let pos = event.board_pos();
        let definition = event.take_definition();
        dispatcher.dispatch(
            CreatureSetEvent::new(player_id, definition, pos),
            game_state,
        );
    }
}
