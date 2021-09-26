use super::{
    event_handlers::{
        AddCardToHandEventHandler, AttackEventHandler, CreatureDealsDamageHandler,
        CreatureDestroyedEventHandler, CreatureHealedEventHandler, CreatureSetEventHandler,
        CreatureTakesDamageHandler, DrawCardEventHandler, EndTurnEventHandler, EventHandler,
        PlayerGainManaEventHandler, PlayerSpendManaEventHandler, PosTakesDamageHandler,
        StartGameEventHandler, SummonCreatureFromHandEventHandler, TurnStartHandler,
    },
    events::{Event, GameEvent},
};
use crate::{
    game_agent::{ClientNotifier, Prompter},
    game_state::{GameState, PlayerId},
};
use futures::join;
use log::{debug, info};

#[derive(Debug)]
pub struct EventDispatcher {
    stack: Vec<GameEvent>,
    player_a_notifier: Box<dyn ClientNotifier>,
    player_a_prompter: Box<dyn Prompter>,
    player_a_id: PlayerId,
    player_b_notifier: Box<dyn ClientNotifier>,
    player_b_prompter: Box<dyn Prompter>,
    player_b_id: PlayerId,
}

impl EventDispatcher {
    #[must_use]
    pub fn new(
        player_a_notifier: Box<dyn ClientNotifier>,
        player_a_prompter: Box<dyn Prompter>,
        player_a_id: PlayerId,
        player_b_notifier: Box<dyn ClientNotifier>,
        player_b_prompter: Box<dyn Prompter>,
        player_b_id: PlayerId,
    ) -> Self {
        Self {
            stack: Vec::new(),
            player_a_notifier,
            player_a_prompter,
            player_a_id,
            player_b_notifier,
            player_b_prompter,
            player_b_id,
        }
    }

    pub async fn dispatch(&mut self, event: impl Event, game_state: &mut GameState) {
        event
            .validate(game_state)
            .expect("Validation failed for dispatched event.");

        let event = event.into();

        self.stack.push(event);

        while let Some(event) = self.stack.pop() {
            game_state.evaluate_passives();

            self.handle(event, game_state).await;

            game_state.evaluate_passives();
        }
    }

    #[must_use]
    pub fn player_notifier(&self, player_id: PlayerId) -> &dyn ClientNotifier {
        if player_id == self.player_a_id {
            self.player_a_notifier.as_ref()
        } else if player_id == self.player_b_id {
            self.player_b_notifier.as_ref()
        } else {
            panic!("Cannot get notifier for unknown player ID: {:?}", player_id)
        }
    }

    #[must_use]
    pub fn opponent_notifier(&self, player_id: PlayerId) -> &dyn ClientNotifier {
        if player_id == self.player_a_id {
            self.player_b_notifier.as_ref()
        } else if player_id == self.player_b_id {
            self.player_a_notifier.as_ref()
        } else {
            panic!(
                "Cannot get notifier for opponent of unknown player ID: {:?}",
                player_id
            )
        }
    }

    #[must_use]
    pub fn player_prompter(&self, player_id: PlayerId) -> &dyn Prompter {
        if player_id == self.player_a_id {
            self.player_a_prompter.as_ref()
        } else if player_id == self.player_b_id {
            self.player_b_prompter.as_ref()
        } else {
            panic!("Cannot get notifier for unknown player ID: {:?}", player_id)
        }
    }

    async fn handle(&mut self, event: GameEvent, game_state: &mut GameState) {
        debug!("Dispatching event: {:?}", event);

        let maybe_client_event = event.maybe_client_event();

        if let Some(event_view) = maybe_client_event {
            info!("Notifying players of event: {:?}", event_view);
            let notify_opponent = self
                .opponent_notifier(game_state.cur_player_id())
                .notify(event_view.clone());

            let notify_player = self
                .player_notifier(game_state.cur_player_id())
                .notify(event_view);

            join!(notify_opponent, notify_player);
        }

        match event {
            GameEvent::Attack(e) => {
                AttackEventHandler::default()
                    .handle(e, game_state, self)
                    .await;
            }
            GameEvent::EndTurn(e) => {
                EndTurnEventHandler::default()
                    .handle(e, game_state, self)
                    .await;
            }
            GameEvent::Summon(e) => {
                CreatureSetEventHandler::default()
                    .handle(e, game_state, self)
                    .await;
            }
            GameEvent::CreatureDealsDamage(e) => {
                CreatureDealsDamageHandler::default()
                    .handle(e, game_state, self)
                    .await;
            }
            GameEvent::CreatureTakesDamage(e) => {
                CreatureTakesDamageHandler::default()
                    .handle(e, game_state, self)
                    .await;
            }
            GameEvent::CreatureDestroyed(e) => {
                CreatureDestroyedEventHandler::default()
                    .handle(e, game_state, self)
                    .await;
            }
            GameEvent::TurnStart(e) => {
                TurnStartHandler::default()
                    .handle(e, game_state, self)
                    .await;
            }
            GameEvent::DrawCard(e) => {
                DrawCardEventHandler::default()
                    .handle(e, game_state, self)
                    .await;
            }
            GameEvent::AddCardToHand(e) => {
                AddCardToHandEventHandler::default()
                    .handle(e, game_state, self)
                    .await;
            }
            GameEvent::StartGame(e) => {
                StartGameEventHandler::default()
                    .handle(e, game_state, self)
                    .await;
            }
            GameEvent::GainMana(e) => {
                PlayerGainManaEventHandler::default()
                    .handle(e, game_state, self)
                    .await;
            }
            GameEvent::SpendMana(e) => {
                PlayerSpendManaEventHandler::default()
                    .handle(e, game_state, self)
                    .await;
            }
            GameEvent::SummonCreatureFromHand(e) => {
                SummonCreatureFromHandEventHandler::default()
                    .handle(e, game_state, self)
                    .await;
            }
            GameEvent::PosTakesDamage(e) => {
                PosTakesDamageHandler::default()
                    .handle(e, game_state, self)
                    .await;
            }
            GameEvent::CreatureHealed(e) => {
                CreatureHealedEventHandler::default()
                    .handle(e, game_state, self)
                    .await;
            }
        }
    }

    async fn _invoke<E: Event, H: EventHandler<Event = E> + Default>(
        event: E,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        let handler = H::default();
        handler.handle(event, game_state, dispatcher).await;
    }
}

#[cfg(test)]
mod test {}
