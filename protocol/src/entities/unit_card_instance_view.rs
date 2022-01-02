pub use id::*;

use super::{
    board::BoardPos, buff::BuffPlayerView, unit_card_definition::UnitCardDefinitionPlayerView,
    PassiveEffectInstancePlayerView,
};
use serde::{Deserialize, Serialize};

/// A view of a creature card instance.
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct UnitCardInstancePlayerView {
    definition: UnitCardDefinitionPlayerView,
    buffs: Vec<BuffPlayerView>,
    passive_effect: Option<PassiveEffectInstancePlayerView>,
    id: UnitCardInstanceId,
    attack: i32,
    health: i32,
    width: usize,
    state: Option<InstanceState>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum InstanceState {
    Pos(BoardPos),
    CreatureInstanceId(UnitCardInstanceId),
}

mod id {
    use std::fmt::Display;

    use crate::entities::Id;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
    pub struct UnitCardInstanceId(Id);

    impl UnitCardInstanceId {
        #[must_use]
        pub fn new() -> Self {
            Self(Id::new())
        }
    }

    impl Default for UnitCardInstanceId {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Display for UnitCardInstanceId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use std::fmt::Debug;
            self.0.fmt(f)
        }
    }
}
