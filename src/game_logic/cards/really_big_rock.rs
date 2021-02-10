use crate::id::Id;

use super::{CardDefinition, UnitCardDefinition};

#[derive(Debug, Clone)]
pub struct ReallyBigRock;

impl ReallyBigRock {
    fn id(&self) -> Id {
        // id::parse("...")
        todo!()
    }
}

impl CardDefinition for ReallyBigRock {
    fn title(&self) -> &str {
        "Really Big Rock"
    }

    fn cost(&self) -> i32 {
        3
    }

    fn flavor_text(&self) -> &str {
        "Not just a boulder."
    }

    fn text(&self) -> &str {
        "Defender"
    }
}

impl UnitCardDefinition for ReallyBigRock {
    fn attack(&self) -> i32 {
        0
    }

    fn health(&self) -> i32 {
        4
    }

    fn row_width(&self) -> usize {
        2
    }

    fn is_defender(&self) -> bool {
        true
    }
}
