use super::{card_instance::UnitCardInstance, PlayerId, UnitCardInstanceId};

const BOARD_WIDTH: usize = 6;
const SLOTS_COUNT: usize = BOARD_WIDTH * 4;

enum PlayerAB {
    PlayerA,
    PlayerB,
}

#[derive(Debug)]
pub struct BoardSlot {
    pos: BoardPos,
    creature: Option<UnitCardInstance>,
}

impl BoardSlot {
    fn new(pos: BoardPos) -> Self {
        Self {
            pos,
            creature: None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RowId {
    FrontRow,
    BackRow,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BoardPos {
    pub player_id: PlayerId,
    pub row_id: RowId,
    pub row_index: usize,
}

impl BoardPos {
    pub fn new(player_id: PlayerId, row_id: RowId, row_index: usize) -> Self {
        Self {
            player_id,
            row_id,
            row_index,
        }
    }
}

struct BoardRow {
    size: usize,
    slots: Vec<Option<UnitCardInstance>>,
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

    pub fn front_row(&self) -> &[Option<UnitCardInstance>] {
        &self.front_row.slots
    }

    fn front_row_mut(&mut self) -> &mut [Option<UnitCardInstance>] {
        &mut self.front_row.slots
    }

    pub fn back_row(&self) -> &[Option<UnitCardInstance>] {
        &self.back_row.slots
    }

    fn back_row_mut(&mut self) -> &mut [Option<UnitCardInstance>] {
        &mut self.back_row.slots
    }

    pub fn iter(&self) -> impl Iterator<Item = &UnitCardInstance> {
        self.front_row()
            .iter()
            .chain(self.back_row())
            .filter_map(|i| i.as_ref())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut UnitCardInstance> {
        let front = &mut self.front_row.slots;
        let back = &mut self.back_row.slots;

        front.iter_mut().chain(back).filter_map(|i| i.as_mut())
    }
}

pub struct Board {
    player_a_id: PlayerId,
    player_b_id: PlayerId,
    slots: Vec<BoardSlot>,
}

impl Board {
    pub fn new(size: usize, player_a_id: PlayerId, player_b_id: PlayerId) -> Self {
        let mut slots = Vec::with_capacity(SLOTS_COUNT);

        // playber b
        for i in 0..BOARD_WIDTH {
            let pos = BoardPos::new(player_b_id, RowId::BackRow, i);
            slots.push(BoardSlot::new(pos));
        }
        for i in 0..BOARD_WIDTH {
            let pos = BoardPos::new(player_b_id, RowId::FrontRow, i);
            slots.push(BoardSlot::new(pos));
        }

        // playber a
        for i in 0..BOARD_WIDTH {
            let pos = BoardPos::new(player_a_id, RowId::FrontRow, i);
            slots.push(BoardSlot::new(pos));
        }
        for i in 0..BOARD_WIDTH {
            let pos = BoardPos::new(player_a_id, RowId::BackRow, i);
            slots.push(BoardSlot::new(pos));
        }

        Self {
            player_a_id,
            player_b_id,
            slots,
        }
    }

    fn player_ab(&self, player_id: PlayerId) -> PlayerAB {
        if player_id == self.player_a_id {
            PlayerAB::PlayerA
        } else if player_id == self.player_b_id {
            PlayerAB::PlayerB
        } else {
            panic!("Unknown player id: {:?}", player_id)
        }
    }

    /// The range for the entire board.
    fn board_range(&self, player_id: PlayerId) -> std::ops::Range<usize> {
        0..SLOTS_COUNT
    }

    /// The range for the given player.
    fn player_range(&self, player_id: PlayerId) -> std::ops::Range<usize> {
        match self.player_ab(player_id) {
            PlayerAB::PlayerB => front_half(0..SLOTS_COUNT),
            PlayerAB::PlayerA => end_half(0..SLOTS_COUNT),
        }
    }

    /// The range for the given player's row.
    fn row_range(&self, player_id: PlayerId, row_id: RowId) -> std::ops::Range<usize> {
        let player_range = self.player_range(player_id);

        match self.player_ab(player_id) {
            PlayerAB::PlayerB => match row_id {
                RowId::BackRow => front_half(player_range),
                RowId::FrontRow => end_half(player_range),
            },
            PlayerAB::PlayerA => match row_id {
                RowId::FrontRow => front_half(player_range),
                RowId::BackRow => end_half(player_range),
            },
        }
    }

    pub fn player_side(&self, player_id: PlayerId) -> &[BoardSlot] {
        &self.slots[self.player_range(player_id)]
    }

    pub fn player_side_mut(&mut self, player_id: PlayerId) -> &mut [BoardSlot] {
        &mut self.slots[self.player_range(player_id)]
    }

    pub fn get_at(&self, pos: BoardPos) -> Option<&UnitCardInstance> {
        let side = if pos.player_id == self.player_id {
            self.player_side()
        } else {
            self.opponent_side()
        };

        let row = if pos.row_id == RowId::BackRow {
            side.back_row()
        } else {
            side.front_row()
        };

        // start at pos.row_index, and work back, in case there's
        // a creature taking up multiple rows
        for i in (0..=pos.row_index).rev() {
            let distance = pos.row_index - i;
            let occupant = &row[i];

            if let Some(occupant) = occupant {
                if occupant.width() > distance {
                    return Some(occupant);
                } else {
                    return None;
                }
            }
        }

        return None;
    }

    pub fn set_at(&mut self, pos: BoardPos, card_instance: UnitCardInstance) {
        if let Some(existing) = self.get_at(pos) {
            panic!(
                "Could not set at pos {:?} due to existing occupant: {:?}",
                pos,
                existing.id()
            );
        }

        let side = if pos.player_id == self.player_id {
            self.player_side_mut()
        } else {
            self.opponent_side_mut()
        };

        let row = if pos.row_id == RowId::BackRow {
            side.back_row_mut()
        } else {
            side.front_row_mut()
        };

        row[pos.row_index] = Some(card_instance);
    }

    pub fn get_position_by_id(&self, id: UnitCardInstanceId) -> BoardPos {
        // Check opponent back
        {
            let found = self
                .opponent_side()
                .back_row()
                .iter()
                .enumerate()
                .filter(|i| match i.1 {
                    None => false,
                    Some(c) => c.id() == id,
                })
                .next();

            if let Some(found) = found {
                let (index, _) = found;
                let player_id = self.opponent_id;
                return BoardPos::new(player_id, RowId::BackRow, index);
            }
        }

        // Check opponent front
        {
            let found = self
                .opponent_side()
                .front_row()
                .iter()
                .enumerate()
                .filter(|i| match i.1 {
                    None => false,
                    Some(c) => c.id() == id,
                })
                .next();

            if let Some(found) = found {
                let (index, _) = found;
                let player_id = self.opponent_id;
                return BoardPos::new(player_id, RowId::FrontRow, index);
            }
        }

        // Check player back
        {
            let found = self
                .player_side()
                .back_row()
                .iter()
                .enumerate()
                .filter(|i| match i.1 {
                    None => false,
                    Some(c) => c.id() == id,
                })
                .next();

            if let Some(found) = found {
                let (index, _) = found;
                let player_id = self.player_id;
                return BoardPos::new(player_id, RowId::BackRow, index);
            }
        }

        // Check player front
        {
            let found = self
                .player_side()
                .front_row()
                .iter()
                .enumerate()
                .filter(|i| match i.1 {
                    None => false,
                    Some(c) => c.id() == id,
                })
                .next();

            if let Some(found) = found {
                let (index, _) = found;
                let player_id = self.player_id;
                return BoardPos::new(player_id, RowId::FrontRow, index);
            }
        }

        panic!("Id not found: {:?}", id);
    }

    pub fn get_by_id(&self, id: UnitCardInstanceId) -> &UnitCardInstance {
        self.iter()
            .filter(|i| i.id() == id)
            .next()
            .expect(&format!("No creature found with id {:?}", id))
    }

    /// An iterator over all unit instances on the entire board.
    pub fn iter(&self) -> impl Iterator<Item = &UnitCardInstance> {
        let opponent_side = self.opponent_side();
        let player_side = self.player_side();

        opponent_side.iter().chain(player_side.iter())
    }

    pub fn update_by_id(
        &mut self,
        id: UnitCardInstanceId,
        update: impl FnOnce(&mut UnitCardInstance),
    ) {
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

    pub fn remove_by_id(&mut self, id: UnitCardInstanceId) -> UnitCardInstance {
        let found_creature;
        if let Some(creature) = self
            .player_side_mut()
            .front_row_mut()
            .iter_mut()
            .filter(|i| match i {
                Some(c) => c.id() == id,
                None => false,
            })
            .next()
        {
            found_creature = creature;
        } else if let Some(creature) = self
            .player_side_mut()
            .back_row_mut()
            .iter_mut()
            .filter(|i| match i {
                Some(c) => c.id() == id,
                None => false,
            })
            .next()
        {
            found_creature = creature;
        } else if let Some(creature) = self
            .opponent_side_mut()
            .front_row_mut()
            .iter_mut()
            .filter(|i| match i {
                Some(c) => c.id() == id,
                None => false,
            })
            .next()
        {
            found_creature = creature;
        } else if let Some(creature) = self
            .opponent_side_mut()
            .back_row_mut()
            .iter_mut()
            .filter(|i| match i {
                Some(c) => c.id() == id,
                None => false,
            })
            .next()
        {
            found_creature = creature;
        } else {
            panic!("Creature not found on board with id: {:?}", id);
        }

        let mut next = None;
        std::mem::swap(&mut next, found_creature);
        return next.unwrap();
    }
}

fn front_half(ops: std::ops::Range<usize>) -> std::ops::Range<usize> {
    // ex: 10..20. includes 10 thru 19, with 10 total elements

    // 20 - 10 = 10
    let len = ops.end - ops.start;
    assert!(len % 2 == 0);

    // 10..15 (5 elements, 10-14 inclusive)
    ops.start..ops.start + len / 2
}

fn end_half(ops: std::ops::Range<usize>) -> std::ops::Range<usize> {
    let len = ops.end - ops.start;
    assert!(len % 2 == 0);
    ops.start + len / 2..ops.end
}
