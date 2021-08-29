use crate::{
    game_agent::{ClientNotifier, Prompter},
    game_logic::{ClientActionEvent, EventDispatcher, GameEvent, StartGameEvent},
    game_state::{GameState, GameStatePlayerView, GameStateView, MakePlayerView},
};
use async_trait::async_trait;
use log::info;

/// A trait that defines the interaction between the GameRunner
/// and the client.
/// The GameRunner is the rules engine, and it will use the
/// GameRunnerHandler for each player client to alert that client
/// to events, and to receive input from the player client.
#[async_trait]
pub trait GameClient: Send + Sync {
    async fn on_turn_start(&mut self, game_state: &GameState);
    async fn next_action(&mut self, game_state_view: GameStatePlayerView) -> ClientActionEvent;
    async fn make_prompter(&self) -> Box<dyn Prompter>;
    async fn make_notifier(&self) -> Box<dyn ClientNotifier>;
    async fn observe_state_update(&mut self, game_state_view: GameStatePlayerView);
}

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
            let (client, other) = if game_state.cur_player_turn() == game_state.player_a_id() {
                (
                    self.player_a_handler.as_mut(),
                    self.player_b_handler.as_mut(),
                )
            } else {
                (
                    self.player_b_handler.as_mut(),
                    self.player_a_handler.as_mut(),
                )
            };

            GameRunner::player_take_turn_stage(client, other, &mut game_state, &mut dispatcher)
                .await;
        }
    }

    async fn player_take_turn_stage(
        handler_player: &mut dyn GameClient,
        handler_opponent: &mut dyn GameClient,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        let cur_player_id = game_state.cur_player_id();
        let opponent = game_state.other_player(cur_player_id);
        info!("Turn starts for player: {:?}", cur_player_id);

        handler_player.on_turn_start(game_state).await;

        loop {
            info!("Getting next action from client.");
            let action = handler_player
                .next_action(game_state.player_view(cur_player_id))
                .await;

            let action: GameEvent = action.into();

            let turn_is_over = action.is_end_turn();

            dispatcher.dispatch(action, game_state).await;

            handler_opponent
                .observe_state_update(game_state.player_view(opponent))
                .await;

            if turn_is_over {
                info!("Turn ends for player: {:?}", cur_player_id);
                return;
            }
        }
    }
}
