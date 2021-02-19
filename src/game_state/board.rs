use super::{card_instance::UnitCardInstance, PlayerId, UnitCardInstanceId};

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
}

pub struct Board {
    player_side: BoardSide,
    opponent_side: BoardSide,
    player_id: PlayerId,
    opponent_id: PlayerId,
}

impl Board {
    pub fn new(size: usize, player_id: PlayerId, opponent_id: PlayerId) -> Self {
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

    pub fn player_side_id(&self, id: PlayerId) -> &BoardSide {
        match id {
            p if p == self.player_id => self.player_side(),
            p if p == self.opponent_id => self.opponent_side(),
            _ => panic!("Unknown player id {:?}", id),
        }
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
