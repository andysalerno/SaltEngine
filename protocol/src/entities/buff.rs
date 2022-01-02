pub use id::*;

use super::Id;
use serde::{Deserialize, Serialize};

/// A view of a buff.
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct BuffPlayerView {
    pub attack_amount: i32,
    pub health_amount: i32,
    pub source_id: BuffSourceId,
    pub instance_id: BuffInstanceId,
    pub definition_id: Id,
    pub is_from_passive: bool,
}

mod id {
    use crate::entities::{Id, PassiveEffectInstanceId, UnitCardInstanceId};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
    pub enum BuffSourceId {
        Passive(PassiveEffectInstanceId),
        CreatureInstance(UnitCardInstanceId),
        Other(Id),
    }

    #[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
    pub struct BuffInstanceId(Id);

    impl From<UnitCardInstanceId> for BuffSourceId {
        fn from(id: UnitCardInstanceId) -> Self {
            BuffSourceId::CreatureInstance(id)
        }
    }

    impl From<PassiveEffectInstanceId> for BuffSourceId {
        fn from(id: PassiveEffectInstanceId) -> Self {
            BuffSourceId::Passive(id)
        }
    }

    impl BuffInstanceId {
        #[must_use]
        pub fn new() -> Self {
            Self(Id::new())
        }
    }

    impl Default for BuffInstanceId {
        fn default() -> Self {
            Self::new()
        }
    }
}
