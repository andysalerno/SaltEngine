use crate::id::Id;

use super::{CardDefinition, Position, UnitCardDefinition};

#[derive(Debug, Clone)]
pub struct AttackDog;

impl AttackDog {
    fn id(&self) -> Id {
        // id::parse("...")
        todo!()
    }
}

impl CardDefinition for AttackDog {
    fn title(&self) -> &str {
        "Attack Dog"
    }

    fn cost(&self) -> i32 {
        3
    }

    fn flavor_text(&self) -> &str {
        "todo"
    }

    fn text(&self) -> &str {
        ""
    }
}

impl UnitCardDefinition for AttackDog {
    fn attack(&self) -> i32 {
        5
    }

    fn health(&self) -> i32 {
        3
    }

    fn row_width(&self) -> usize {
        1
    }

    fn placeable_at(&self) -> Position {
        Position::Front
    }
}
