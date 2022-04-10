use entity_arena::id::Id;
use serde::{Deserialize, Serialize};

use super::creature_instance::Boon;

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

/// The view of a creature card definition.
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CreatureDefinition {
    definition_id: CreatureDefinitionId,
    title: String,
    cost: i32,
    text: String,
    flavor_text: String,
    attack: i32,
    health: i32,
    row_width: usize,
    placeable_at: Position,
    boons: Vec<Boon>,
}

impl CreatureDefinition {}

#[derive(Copy, Clone, PartialEq, Debug, Eq, Hash, Serialize, Deserialize)]
pub struct CreatureDefinitionId(Id);

impl CreatureDefinitionId {
    pub fn new() -> Self {
        Self(Id::new())
    }
}
