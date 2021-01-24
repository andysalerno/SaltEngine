use super::board::Board;
use crate::id::Id;

pub struct GameState {
    player_b_id: Id,
    player_a_id: Id,
    cur_player_turn: Id,

    player_a_health: i32,
    player_b_health: i32,

    player_a_mana: u32,
    player_b_mana: u32,

    board: Box<Board>,
}

impl GameState {
    pub fn is_game_over(&self) -> bool {
        self.player_a_health <= 0 || self.player_b_health <= 0
    }
}
