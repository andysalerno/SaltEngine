use crate::game_state::GameState;

use super::{event_handlers::*, events::GameEvent};

pub struct EventDispatcher;

impl EventDispatcher {
    pub fn dispatch(event: GameEvent, game_state: &mut GameState) {
        println!("Dispatching: {:?}", event);

        match event {
            GameEvent::Attack(e) => AttackEventHandler::default().handle(e, game_state),
            GameEvent::EndTurn(e) => EndTurnEventHandler::default().handle(e, game_state),
            GameEvent::Summon(e) => SummonCreatureEventHandler::default().handle(e, game_state),
        }
    }
}
