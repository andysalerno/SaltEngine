use crate::{
    game_agent::game_agent::GameAgent,
    game_logic::{cards::*, EventDispatcher, GameEvent, StartGameEvent},
    game_state::{
        Deck, GameState, GameStatePlayerView, GameStateView, MakePlayerView, UnitCardInstance,
    },
};

/// A trait that defines the interaction between the GameRunner
/// and the client.
/// The GameRunner is the rules engine, and it will use the
/// GameRunnerHandler for each player client to alert that client
/// to events, and to receive input from the player client.
pub trait GameRunnerHandler: Send + Sync {
    fn on_turn_start(&self, game_state: &GameState);
    //fn has_ended_turn(&self) -> bool;
    fn next_action(&self) -> GameEvent;
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

    pub async fn run_game(self) {
        // let dispatcher = EventDispatcher::new(player_a_prompter, player_b_prompter)
        let dispatcher = EventDispatcher::new();

        while !self.game_state.is_game_over() {
            let handler = if self.game_state.cur_player_turn() == self.game_state.player_a_id() {
                self.player_a_handler.as_ref()
            } else {
                self.player_b_handler.as_ref()
            };

            GameRunnerZ::player_take_turn_stage(handler, &self.game_state).await;
        }
    }

    async fn player_take_turn_stage(handler: &dyn GameRunnerHandler, game_state: &GameState) {
        let cur_player_id = game_state.cur_player_id();
        println!("Turn starts for player: {:?}", cur_player_id);

        handler.on_turn_start(game_state);

        loop {
            let action = handler.next_action();

            if let GameEvent::EndTurn(__) = action {
                println!("Player has ended their turn.");
                break;
            }
        }
    }
}

pub trait GameDisplay {
    fn display(&mut self, game_state: &GameStatePlayerView);
}

pub struct GameRunner {
    player_a: Box<dyn GameAgent>,
    player_b: Box<dyn GameAgent>,
    display: Box<dyn GameDisplay>,
    game_state: GameState,
}

impl GameRunner {
    pub fn new(
        player_a: Box<dyn GameAgent>,
        player_b: Box<dyn GameAgent>,
        display: Box<dyn GameDisplay>,
    ) -> Self {
        let mut player_a_deck = {
            let cards: Vec<UnitCardInstance> = (0..8)
                .flat_map(|_| {
                    let cards = vec![
                        RicketyCannon.make_instance(),
                        Pawn.make_instance(),
                        EmotionalSupportDog.make_instance(),
                        ReallyBigRock.make_instance(),
                        AttackDog.make_instance(),
                        SleepingDog.make_instance(),
                        PopcornVendor.make_instance(),
                        PriestOfTheLowland.make_instance(),
                        FraidyCat.make_instance(),
                        OutdoorCat.make_instance(),
                        IndoorCat.make_instance(),
                    ];

                    cards
                })
                .collect();

            Deck::new(cards)
        };

        let mut player_b_deck = {
            let cards: Vec<UnitCardInstance> = (0..8)
                .flat_map(|_| {
                    let cards = vec![
                        RicketyCannon.make_instance(),
                        Pawn.make_instance(),
                        EmotionalSupportDog.make_instance(),
                        ReallyBigRock.make_instance(),
                        AttackDog.make_instance(),
                        SleepingDog.make_instance(),
                        PopcornVendor.make_instance(),
                        PriestOfTheLowland.make_instance(),
                        FraidyCat.make_instance(),
                        OutdoorCat.make_instance(),
                        IndoorCat.make_instance(),
                    ];

                    cards
                })
                .collect();

            Deck::new(cards)
        };

        player_a_deck.shuffle();
        player_b_deck.shuffle();

        let game_state =
            GameState::initial_state(player_a.id(), player_a_deck, player_b.id(), player_b_deck);

        Self {
            player_a,
            player_b,
            display,
            game_state,
        }
    }

    pub fn run_game(&mut self) {
        let a_prompter = self.player_a.make_prompter();
        let b_prompter = self.player_b.make_prompter();
        //let mut dispatcher = EventDispatcher::new(a_prompter, b_prompter);
        let mut dispatcher = EventDispatcher::new();

        dispatcher.dispatch(StartGameEvent, &mut self.game_state);

        {
            let player_a_id = self.game_state.player_a_id();
            let player_b_id = self.game_state.player_b_id();
            println!(
                "PlayerA starts with {} cards.\nPlayerB starts wtih {} cards.",
                self.game_state.deck(player_a_id).len(),
                self.game_state.deck(player_b_id).len()
            );
        }

        while !self.game_state.is_game_over() {
            let cur_player_id = self.game_state.cur_player_id();

            println!("Player {:?} to take an action.", cur_player_id);
            println!(
                "Available mana: {:?}",
                self.game_state.player_mana(cur_player_id)
            );

            self.display
                .display(&self.game_state.player_view(cur_player_id));

            let cur_player = self.cur_player();
            let action = cur_player.get_action(&self.game_state.player_view(cur_player.id()));
            dispatcher.dispatch(action, &mut self.game_state);
        }
    }

    fn cur_player(&self) -> &dyn GameAgent {
        let cur_id = self.game_state.cur_player_id();

        if cur_id == self.player_a.id() {
            self.player_a.as_ref()
        } else if cur_id == self.player_b.id() {
            self.player_b.as_ref()
        } else {
            panic!("Unknown player id: {:?}", cur_id)
        }
    }
}
