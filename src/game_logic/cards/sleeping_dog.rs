use crate::{
    game_logic::{
        buff::{Buff, BuffSourceId},
        BuffInstanceId,
    },
    game_state::{board::RowId, UnitCardInstanceId},
    id::Id,
};
use crate::{
    game_logic::{passive_effect::PassiveEffectInstanceId, PassiveEffectDefinition},
    game_state::GameState,
};

use super::{CardDefinition, Position, UnitCardDefinition};

#[derive(Debug, Clone)]
pub struct SleepingDog;

impl SleepingDog {
    fn id(&self) -> Id {
        // id::parse("...")
        todo!()
    }
}

impl CardDefinition for SleepingDog {
    fn title(&self) -> &str {
        "Sleeping Dog"
    }

    fn cost(&self) -> i32 {
        3
    }

    fn flavor_text(&self) -> &str {
        "todo"
    }

    fn text(&self) -> &str {
        "Back
If Sleeping Dog takes damage,
move it to the front row
if possible, and +7 attack."
    }
}

impl UnitCardDefinition for SleepingDog {
    fn attack(&self) -> i32 {
        0
    }

    fn health(&self) -> i32 {
        3
    }

    fn row_width(&self) -> usize {
        2
    }

    fn placeable_at(&self) -> Position {
        Position::Back
    }
}
