use crate::{
    game_agent::game_agent::Prompter,
    game_logic::{ClientGameEvent, EventDispatcher, GameEvent, StartGameEvent},
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
    async fn next_action(&mut self, game_state_view: GameStatePlayerView) -> ClientGameEvent;
    async fn make_prompter(&self) -> Box<dyn Prompter>;
}

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
        let player_a_prompter = self.player_a_handler.make_prompter().await;
        let player_b_prompter = self.player_b_handler.make_prompter().await;

        let mut dispatcher = EventDispatcher::new(player_a_prompter, player_b_prompter);

        let mut game_state = self.game_state;

        dispatcher.dispatch(StartGameEvent, &mut game_state);

        while !game_state.is_game_over() {
            let client = if game_state.cur_player_turn() == game_state.player_a_id() {
                self.player_a_handler.as_mut()
            } else {
                self.player_b_handler.as_mut()
            };

            GameRunner::player_take_turn_stage(client, &mut game_state, &mut dispatcher).await;
        }
    }

    async fn player_take_turn_stage(
        handler: &mut dyn GameClient,
        game_state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        let cur_player_id = game_state.cur_player_id();
        info!("Turn starts for player: {:?}", cur_player_id);

        handler.on_turn_start(game_state).await;

        loop {
            info!("Getting next action from client.");
            let action = handler
                .next_action(game_state.player_view(cur_player_id))
                .await;
            let action: GameEvent = action.into();

            let turn_is_over = action.is_end_turn();

            dispatcher.dispatch(action, game_state);

            if turn_is_over {
                info!("Turn ends for player: {:?}", cur_player_id);
                return;
            }
        }
    }
}

pub trait GameDisplay {
    fn display(&mut self, game_state: &GameStatePlayerView);
}
