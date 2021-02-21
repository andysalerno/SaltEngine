use crate::{
    game_agent::game_agent::GameAgent,
    game_logic::{cards::*, EventDispatcher, StartGameEvent},
    game_state::{Deck, GameState, UnitCardInstance},
};

pub trait GameDisplay {
    fn display(&mut self, game_state: &GameState);
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
        let card_set: Vec<Box<dyn UnitCardDefinition>> = vec![
            Box::new(Prawn),
            Box::new(RicketyCannon),
            Box::new(EmotionalSupportDog),
            Box::new(ReallyBigRock),
            Box::new(AttackDog),
            Box::new(SleepingDog),
        ];

        let mut player_a_deck = {
            let cards: Vec<UnitCardInstance> = (0..8)
                .flat_map(|_| {
                    let cards = vec![
                        RicketyCannon.make_instance(),
                        Prawn.make_instance(),
                        EmotionalSupportDog.make_instance(),
                        ReallyBigRock.make_instance(),
                    ];

                    cards
                })
                .collect();

            let mut cards = Vec::new();

            while cards.len() < 30 {
                for definition in &card_set {
                    cards.push(definition.make_instance());
                }
            }

            Deck::new(cards)
        };

        let mut player_b_deck = {
            let cards: Vec<UnitCardInstance> = (0..8)
                .flat_map(|_| {
                    let cards = vec![
                        RicketyCannon.make_instance(),
                        Prawn.make_instance(),
                        EmotionalSupportDog.make_instance(),
                        ReallyBigRock.make_instance(),
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
        let mut dispatcher = EventDispatcher::new(a_prompter, b_prompter);

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

            self.display.display(&mut self.game_state);

            let cur_player = self.cur_player();
            let action = cur_player.get_action(&self.game_state);
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
