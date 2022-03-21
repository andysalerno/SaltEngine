use super::Position;
use serde::{Deserialize, Serialize};

/// The view of a creature card definition.
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct UnitCardDefinition {
    pub title: String,
    pub cost: i32,
    pub text: String,
    pub flavor_text: String,
    pub attack: i32,
    pub health: i32,
    pub row_width: usize,
    pub placeable_at: Position,
}

impl UnitCardDefinition {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            cost: 0,
            text: String::new(),
            flavor_text: String::new(),
            attack: 0,
            health: 1,
            row_width: 1,
            placeable_at: Position::Either,
        }
    }
}
