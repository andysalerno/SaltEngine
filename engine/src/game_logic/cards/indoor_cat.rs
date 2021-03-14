use super::{CardDefinition, Position, UnitCardDefinition};
use crate::id::Id;

#[derive(Debug, Clone)]
pub struct IndoorCat;

impl IndoorCat {
    fn id(&self) -> Id {
        // id::parse("...")
        todo!()
    }
}

impl CardDefinition for IndoorCat {
    fn title(&self) -> &str {
        "Indoor Cat"
    }

    fn cost(&self) -> i32 {
        3
    }

    fn flavor_text(&self) -> &str {
        "todo"
    }

    fn text(&self) -> &str {
        "Front.
Hidden.
If revealed during
your turn, gains
+1/+1."
    }
}

impl UnitCardDefinition for IndoorCat {
    fn attack(&self) -> i32 {
        2
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

    fn is_hidden(&self) -> bool {
        true
    }
}
