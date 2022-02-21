pub use id::*;

use super::{
    board::BoardPos, buff::BuffPlayerView, unit_card_definition::UnitCardDefinition,
    EntityPosition, EntityTypeId, HasId, IsEntity, PassiveEffectInstancePlayerView,
};
use serde::{Deserialize, Serialize};

/// A view of a creature card instance.
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct UnitCardInstance {
    //position: EntityPosition,
    definition: UnitCardDefinition,
    buffs: Vec<BuffPlayerView>,
    passive_effect: Option<PassiveEffectInstancePlayerView>,
    id: UnitCardInstanceId,
    attack: i32,
    health: i32,
    width: usize,
    state: Option<InstanceState>,
}

impl UnitCardInstance {
    pub fn new(
        id: UnitCardInstanceId,
        definition: UnitCardDefinition,
        buffs: Vec<BuffPlayerView>,
        passive_effect: Option<PassiveEffectInstancePlayerView>,
        //position: EntityPosition,
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
            //osition,
        }
    }

    /// Get a reference to the unit card instance's definition.
    pub fn definition(&self) -> &UnitCardDefinition {
        &self.definition
    }
}

impl HasId for UnitCardInstance {
    type IdType = UnitCardInstanceId;

    fn id(&self) -> Self::IdType {
        self.id
    }
}

impl IsEntity for UnitCardInstance {
    type IdType = UnitCardInstanceId;

    fn type_id() -> EntityTypeId {
        EntityTypeId::parse_str("896a090e-9efd-4cf1-aece-52fb7bb47344")
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum InstanceState {
    Pos(BoardPos),
    CreatureInstanceId(UnitCardInstanceId),
}

mod id {
    use std::fmt::Display;

    use crate::entities::{AsId, EntityId, Id};
    use serde::{Deserialize, Serialize};

    use super::UnitCardInstance;

    #[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
    pub struct UnitCardInstanceId(Id);

    impl UnitCardInstanceId {
        #[must_use]
        pub fn new() -> Self {
            Self(Id::new())
        }

        #[must_use]
        pub fn id(&self) -> Id {
            self.0
        }
    }

    impl From<Id> for UnitCardInstanceId {
        fn from(id: Id) -> Self {
            UnitCardInstanceId(id)
        }
    }

    impl AsId for UnitCardInstanceId {
        fn as_id(&self) -> Id {
            self.0
        }
    }

    impl EntityId for UnitCardInstanceId {
        type EntityType = UnitCardInstance;
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
