use crate::id::HasId;
use crate::id::Id;

use super::{CardDefinition, UnitCardDefinition};

#[derive(Debug, Clone)]
pub struct RicketyCannon;

impl HasId for RicketyCannon {
    fn id(&self) -> Id {
        // id::parse("...")
        todo!()
    }
}

impl CardDefinition for RicketyCannon {
    fn title(&self) -> &str {
        "Rickety cannon"
    }
    fn flavor_text(&self) -> &str {
        "yep"
    }

    fn cost(&self) -> i32 {
        2
    }
}

impl UnitCardDefinition for RicketyCannon {
    fn attack(&self) -> i32 {
        0
    }

    fn health(&self) -> i32 {
        3
    }

    fn row_width(&self) -> usize {
        3
    }
}
