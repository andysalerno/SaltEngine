use super::events::GameEvent;
use crate::game_logic::event_handlers::{
    DrawCardEventHandler, EventHandler, StartGameEventHandler,
};
use crate::game_logic::events::Event;
use crate::{
    game_agent::{ClientNotifier, Prompter},
    game_state::game_state::GameState,
};
use log::{debug, info};
use protocol::{entities::PlayerId, from_server::Notification};
use std::sync::Arc;

#[derive(Debug)]
pub struct EventDispatcher {
    stack: Vec<GameEvent>,
    player_a_notifier: Arc<dyn ClientNotifier>,
    player_a_prompter: Arc<dyn Prompter>,
    player_a_id: PlayerId,
    player_b_notifier: Arc<dyn ClientNotifier>,
    player_b_prompter: Arc<dyn Prompter>,
    player_b_id: PlayerId,
}

impl EventDispatcher {
    #[must_use]
    pub fn new(
        player_a_notifier: Arc<dyn ClientNotifier>,
        player_a_prompter: Arc<dyn Prompter>,
        player_a_id: PlayerId,
        player_b_notifier: Arc<dyn ClientNotifier>,
        player_b_prompter: Arc<dyn Prompter>,
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

    pub async fn dispatch(&mut self, event: impl Into<GameEvent>, game_state: &mut GameState) {
        let event = event.into();
        event
            .validate(game_state)
            .expect("Validation failed for dispatched event.");

        self.stack.push(event);

        while let Some(event) = self.stack.pop() {
            game_state.evaluate_passives();

            self.pre_handle(&event, game_state).await;
            self.handle(&event, game_state).await;
            self.post_handle(&event, game_state).await;

            game_state.evaluate_passives();
        }
    }

    pub async fn notify_players(&self, notification: Notification) {
        self.player_a_notifier.notify(notification.clone()).await;
        self.player_b_notifier.notify(notification).await;
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

    async fn pre_handle(&mut self, event: &GameEvent, game_state: &mut GameState) {
        // todo!()
        // let pre_existing_actions = game_state
        //     .board()
        //     .all_characters_slots()
        //     .creatures()
        //     .filter_map(|c| {
        //         c.definition()
        //             .pre_event_action(c.id(), event, game_state, self)
        //     })
        //     .collect::<Vec<_>>();

        // for action in pre_existing_actions {
        //     action.action(event, game_state, self).await;
        // }
    }

    async fn post_handle(&mut self, event: &GameEvent, game_state: &mut GameState) {
        // todo!()
        // let pre_existing_actions = game_state
        //     .board()
        //     .all_characters_slots()
        //     .creatures()
        //     .filter_map(|c| {
        //         c.definition()
        //             .post_event_action(c.id(), event, game_state, self)
        //     })
        //     .collect::<Vec<_>>();

        // for action in pre_existing_actions {
        //     action.action(event, game_state, self).await;
        // }
    }

    async fn handle(&mut self, event: &GameEvent, game_state: &mut GameState) {
        debug!("Dispatching event: {:?}", event);

        if let Some(client_event) = event.maybe_client_event(game_state.player_a_id(), game_state) {
            info!("Notifying player_a of event: {:?}", client_event);
            self.player_notifier(game_state.player_a_id())
                .notify(Notification::VisualEvent(client_event))
                .await;
        }

        if let Some(client_event) = event.maybe_client_event(game_state.player_b_id(), game_state) {
            info!("Notifying player_b of event: {:?}", client_event);
            self.player_notifier(game_state.player_b_id())
                .notify(Notification::VisualEvent(client_event))
                .await;
        }

        match event {
            GameEvent::AttackEvent(_) => todo!(),
            GameEvent::EndTurnEvent(_) => todo!(),
            GameEvent::CreatureSetEvent(_) => todo!(),
            GameEvent::CreatureDealsDamageEvent(_) => todo!(),
            GameEvent::CreatureTakesDamageEvent(_) => todo!(),
            GameEvent::CreatureDestroyedEvent(_) => todo!(),
            GameEvent::TurnStartEvent(_) => todo!(),
            GameEvent::DrawCardEvent(event) => {
                DrawCardEventHandler::default()
                    .handle(event, game_state, self)
                    .await
            }
            GameEvent::AddCardToHandEvent(_) => todo!(),
            GameEvent::StartGameEvent(event) => {
                StartGameEventHandler::default()
                    .handle(event, game_state, self)
                    .await
            }
            GameEvent::PlayerGainManaEvent(_) => todo!(),
            GameEvent::PlayerSpendManaEvent(_) => todo!(),
            GameEvent::CreatureSummonedFromHandEvent(_) => todo!(),
            GameEvent::PosTakesDamageEvent(_) => todo!(),
            GameEvent::CreatureHealedEvent(_) => todo!(),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::EventDispatcher;
//     use crate::game_agent::tests::{MockTestPrompter, StubNotifier};
//     use crate::game_state::{Deck, GameState};
//     use protocol::entities::PlayerId;
//     use std::sync::Arc;

//     pub(crate) fn make_test_state() -> GameState {
//         let player_a_deck = Deck::new(Vec::new());
//         let player_b_deck = Deck::new(Vec::new());

//         let mut state = GameState::initial_state(
//             PlayerId::new(),
//             player_a_deck,
//             PlayerId::new(),
//             player_b_deck,
//         );

//         state.raise_mana_limit(state.player_a_id(), 10);
//         state.raise_mana_limit(state.player_b_id(), 10);
//         state.refresh_player_mana(state.player_a_id());
//         state.refresh_player_mana(state.player_b_id());

//         state
//     }

//     #[test]
//     fn dispatcher_uses_stack_ordering() {
//         let prompter_a = Arc::new(MockTestPrompter::new());
//         let prompter_b = Arc::new(MockTestPrompter::new());

//         let notifier_a = Arc::new(StubNotifier);
//         let notifier_b = Arc::new(StubNotifier);

//         let _dispatcher = EventDispatcher::new(
//             notifier_a,
//             prompter_a,
//             PlayerId::new(),
//             notifier_b,
//             prompter_b,
//             PlayerId::new(),
//         );
//     }
// }
