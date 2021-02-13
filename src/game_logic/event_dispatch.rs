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

    pub fn dispatch(&mut self, event: impl Into<GameEvent>, game_state: &mut GameState) {
        let event = event.into();

        // println!("Dispatching: {:?}", event);

        self.stack.push(event);

        while let Some(event) = self.stack.pop() {
            game_state.evaluate_passives();

            self.handle(event, game_state);

            game_state.evaluate_passives();
        }
    }

    fn handle(&mut self, event: GameEvent, game_state: &mut GameState) {
        match event {
            GameEvent::Attack(e) => AttackEventHandler::default().handle(e, game_state, self),
            GameEvent::EndTurn(e) => EndTurnEventHandler::default().handle(e, game_state, self),
            GameEvent::Summon(e) => CreatureSetEventHandler::default().handle(e, game_state, self),
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
            GameEvent::DrawCard(e) => DrawCardEventHandler::default().handle(e, game_state, self),
            GameEvent::AddCardToHand(e) => {
                AddCardToHandEventHandler::default().handle(e, game_state, self)
            }
            GameEvent::StartGame(e) => StartGameEventHandler::default().handle(e, game_state, self),
            GameEvent::GainMana(e) => {
                PlayerGainManaEventHandler::default().handle(e, game_state, self)
            }
            GameEvent::SpendMana(e) => {
                PlayerSpendManaEventHandler::default().handle(e, game_state, self)
            }
            GameEvent::SummonCreatureFromHand(e) => {
                SummonCreatureFromHandEventHandler::default().handle(e, game_state, self)
            }
            GameEvent::PosTakesDamage(e) => {
                PosTakesDamageHandler::default().handle(e, game_state, self)
            }
        }
    }
}
