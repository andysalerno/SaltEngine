use super::card_instance::UnitCardBoardInstance;

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

pub struct Board {
    player_side: BoardSide,
    opponent_side: BoardSide,
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
        self.front_row.slots.as_slice()
    }

    pub fn back_row(&self) -> &[Option<UnitCardBoardInstance>] {
        &self.back_row.slots.as_slice()
    }
}

impl Board {
    pub fn new(size: usize) -> Self {
        Self {
            player_side: BoardSide::new(size),
            opponent_side: BoardSide::new(size),
        }
    }

    pub fn player_side(&self) -> &BoardSide {
        &self.player_side
    }

    pub fn opponent_side(&self) -> &BoardSide {
        &self.opponent_side
    }
}
