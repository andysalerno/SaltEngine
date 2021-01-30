use crate::id::Id;

use super::card_instance::{self, UnitCardBoardInstance};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RowId {
    front_row,
    back_row,
}

#[derive(Debug, Copy, Clone)]
pub struct BoardPos {
    player_id: Id,
    row_id: RowId,
    row_index: usize,
}

impl BoardPos {
    pub fn new(player_id: Id, row_id: RowId, row_index: usize) -> Self {
        Self {
            player_id,
            row_id,
            row_index,
        }
    }
}

struct BoardRow {
    size: usize,
    slots: Vec<Option<UnitCardBoardInstance>>,
}

impl BoardRow {
    pub fn new(size: usize) -> Self {
        let mut slots = Vec::new();

        (0..size).for_each(|_| slots.push(None));

        Self { size, slots }
    }
}

pub struct BoardSide {
    front_row: BoardRow,
    back_row: BoardRow,
}

impl BoardSide {
    pub fn new(size: usize) -> Self {
        Self {
            front_row: BoardRow::new(size),
            back_row: BoardRow::new(size),
        }
    }

    pub fn front_row(&self) -> &[Option<UnitCardBoardInstance>] {
        &self.front_row.slots
    }

    fn front_row_mut(&mut self) -> &mut [Option<UnitCardBoardInstance>] {
        &mut self.front_row.slots
    }

    pub fn back_row(&self) -> &[Option<UnitCardBoardInstance>] {
        &self.back_row.slots
    }

    fn back_row_mut(&mut self) -> &mut [Option<UnitCardBoardInstance>] {
        &mut self.back_row.slots
    }
}

pub struct Board {
    player_side: BoardSide,
    opponent_side: BoardSide,
    player_id: Id,
    opponent_id: Id,
}

impl Board {
    pub fn new(size: usize, player_id: Id, opponent_id: Id) -> Self {
        Self {
            player_side: BoardSide::new(size),
            opponent_side: BoardSide::new(size),
            player_id,
            opponent_id,
        }
    }

    pub fn player_side(&self) -> &BoardSide {
        &self.player_side
    }

    fn player_side_mut(&mut self) -> &mut BoardSide {
        &mut self.player_side
    }

    pub fn opponent_side(&self) -> &BoardSide {
        &self.opponent_side
    }

    fn opponent_side_mut(&mut self) -> &mut BoardSide {
        &mut self.opponent_side
    }

    pub fn get_at(&self, pos: BoardPos) -> &Option<UnitCardBoardInstance> {
        let side = if pos.player_id == self.player_id {
            self.player_side()
        } else {
            self.opponent_side()
        };

        let row = if pos.row_id == RowId::back_row {
            side.back_row()
        } else {
            side.front_row()
        };

        &row[pos.row_index]
    }

    pub fn set_at(&mut self, pos: BoardPos, card_instance: UnitCardBoardInstance) {
        let side = if pos.player_id == self.player_id {
            self.player_side_mut()
        } else {
            self.opponent_side_mut()
        };

        let row = if pos.row_id == RowId::back_row {
            side.back_row_mut()
        } else {
            side.front_row_mut()
        };

        row[pos.row_index] = Some(card_instance);
    }
}
