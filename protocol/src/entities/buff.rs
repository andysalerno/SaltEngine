pub use id::*;

use super::{HasId, Id, IsEntity};
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

impl HasId for BuffPlayerView {
    type IdType = id::BuffInstanceId;

    fn id(&self) -> Self::IdType {
        self.instance_id
    }
}

impl IsEntity for BuffPlayerView {
    type IdType = id::BuffInstanceId;
}

mod id {
    use crate::entities::{AsId, EntityId, Id, PassiveEffectInstanceId, UnitCardInstanceId};
    use serde::{Deserialize, Serialize};

    use super::BuffPlayerView;

    #[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
    pub enum BuffSourceId {
        Passive(PassiveEffectInstanceId),
        CreatureInstance(UnitCardInstanceId),
        Other(Id),
    }

    #[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
    pub struct BuffInstanceId(Id);

    impl AsId for BuffInstanceId {
        fn as_id(&self) -> Id {
            self.0
        }
    }

    impl EntityId for BuffInstanceId {
        type EntityType = BuffPlayerView;
    }

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
