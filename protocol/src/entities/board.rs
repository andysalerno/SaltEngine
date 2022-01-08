use std::collections::HashMap;

use super::{PlayerId, UnitCardInstance};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BoardPos {
    pub player_id: PlayerId,
    pub row_id: RowId,
    pub row_index: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RowId {
    FrontRow,
    BackRow,
    Hero,
}

/// Describes which board positions
/// this creature card may occupy.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Position {
    /// The front side of the board.
    Front,

    /// The back side of the board.
    Back,

    /// Either the front or the back sides of the board.
    Either,
}

impl RowId {
    #[must_use]
    pub fn is_back(&self) -> bool {
        matches!(self, RowId::BackRow)
    }

    #[must_use]
    pub fn is_front(&self) -> bool {
        matches!(self, RowId::FrontRow)
    }

    #[must_use]
    pub fn is_hero(&self) -> bool {
        matches!(self, RowId::Hero)
    }
}

impl BoardPos {
    #[must_use]
    pub fn new(player_id: PlayerId, row_id: RowId, row_index: usize) -> Self {
        Self {
            player_id,
            row_id,
            row_index,
        }
    }

    #[must_use]
    pub fn hero_pos(player_id: PlayerId) -> Self {
        Self {
            player_id,
            row_id: RowId::Hero,
            row_index: 0,
        }
    }

    #[must_use]
    pub fn row(&self) -> RowId {
        self.row_id
    }
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct BoardPlayerView {
    player_a_id: PlayerId,
    player_b_id: PlayerId,
    player_a_hero: BoardSlotPlayerView,
    player_b_hero: BoardSlotPlayerView,
    slots: Vec<BoardSlotPlayerView>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct BoardSlotPlayerView {
    pos: BoardPos,
    creature: Option<UnitCardInstance>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Board {
    slots_per_row: usize,
    entities_by_pos: HashMap<BoardPos, UnitCardInstance>,
}
