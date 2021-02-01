use std::fs::create_dir;

use crate::id::{HasId, Id};

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

    pub fn get_at(&self, pos: BoardPos) -> Option<&UnitCardBoardInstance> {
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

        row[pos.row_index].as_ref()
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

        if row[pos.row_index].is_some() {
            panic!(
                "Cannot place at a position with an existing instance: {:?}",
                pos
            );
        }

        row[pos.row_index] = Some(card_instance);
    }

    pub fn get_by_id(&self, id: Id) -> &UnitCardBoardInstance {
        let opponent_side = self.opponent_side();
        let player_side = self.player_side();

        let all_slots_iter = opponent_side
            .back_row()
            .iter()
            .chain(opponent_side.front_row())
            .chain(player_side.front_row())
            .chain(player_side.back_row());

        for slot in all_slots_iter {
            if let Some(creature) = slot {
                if creature.id() == id {
                    return &creature;
                }
            }
        }

        panic!("creature by id not found")
    }

    pub fn get_by_id_mut<'a>(&'a mut self, id: Id) -> &'a mut UnitCardBoardInstance {
        // {
        //     let player_side = self.player_side_mut();
        //     let front_row = player_side.front_row_mut();

        //     if let Some(creature) = front_row.first_mut().unwrap_or(&mut None) {
        //         return creature;
        //     }
        // }

        // {
        //     let opponent_side = self.opponent_side_mut();
        // }

        panic!()
        // {
        //     let found = self
        //         .player_side_mut()
        //         .back_row_mut()
        //         .iter_mut()
        //         .filter_map(|i| i.as_mut())
        //         .find(|i| i.id() == id);

        //     if let Some(creature) = found {
        //         return creature;
        //     }
        // }
        // {
        //     let found = self
        //         .player_side_mut()
        //         .front_row_mut()
        //         .iter_mut()
        //         .filter_map(|i| i.as_mut())
        //         .find(|i| i.id() == id);

        //     if let Some(creature) = found {
        //         return creature;
        //     }
        // }
    }

    pub fn update_by_id(&mut self, id: Id, update: impl FnOnce(&mut UnitCardBoardInstance)) {
        if let Some(mut creature) = self
            .player_side_mut()
            .front_row_mut()
            .iter_mut()
            .filter_map(|i| i.as_mut())
            .filter(|i| i.id() == id)
            .next()
        {
            update(&mut creature);
        } else if let Some(mut creature) = self
            .player_side_mut()
            .back_row_mut()
            .iter_mut()
            .filter_map(|i| i.as_mut())
            .filter(|i| i.id() == id)
            .next()
        {
            update(&mut creature);
        } else if let Some(mut creature) = self
            .opponent_side_mut()
            .front_row_mut()
            .iter_mut()
            .filter_map(|i| i.as_mut())
            .filter(|i| i.id() == id)
            .next()
        {
            update(&mut creature);
        } else if let Some(mut creature) = self
            .opponent_side_mut()
            .back_row_mut()
            .iter_mut()
            .filter_map(|i| i.as_mut())
            .filter(|i| i.id() == id)
            .next()
        {
            update(&mut creature);
        }
    }
}
