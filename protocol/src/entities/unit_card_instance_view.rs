pub use id::*;

use super::{
    board::BoardPos, buff::BuffPlayerView, unit_card_definition::CreatureDefinition,
    EntityPosition, EntityTypeId, HasId, IsEntity, PassiveEffectInstancePlayerView,
};
use serde::{Deserialize, Serialize};

/// A view of a creature card instance.
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct CreatureInstance {
    definition: CreatureDefinition,
    buffs: Vec<BuffPlayerView>,
    passive_effect: Option<PassiveEffectInstancePlayerView>,
    id: CreatureInstanceId,
    attack: i32,
    health: i32,
    width: usize,
    state: Option<InstanceState>,
}

impl CreatureInstance {
    pub fn new(
        id: CreatureInstanceId,
        definition: CreatureDefinition,
        buffs: Vec<BuffPlayerView>,
        passive_effect: Option<PassiveEffectInstancePlayerView>,
    ) -> Self {
        let width = definition.row_width;
        let attack = definition.attack;
        let health = definition.health;

        Self {
            definition,
            buffs,
            passive_effect,
            id,
            attack,
            health,
            width,
            state: None,
        }
    }

    /// Get a reference to the unit card instance's definition.
    pub fn definition(&self) -> &CreatureDefinition {
        &self.definition
    }
}

impl HasId for CreatureInstance {
    type IdType = CreatureInstanceId;

    fn id(&self) -> Self::IdType {
        self.id
    }
}

impl IsEntity for CreatureInstance {
    type IdType = CreatureInstanceId;

    fn type_id() -> EntityTypeId {
        EntityTypeId::parse_str("896a090e-9efd-4cf1-aece-52fb7bb47344")
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum InstanceState {
    Pos(BoardPos),
    CreatureInstanceId(CreatureInstanceId),
}

mod id {
    use std::fmt::Display;

    use crate::entities::{AsId, EntityId, Id};
    use serde::{Deserialize, Serialize};

    use super::CreatureInstance;

    #[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
    pub struct CreatureInstanceId(Id);

    impl CreatureInstanceId {
        #[must_use]
        pub fn new() -> Self {
            Self(Id::new())
        }

        #[must_use]
        pub fn id(&self) -> Id {
            self.0
        }
    }

    impl From<Id> for CreatureInstanceId {
        fn from(id: Id) -> Self {
            CreatureInstanceId(id)
        }
    }

    impl AsId for CreatureInstanceId {
        fn as_id(&self) -> Id {
            self.0
        }
    }

    impl EntityId for CreatureInstanceId {
        type EntityType = CreatureInstance;
    }

    impl Default for CreatureInstanceId {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Display for CreatureInstanceId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use std::fmt::Debug;
            self.0.fmt(f)
        }
    }
}
