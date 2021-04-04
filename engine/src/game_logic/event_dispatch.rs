use log::{debug, trace};

use crate::{
    game_agent::game_agent::Prompter,
    game_state::{GameState, PlayerId},
};

use super::{event_handlers::*, events::GameEvent};

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

    pub fn dispatch(&mut self, event: impl Into<GameEvent>, game_state: &mut GameState) {
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

    // #[cfg(test)]
    // pub fn set_player_a_prompter(&mut self, prompter: Box<dyn Prompter>) {
    //     self.player_a_prompter = prompter;
    // }

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

#[cfg(test)]
pub(crate) mod tests {
    use crate::{game_agent::game_agent::*, game_state::PlayerId};

    use super::EventDispatcher;

    pub fn make_default_dispatcher() -> EventDispatcher {
        let prompt_a = MockPrompter::new();
        let prompt_b = MockPrompter::new();

        make_dispatcher(prompt_a, PlayerId::new(), prompt_b, PlayerId::new())
    }

    pub fn make_dispatcher(
        prompter_a: impl Prompter + 'static,
        player_a_id: PlayerId,
        prompter_b: impl Prompter + 'static,
        player_b_id: PlayerId,
    ) -> EventDispatcher {
        EventDispatcher::new(
            Box::new(prompter_a),
            player_a_id,
            Box::new(prompter_b),
            player_b_id,
        )
    }
}
