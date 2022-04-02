pub use self::id::HandId;

use super::{CreatureInstance, EntityTypeId, HasId, IsEntity, PlayerId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Hand {
    pub player_id: PlayerId,
    pub id: HandId,
    pub cards: Vec<CreatureInstance>,
}

impl Hand {
    pub fn new(player_id: PlayerId) -> Self {
        Self {
            player_id,
            id: HandId::new(),
            cards: Vec::new(),
        }
    }
}

impl IsEntity for Hand {
    type IdType = HandId;

    fn type_id() -> EntityTypeId {
        EntityTypeId::parse_str("0ab64181-26e9-4929-bbcb-8033f4949e78")
    }
}

impl HasId for Hand {
    type IdType = HandId;

    fn id(&self) -> Self::IdType {
        self.id
    }
}

mod id {
    use super::Hand;
    use crate::entities::{AsId, EntityId, Id};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
    pub struct HandId(Id);

    impl AsId for HandId {
        fn as_id(&self) -> Id {
            self.0
        }
    }

    impl EntityId for HandId {
        type EntityType = Hand;
    }

    impl HandId {
        #[must_use]
        pub fn new() -> Self {
            Self(Id::new())
        }
    }

    impl Default for HandId {
        fn default() -> Self {
            Self::new()
        }
    }
}
