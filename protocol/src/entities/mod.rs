mod board;
mod buff;
mod hand;
mod hero;
mod hideable;
mod id;
mod passive_effect;
mod player;
mod unit_card_definition;
mod unit_card_instance_view;

pub use board::*;
pub use buff::*;
pub use hand::*;
pub use hero::*;
pub use hideable::*;
pub use id::*;
pub use passive_effect::*;
pub use player::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
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
            data: serde_json::to_value(self).unwrap(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entity {
    id: Id,
    type_id: EntityTypeId,
    data: serde_json::Value,
}

impl Entity {
    pub fn as_json(&self) -> String {
        self.data.to_string()
    }

    pub fn update_property(&mut self, property_name: &str, next_value: &str) {
        let got = self
            .data
            .get_mut(property_name)
            .expect("Expected to find the given property on the entity, but did not.");

        *got = json!(next_value);
    }

    #[must_use]
    pub fn unpack_copy<T: IsEntity>(&self) -> T {
        {
            let requested_type_id = T::type_id();
            let my_type_id = self.type_id();
            if T::type_id() != self.type_id {
                panic!("A request was made to deserialize an entity with type id {my_type_id:#?} as a different type with id {requested_type_id:#?}");
            }
        }

        serde_json::from_value::<T>(self.data.clone())
            .expect("Expected serialization to succeed, assuming the given type is correct")
    }

    #[must_use]
    pub fn unpack<T: IsEntity>(self) -> T {
        {
            let requested_type_id = T::type_id();
            let my_type_id = self.type_id();
            if T::type_id() != self.type_id {
                panic!("A request was made to deserialize an entity with type id {my_type_id:#?} as a different type with id {requested_type_id:#?}");
            }
        }

        serde_json::from_value::<T>(self.data)
            .expect("Expected serialization to succeed, assuming the given type is correct")
    }

    /// Get the entity's id.
    #[must_use]
    pub fn id(&self) -> Id {
        self.id
    }

    /// Get the entity's type id.
    #[must_use]
    pub fn type_id(&self) -> EntityTypeId {
        self.type_id
    }
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
