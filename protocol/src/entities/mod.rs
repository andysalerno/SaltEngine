mod board;
mod buff;
mod hand;
mod hideable;
mod id;
mod passive_effect;
mod player;
mod unit_card_definition;
mod unit_card_instance_view;

pub use board::*;
pub use buff::*;
pub use hand::*;
pub use hideable::*;
pub use id::*;
pub use passive_effect::*;
pub use player::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub use unit_card_definition::*;
pub use unit_card_instance_view::*;

/// A trait that indicates a type should be considered an `Entity`, and can be transformed into an `Entity`.
pub trait IsEntity: HasId + Serialize + DeserializeOwned + 'static {
    /// The type of `Id` that describes this entity.
    type IdType: EntityId;

    fn type_id() -> EntityTypeId;

    /// Creates an `Entity` representation of this object.
    fn as_entity(&self) -> Entity {
        Entity {
            id: self.id().as_id(),
            type_id: Self::type_id(),
            data: serde_json::to_string(self).unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Entity {
    pub id: Id,
    pub type_id: EntityTypeId,
    pub data: String,
}

#[derive(Debug, Eq, Hash, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct EntityTypeId(Id);

impl EntityTypeId {
    #[must_use]
    pub fn new() -> Self {
        Self(Id::new())
    }

    #[must_use]
    pub fn parse_str(s: &str) -> Self {
        Self(Id::parse_str(s))
    }
}

impl Default for EntityTypeId {
    fn default() -> Self {
        Self::new()
    }
}

impl AsId for EntityTypeId {
    fn as_id(&self) -> Id {
        self.0
    }
}
