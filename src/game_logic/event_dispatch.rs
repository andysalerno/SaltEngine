use crate::game_state::GameState;

use super::{event_handlers::*, events::GameEvent};

#[derive(Debug, Default)]
pub struct EventDispatcher {
    stack: Vec<GameEvent>,
}

impl EventDispatcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn dispatch(&mut self, event: GameEvent, game_state: &mut GameState) {
        println!("Dispatching: {:?}", event);

        self.stack.push(event);

        while let Some(event) = self.stack.pop() {
            game_state.evaluate_passives();

            match event {
                GameEvent::Attack(e) => AttackEventHandler::default().handle(e, game_state, self),
                GameEvent::EndTurn(e) => EndTurnEventHandler::default().handle(e, game_state, self),
                GameEvent::Summon(e) => {
                    SummonCreatureEventHandler::default().handle(e, game_state, self)
                }
                GameEvent::CreatureDealsDamage(e) => {
                    CreatureDealsDamageHandler::default().handle(e, game_state, self)
                }
                GameEvent::CreatureTakesDamage(e) => {
                    CreatureTakesDamageHandler::default().handle(e, game_state, self)
                }
                GameEvent::CreatureDestroyed(e) => {
                    CreatureDestroyedEventHandler::default().handle(e, game_state, self)
                }
                GameEvent::TurnEnd(e) => EndTurnEventHandler::default().handle(e, game_state, self),
                GameEvent::TurnStart(e) => TurnStartHandler::default().handle(e, game_state, self),
            }

            game_state.evaluate_passives();
        }
    }
}
