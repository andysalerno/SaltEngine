use crate::{
    game_agent::game_agent::{GameAgent, Prompter},
    game_logic::{cards::*, ClientGameEvent, EventDispatcher, GameEvent, StartGameEvent},
    game_state::{
        board::BoardPos, Deck, GameState, GameStatePlayerView, GameStateView, MakePlayerView,
        UnitCardInstance,
    },
};
use async_trait::async_trait;
use log::info;

/// A trait that defines the interaction between the GameRunner
/// and the client.
/// The GameRunner is the rules engine, and it will use the
/// GameRunnerHandler for each player client to alert that client
/// to events, and to receive input from the player client.
#[async_trait]
pub trait GameRunnerHandler: Send + Sync {
    async fn on_turn_start(&mut self, game_state: &GameState);
    async fn next_action(&mut self, game_state_view: GameStatePlayerView) -> ClientGameEvent;
    async fn make_prompter(&self) -> Box<dyn Prompter>;
}

pub struct GameRunnerZ {
    player_a_handler: Box<dyn GameRunnerHandler>,
    player_b_handler: Box<dyn GameRunnerHandler>,
    game_state: GameState,
}

impl GameRunnerZ {
    pub fn new(
        player_a_handler: Box<dyn GameRunnerHandler>,
        player_b_handler: Box<dyn GameRunnerHandler>,
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

        while !self.game_state.is_game_over() {
            let handler = if self.game_state.cur_player_turn() == self.game_state.player_a_id() {
                self.player_a_handler.as_mut()
            } else {
                self.player_b_handler.as_mut()
            };

            GameRunnerZ::player_take_turn_stage(handler, &mut self.game_state, &mut dispatcher)
                .await;
        }
    }

    async fn player_take_turn_stage(
        handler: &mut dyn GameRunnerHandler,
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

            let turn_is_over = match action {
                GameEvent::EndTurn(_) => true,
                _ => false,
            };

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

// To be deleted shortly
// pub struct GameRunner {
//     player_a: Box<dyn GameAgent>,
//     player_b: Box<dyn GameAgent>,
//     display: Box<dyn GameDisplay>,
//     game_state: GameState,
// }

// impl GameRunner {
//     pub fn new(
//         player_a: Box<dyn GameAgent>,
//         player_b: Box<dyn GameAgent>,
//         display: Box<dyn GameDisplay>,
//     ) -> Self {
//         let mut player_a_deck = {
//             let cards: Vec<UnitCardInstance> = (0..8)
//                 .flat_map(|_| {
//                     let cards = vec![
//                         RicketyCannon.make_instance(),
//                         Pawn.make_instance(),
//                         EmotionalSupportDog.make_instance(),
//                         ReallyBigRock.make_instance(),
//                         AttackDog.make_instance(),
//                         SleepingDog.make_instance(),
//                         PopcornVendor.make_instance(),
//                         PriestOfTheLowland.make_instance(),
//                         FraidyCat.make_instance(),
//                         OutdoorCat.make_instance(),
//                         IndoorCat.make_instance(),
//                     ];

//                     cards
//                 })
//                 .collect();

//             Deck::new(cards)
//         };

//         let mut player_b_deck = {
//             let cards: Vec<UnitCardInstance> = (0..8)
//                 .flat_map(|_| {
//                     let cards = vec![
//                         RicketyCannon.make_instance(),
//                         Pawn.make_instance(),
//                         EmotionalSupportDog.make_instance(),
//                         ReallyBigRock.make_instance(),
//                         AttackDog.make_instance(),
//                         SleepingDog.make_instance(),
//                         PopcornVendor.make_instance(),
//                         PriestOfTheLowland.make_instance(),
//                         FraidyCat.make_instance(),
//                         OutdoorCat.make_instance(),
//                         IndoorCat.make_instance(),
//                     ];

//                     cards
//                 })
//                 .collect();

//             Deck::new(cards)
//         };

//         player_a_deck.shuffle();
//         player_b_deck.shuffle();

//         let game_state =
//             GameState::initial_state(player_a.id(), player_a_deck, player_b.id(), player_b_deck);

//         Self {
//             player_a,
//             player_b,
//             display,
//             game_state,
//         }
//     }

//     pub fn run_game(&mut self) {
//         let a_prompter = self.player_a.make_prompter();
//         let b_prompter = self.player_b.make_prompter();
//         let mut dispatcher = EventDispatcher::new(a_prompter, b_prompter);
//         //let mut dispatcher = EventDispatcher::new();

//         dispatcher.dispatch(StartGameEvent, &mut self.game_state);

//         {
//             let player_a_id = self.game_state.player_a_id();
//             let player_b_id = self.game_state.player_b_id();
//             info!(
//                 "PlayerA starts with {} cards.\nPlayerB starts wtih {} cards.",
//                 self.game_state.deck(player_a_id).len(),
//                 self.game_state.deck(player_b_id).len()
//             );
//         }

//         while !self.game_state.is_game_over() {
//             let cur_player_id = self.game_state.cur_player_id();

//             info!("Player {:?} to take an action.", cur_player_id);
//             info!(
//                 "Available mana: {:?}",
//                 self.game_state.player_mana(cur_player_id)
//             );

//             self.display
//                 .display(&self.game_state.player_view(cur_player_id));

//             let cur_player = self.cur_player();
//             let action = cur_player.get_action(&self.game_state.player_view(cur_player.id()));
//             dispatcher.dispatch(action, &mut self.game_state);
//         }
//     }

//     fn cur_player(&self) -> &dyn GameAgent {
//         let cur_id = self.game_state.cur_player_id();

//         if cur_id == self.player_a.id() {
//             self.player_a.as_ref()
//         } else if cur_id == self.player_b.id() {
//             self.player_b.as_ref()
//         } else {
//             panic!("Unknown player id: {:?}", cur_id)
//         }
//     }
// }
