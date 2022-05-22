use super::creature_instance::Boon;
use serde::{Deserialize, Serialize};

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
