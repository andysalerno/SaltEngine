use super::{event_handlers::*, events::GameEvent, Event};
use crate::{
    game_agent::game_agent::Prompter,
    game_state::{GameState, PlayerId},
};
use log::{debug};

#[derive(Debug)]
pub struct EventDispatcher {
    stack: Vec<GameEvent>,
    player_a_prompter: Box<dyn Prompter>,
    player_a_id: PlayerId,
    player_b_prompter: Box<dyn Prompter>,
    player_b_id: PlayerId,
}

impl EventDispatcher {
    pub fn new(
        player_a_prompter: Box<dyn Prompter>,
        player_a_id: PlayerId,
        player_b_prompter: Box<dyn Prompter>,
        player_b_id: PlayerId,
    ) -> Self {
        Self {
            stack: Vec::new(),
            player_a_prompter,
            player_a_id,
            player_b_prompter,
            player_b_id,
        }
    }

    pub fn dispatch(&mut self, event: impl Event, game_state: &mut GameState) {
        event
            .validate(game_state)
            .expect("Validation failed for dispatched event.");

        let event = event.into();

        self.stack.push(event);

        while let Some(event) = self.stack.pop() {
            game_state.evaluate_passives();

            self.handle(event, game_state);

            game_state.evaluate_passives();
        }
    }

    pub fn player_prompter(&self, player_id: PlayerId) -> &dyn Prompter {
        if player_id == self.player_a_id {
            self.player_a_prompter.as_ref()
        } else if player_id == self.player_b_id {
            self.player_b_prompter.as_ref()
        } else {
            panic!("Cannot get prompt for unknown player ID: {:?}", player_id)
        }
    }

    fn handle(&mut self, event: GameEvent, game_state: &mut GameState) {
        debug!("Dispatching event: {:?}", event);

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
            GameEvent::CreatureHealed(e) => {
                CreatureHealedEventHandler::default().handle(e, game_state, self)
            }
        }
    }
}
