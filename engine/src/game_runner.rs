use crate::{
    game_agent::GameClient,
    game_logic::{
        event_dispatch::EventDispatcher,
        events::{GameEvent, StartGameEvent, TurnStartEvent},
    },
    game_state::game_state::GameState,
};
use log::info;

/// A runner for a game.
/// Maintains the current `GameState` at any given moment,
/// accepts inputs from `GameClient`s, and alerts `GameClient`s about events
/// throughout the duration of the game.
pub struct GameRunner {
    player_a_handler: Box<dyn GameClient>,
    player_b_handler: Box<dyn GameClient>,
    game_state: GameState,
}

impl GameRunner {
    #[must_use]
    pub fn new(
        player_a_handler: Box<dyn GameClient>,
        player_b_handler: Box<dyn GameClient>,
        game_state: GameState,
    ) -> Self {
        Self {
            player_a_handler,
            player_b_handler,
            game_state,
        }
    }

    /// Run a game until completion.
    pub async fn run_game(mut self) {
        let player_a_notifier = self.player_a_handler.make_notifier().await;
        let player_b_notifier = self.player_b_handler.make_notifier().await;

        let player_a_prompter = self.player_a_handler.make_prompter().await;
        let player_b_prompter = self.player_b_handler.make_prompter().await;

        let mut dispatcher = EventDispatcher::new(
            player_a_notifier,
            player_a_prompter,
            self.game_state.player_a_id(),
            player_b_notifier,
            player_b_prompter,
            self.game_state.player_b_id(),
        );

        let mut game_state = self.game_state;

        dispatcher.dispatch(StartGameEvent, &mut game_state).await;

        while !game_state.is_game_over() {
            let client = if game_state.cur_player_turn() == game_state.player_a_id() {
                self.player_a_handler.as_mut()
            } else {
                self.player_b_handler.as_mut()
            };

            GameRunner::player_take_turn_stage(client, &mut game_state, &mut dispatcher).await;
        }

        info!("Game is over.");
    }

    async fn player_take_turn_stage(
        handler_player: &mut dyn GameClient,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        let cur_player_id = game_state.cur_player_turn();
        info!("Turn starts for player: {:?}", cur_player_id);

        dispatcher
            .dispatch(TurnStartEvent(cur_player_id), game_state)
            .await;

        handler_player.on_turn_start(game_state).await;

        loop {
            if game_state.is_game_over() {
                info!("Game is over.");
                return;
            }

            info!("Getting next action from client.");
            let action = handler_player.next_action().await;

            let action: GameEvent = action.into();

            let turn_is_over = action.is_end_turn();

            dispatcher.dispatch(action, game_state).await;

            if turn_is_over {
                info!("Turn ends for player: {:?}", cur_player_id);
                return;
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::{
        game_agent::{
            tests::{MockTestPrompter, StubNotifier},
            ClientNotifier, Prompter,
        },
        game_state,
    };
    use async_trait::async_trait;
    use protocol::{client_actions::EndTurn, entities::PlayerId, from_client::ClientAction};
    use std::sync::Arc;

    struct TestClient {
        action_queue: Vec<ClientAction>,
        on_turn_start_queue: Vec<Box<dyn FnMut(&GameState) + Send + Sync>>,
        notifier: StubNotifier,
    }

    impl TestClient {
        fn new() -> Self {
            Self {
                action_queue: Vec::new(),
                on_turn_start_queue: Vec::new(),
                notifier: StubNotifier,
            }
        }

        fn add_action(&mut self, action: ClientAction) {
            self.action_queue.push(action);
        }

        fn add_turn_start_check(&mut self, check: Box<dyn FnMut(&GameState) + Send + Sync>) {
            self.on_turn_start_queue.push(check);
        }
    }

    #[async_trait]
    impl GameClient for TestClient {
        async fn on_turn_start(&mut self, game_state: &GameState) {
            if let Some(mut check) = self.on_turn_start_queue.pop() {
                check(game_state);
            }
        }

        async fn next_action(&mut self) -> ClientAction {
            self.action_queue
                .pop()
                .expect("No actions left in the queue")
        }

        async fn make_prompter(&self) -> Arc<dyn Prompter> {
            Arc::new(MockTestPrompter::new())
        }

        async fn make_notifier(&self) -> Arc<dyn ClientNotifier> {
            Arc::new(StubNotifier)
        }
    }

    #[test]
    pub fn gamerunner_when_game_run_expects_game_ends() {
        let _ = env_logger::builder().is_test(true).try_init();
        info!("starting...");
        let mut client_a = Box::new(TestClient::new());
        let mut client_b = Box::new(TestClient::new());

        let game_state = GameState::new(PlayerId::new(), PlayerId::new());

        for _ in 0..100 {
            client_a.add_action(ClientAction::EndTurn(EndTurn {
                player_id: game_state.player_a_id(),
            }));

            client_b.add_action(ClientAction::EndTurn(EndTurn {
                player_id: game_state.player_b_id(),
            }));
        }

        let runner = GameRunner::new(client_a, client_b, game_state);

        smol::block_on(async {
            runner.run_game().await;
        });
    }

    // #[test]
    // pub fn gamerunner_when_game_run_expects_game_ends() {
    //     let _ = env_logger::builder().is_test(true).try_init();
    //     let mut client_a = Box::new(TestClient::new());
    //     let mut client_b = Box::new(TestClient::new());
    //     let game_state = make_test_state();

    //     client_a.add_turn_start_check(Box::new(|game_state| {
    //         let anything_on_board = game_state
    //             .board()
    //             .all_characters_slots()
    //             .exclude_heroes()
    //             .creatures()
    //             .next();
    //         assert!(
    //             anything_on_board.is_none(),
    //             "Expected no creatures on board since none were ever played."
    //         );
    //     }));

    //     for _ in 0..100 {
    //         client_a.add_action(ClientAction::EndTurn(EndTurn {
    //             player_id: game_state.player_a_id(),
    //         }));

    //         client_b.add_action(ClientAction::EndTurn(EndTurn {
    //             player_id: game_state.player_b_id(),
    //         }));
    //     }

    //     let runner = GameRunner::new(client_a, client_b, game_state);

    //     smol::block_on(async {
    //         runner.run_game().await;
    //     });
    // }
}
