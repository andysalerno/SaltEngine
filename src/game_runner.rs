use crate::{
    game_agent::game_agent::GameAgent,
    game_logic::{cards::*, CreatureSetEvent, EventDispatcher, GameEvent, StartGameEvent},
    game_state::{
        board::{BoardPos, RowId},
        Deck, GameState,
    },
};

pub trait GameDisplay {
    fn display(&mut self, game_state: &GameState);
}

pub struct GameRunner {
    player_a: Box<dyn GameAgent>,
    player_b: Box<dyn GameAgent>,
    display: Box<dyn GameDisplay>,
    game_state: GameState,
    event_stack: Vec<GameEvent>,
}

impl GameRunner {
    pub fn new(
        player_a: Box<dyn GameAgent>,
        player_b: Box<dyn GameAgent>,
        display: Box<dyn GameDisplay>,
    ) -> Self {
        let player_a_deck = {
            let cards: Vec<Box<dyn UnitCardDefinition>> = (0..10)
                .flat_map(|_| {
                    let cards: Vec<Box<dyn UnitCardDefinition>> = vec![
                        RicketyCannon.boxed(),
                        Prawn.boxed(),
                        EmotionalSupportDog.boxed(),
                    ];

                    cards
                })
                .collect();

            Deck::new(cards)
        };

        let player_b_deck = {
            let cards: Vec<Box<dyn UnitCardDefinition>> = (0..10)
                .flat_map(|_| {
                    let cards: Vec<Box<dyn UnitCardDefinition>> = vec![
                        RicketyCannon.boxed(),
                        Prawn.boxed(),
                        EmotionalSupportDog.boxed(),
                    ];

                    cards
                })
                .collect();

            Deck::new(cards)
        };

        let game_state =
            GameState::initial_state(player_a.id(), player_a_deck, player_b.id(), player_b_deck);

        Self {
            player_a,
            player_b,
            display,
            game_state,
            event_stack: Vec::new(),
        }
    }

    pub fn run_game(&mut self) {
        let mut dispatcher = EventDispatcher::new();

        dispatcher.dispatch(StartGameEvent, &mut self.game_state);

        let player_a_id = self.game_state.player_a_id();
        let player_b_id = self.game_state.player_a_id();
        println!(
            "PlayerA starts with {} cards.\nPlayerB starts wtih {} cards.",
            self.game_state.deck(player_a_id).len(),
            self.game_state.deck(player_b_id).len()
        );

        while !self.game_state.is_game_over() {
            let cur_player_id = self.game_state.cur_player_id();

            println!("Start turn for player: {:?}", cur_player_id);
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
