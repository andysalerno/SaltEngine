use crate::{
    game_agent::game_agent::GameAgent, game_logic::EventDispatcher, game_state::GameState,
};

pub struct GameRunner {
    player_a: Box<dyn GameAgent>,
    player_b: Box<dyn GameAgent>,
    game_state: GameState,
}

impl GameRunner {
    pub fn new(player_a: Box<dyn GameAgent>, player_b: Box<dyn GameAgent>) -> Self {
        let game_state = GameState::new(player_a.id(), player_b.id());

        Self {
            player_a,
            player_b,
            game_state,
        }
    }

    pub fn run_game(&mut self) {
        while !self.game_state.is_game_over() {
            let cur_player = self.get_cur_player();
            let action = cur_player.get_action(&self.game_state);
            EventDispatcher::dispatch(action, &mut self.game_state);
        }
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
