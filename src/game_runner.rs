use crate::{
    game_agent::game_agent::GameAgent,
    game_logic::{cards::*, EventDispatcher, GameEvent, SummonCreatureEvent},
    game_state::{
        board::{BoardPos, RowId},
        GameState,
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
        let game_state = GameState::new(player_a.id(), player_b.id());

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

        while !self.game_state.is_game_over() {
            let cur_player_id = self.game_state.cur_player_turn();

            println!("Start turn for player: {:?}", cur_player_id);
            println!(
                "Available mana: {:?}",
                self.game_state.player_mana(cur_player_id)
            );

            self.display.display(&mut self.game_state);

            let cur_player = self.get_cur_player();
            let action = cur_player.get_action(&self.game_state);
            dispatcher.dispatch(action, &mut self.game_state);
        }
    }

    fn get_cur_player(&self) -> &dyn GameAgent {
        let cur_id = self.game_state.cur_player_turn();

        if cur_id == self.player_a.id() {
            self.player_a.as_ref()
        } else if cur_id == self.player_b.id() {
            self.player_b.as_ref()
        } else {
            panic!("Unknown player id: {:?}", cur_id)
        }
    }
}
