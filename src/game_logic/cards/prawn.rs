use crate::id::Id;

use super::{CardDefinition, Position, UnitCardDefinition};

#[derive(Debug, Clone)]
pub struct Prawn;

impl Prawn {
    fn id(&self) -> Id {
        // id::parse("...")
        todo!()
    }
}

impl CardDefinition for Prawn {
    fn title(&self) -> &str {
        "Pawn"
    }

    fn cost(&self) -> i32 {
        1
    }

    fn flavor_text(&self) -> &str {
        "Just a lowly Pawn."
    }

    fn text(&self) -> &str {
        "Front"
    }
}

impl UnitCardDefinition for Prawn {
    fn attack(&self) -> i32 {
        1
    }

    fn health(&self) -> i32 {
        1
    }

    fn row_width(&self) -> usize {
        1
    }

    fn placeable_at(&self) -> Position {
        Position::Front
    }
}
