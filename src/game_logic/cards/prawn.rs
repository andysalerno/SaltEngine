use crate::id::HasId;
use crate::id::Id;

use super::{CardDefinition, UnitCardDefinition};

#[derive(Debug, Clone)]
pub struct Prawn;

impl HasId for Prawn {
    fn id(&self) -> Id {
        // id::parse("...")
        todo!()
    }
}

impl CardDefinition for Prawn {
    fn title(&self) -> &str {
        "Prawn"
    }
    fn cost(&self) -> i32 {
        1
    }

    fn flavor_text(&self) -> &str {
        "Just a lowly Prawn."
    }

    fn text(&self) -> &str {
        ""
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
}
