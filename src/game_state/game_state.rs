use super::{
    board::{Board, BoardPos},
    UnitCardBoardInstance,
};
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

const BOARD_LEN: usize = 6;

impl GameState {
    pub fn is_game_over(&self) -> bool {
        self.player_a_health <= 0 || self.player_b_health <= 0
    }

    pub fn new(player_a_id: Id, player_b_id: Id) -> Self {
        Self {
            player_a_id,
            player_b_id,
            cur_player_turn: player_a_id,
            player_a_health: 30,
            player_b_health: 30,
            player_a_mana: 0,
            player_b_mana: 0,
            board: Box::new(Board::new(BOARD_LEN, player_a_id, player_b_id)),
        }
    }

    pub fn cur_player_turn(&self) -> Id {
        self.cur_player_turn
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    pub fn get_at(&self, pos: BoardPos) -> Option<&UnitCardBoardInstance> {
        self.board.get_at(pos)
    }

    pub fn set_at(&mut self, pos: BoardPos, card_instance: UnitCardBoardInstance) {
        self.board.set_at(pos, card_instance)
    }

    pub fn player_a_id(&self) -> Id {
        self.player_a_id
    }

    pub fn player_b_id(&self) -> Id {
        self.player_b_id
    }

    pub fn get_by_id(&self, id: Id) -> &UnitCardBoardInstance {
        self.board.get_by_id(id)
    }
}
