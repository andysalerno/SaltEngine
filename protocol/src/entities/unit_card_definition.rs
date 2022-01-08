use super::Position;
use serde::{Deserialize, Serialize};

/// The view of a creature card definition.
#[derive(Debug, Serialize, Clone, Deserialize)]
// pub struct UnitCardDefinitionPlayerView {
pub struct UnitCardDefinition {
    title: String,
    cost: i32,
    text: String,
    flavor_text: String,
    attack: i32,
    health: i32,
    row_width: usize,
    placeable_at: Position,
}
