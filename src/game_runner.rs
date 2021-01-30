use crate::{
    game_agent::game_agent::GameAgent,
    game_logic::{cards::prawn::Prawn, EventDispatcher, GameEvent, SummonCreatureEvent},
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
        self.event_stack
            .push(GameEvent::Summon(SummonCreatureEvent::new(
                Box::new(Prawn),
                BoardPos::new(self.player_a.id(), RowId::front_row, 2),
            )));

        while let Some(event) = self.event_stack.pop() {
            EventDispatcher::dispatch(event, &mut self.game_state);
        }

        self.display.display(&mut self.game_state);
        // while !self.game_state.is_game_over() {
        //     self.display.display(&mut self.game_state);
        //     let cur_player = self.get_cur_player();
        //     let action = cur_player.get_action(&self.game_state);
        //     EventDispatcher::dispatch(action, &mut self.game_state);
        // }
    }

    fn get_cur_player(&self) -> &Box<dyn GameAgent> {
        let cur_id = self.game_state.cur_player_turn();
        let player_a_id = self.player_a.id();
        let player_b_id = self.player_b.id();

        match cur_id {
            player_a_id => &self.player_a,
            player_b_id => &self.player_b,
        }
    }
}
