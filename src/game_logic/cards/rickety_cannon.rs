use crate::id::Id;

use super::{CardDefinition, UnitCardDefinition};

#[derive(Debug, Clone)]
pub struct RicketyCannon;

impl RicketyCannon {
    fn id(&self) -> Id {
        // id::parse("...")
        todo!()
    }
}

impl CardDefinition for RicketyCannon {
    fn title(&self) -> &str {
        "Rickety cannon"
    }
    fn cost(&self) -> i32 {
        2
    }

    fn flavor_text(&self) -> &str {
        "yep"
    }

    fn text(&self) -> &str {
        ""
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
