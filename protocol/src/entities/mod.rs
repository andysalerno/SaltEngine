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

use std::any::Any;

pub use crate::entities::id::*;
use ::id::Id;
pub use board::*;
pub use buff::*;
pub use hand::*;
pub use hero::*;
pub use hideable::*;
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
}

impl<T> From<T> for Entity
where
    T: IsEntity,
{
    fn from(entity_type: T) -> Self {
        Entity {
            id: entity_type.id().as_id(),
            type_id: T::type_id(),
            data: serde_json::to_value(entity_type).unwrap(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entity {
    id: Id,
    type_id: EntityTypeId,
    data: serde_json::Value,
}

// impl AsEntity for Entity {
//     fn as_entity(&self) -> Entity {
//         self.clone()
//     }
// }

impl Entity {
    pub fn as_json(&self) -> String {
        self.data.to_string()
    }

    pub fn update_property(&mut self, property_name: &str, next_value: impl Serialize) {
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

#[cfg(test)]
pub mod tests {
    use crate::entities::Entity;

    use super::{AsId, EntityId, EntityTypeId, HasId, Id, IsEntity};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct SimpleEntity {
        id: SimpleEntityId,
        string: String,
        int: i32,
        nested_entity: Option<Box<SimpleEntity>>,
    }

    #[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
    struct SimpleEntityId(Id);

    impl SimpleEntityId {
        fn new() -> Self {
            SimpleEntityId(Id::new())
        }
    }

    impl AsId for SimpleEntityId {
        fn as_id(&self) -> Id {
            self.0
        }
    }

    impl EntityId for SimpleEntityId {
        type EntityType = SimpleEntity;
    }

    impl HasId for SimpleEntity {
        type IdType = SimpleEntityId;

        fn id(&self) -> Self::IdType {
            self.id
        }
    }

    impl IsEntity for SimpleEntity {
        type IdType = SimpleEntityId;

        fn type_id() -> super::EntityTypeId {
            EntityTypeId::parse_str("d349f9d5-e181-4c0f-b327-cd7afff6ccee")
        }
    }

    #[test]
    pub fn entity_can_serialize_then_deserialize() {
        let id = SimpleEntityId::new();
        let entity = SimpleEntity {
            id,
            string: "something".into(),
            int: 42,
            nested_entity: None,
        };

        let e: Entity = entity.into();

        let unpacked: SimpleEntity = e.unpack();

        assert_eq!(id, unpacked.id);
        assert_eq!("something", &unpacked.string);
        assert_eq!(42, unpacked.int);
    }

    #[test]
    pub fn entity_can_serialize_then_deserialize_nested() {
        let id = SimpleEntityId::new();
        let nested_id = SimpleEntityId::new();
        let entity = SimpleEntity {
            id,
            string: "something".into(),
            int: 42,
            nested_entity: Some(Box::new(SimpleEntity {
                id: nested_id,
                string: "nested_string".into(),
                int: 43,
                nested_entity: None,
            })),
        };

        let e: Entity = entity.into();

        let unpacked: SimpleEntity = e.unpack();

        assert_eq!(nested_id, unpacked.nested_entity.as_ref().unwrap().id());
        assert_eq!(
            "nested_string",
            unpacked.nested_entity.unwrap().as_ref().string
        );
        assert_eq!(42, unpacked.int);
    }

    #[test]
    pub fn entity_can_update_string() {
        let id = SimpleEntityId::new();
        let entity = SimpleEntity {
            id,
            string: "something".into(),
            int: 42,
            nested_entity: None,
        };

        let mut e: Entity = entity.into();

        e.update_property("string", "or other");

        let unpacked: SimpleEntity = e.unpack();

        assert_eq!(&unpacked.string, "or other");
    }

    #[test]
    pub fn entity_can_update_i32() {
        let id = SimpleEntityId::new();
        let entity = SimpleEntity {
            id,
            string: "something".into(),
            int: 42,
            nested_entity: None,
        };

        let mut e: Entity = entity.into();

        e.update_property("int", 99);

        let unpacked: SimpleEntity = e.unpack();

        assert_eq!(unpacked.int, 99);
    }

    // #[test]
    // pub fn entity_can_update_nested() {
    //     let id = SimpleEntityId::new();
    //     let entity = SimpleEntity {
    //         id,
    //         string: "something".into(),
    //         int: 42,
    //         nested_entity: Some(Box::new(SimpleEntity {
    //             id: SimpleEntityId::new(),
    //             string: "nested_string".into(),
    //             int: 99,
    //             nested_entity: None,
    //         })),
    //     };

    //     let mut e = entity.as_entity();

    //     let unpacked: SimpleEntity = e.unpack();

    //     assert_eq!(&unpacked.string, "or other");
    // }
}
