use super::PlayerId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum RowId {
    FrontRow,
    BackRow,
    Hero,
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

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoardPos {
    pub player_id: PlayerId,
    pub row_id: RowId,
    pub row_index: usize,
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
