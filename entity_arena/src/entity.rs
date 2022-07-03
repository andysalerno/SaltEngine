use crate::id::{EntityId, EntityTypeId};
use serde::{de::DeserializeOwned, Serialize};
use std::borrow::{Borrow, BorrowMut};

/// A marker trait indicating that the type is considered an entity.
/// Implies the type is serializeable / deserializeable.
pub trait IsEntity: Serialize + DeserializeOwned {
    /// The type ID of this entity type.
    fn entity_type_id() -> EntityTypeId;
}

pub type Value = serde_json::Value;

/// Represents an entity, with a globally unique instance ID, a type ID, and the raw entity value.
#[derive(Clone, Debug)]
pub struct Entity {
    id: EntityId,
    entity_type_id: EntityTypeId,
    data: Value,
}

impl Entity {
    pub fn new<T: IsEntity>(data: T) -> Self {
        Self {
            id: EntityId::new(),
            entity_type_id: T::entity_type_id(),
            data: serde_json::to_value(data).expect("The given entity could not be serialized"),
        }
    }

    pub fn into_typed<T: IsEntity>(self) -> TypedEntity<T, Value> {
        self.assert_type_id_matches::<T>();
        TypedEntity {
            id: self.id,
            data: self.data,
            _phantom: std::marker::PhantomData::default(),
        }
    }

    pub fn as_typed_mut<T: IsEntity>(&mut self) -> TypedEntity<T, &mut Value> {
        self.assert_type_id_matches::<T>();
        TypedEntity {
            id: self.id,
            data: &mut self.data,
            _phantom: std::marker::PhantomData::default(),
        }
    }

    pub fn as_typed<T: IsEntity>(&self) -> TypedEntity<T, &Value> {
        self.assert_type_id_matches::<T>();
        TypedEntity {
            id: self.id,
            data: &self.data,
            _phantom: std::marker::PhantomData::default(),
        }
    }

    pub fn id(&self) -> EntityId {
        self.id
    }

    pub fn entity_type_id(&self) -> EntityTypeId {
        self.entity_type_id
    }

    fn assert_type_id_matches<T: IsEntity>(&self) {
        if self.entity_type_id() != T::entity_type_id() {
            panic!(
                "Expected entity {:?} to have type id {:?} but had type id {:?}",
                self.id(),
                T::entity_type_id(),
                self.entity_type_id()
            );
        }
    }
}

pub struct TypedEntity<T: IsEntity, I> {
    id: EntityId,
    data: I,
    _phantom: std::marker::PhantomData<T>,
}

/// A handle to an entity that can represent the entity as its true type.
impl<T, I> TypedEntity<T, I>
where
    T: IsEntity,
    I: Borrow<Value>,
{
    pub fn get<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        // todo - surely possible without cloning?
        let local: T = serde_json::from_value(self.data.borrow().clone()).unwrap();
        f(&local)
    }

    pub fn id(&self) -> EntityId {
        self.id
    }
}

impl<T, I> TypedEntity<T, I>
where
    T: IsEntity,
    I: BorrowMut<Value>,
{
    pub fn get_mut(&mut self, f: impl FnOnce(&mut T)) {
        // todo - surely possible without cloning?
        let mut local: T = serde_json::from_value(self.data.borrow().clone()).unwrap();
        f(&mut local);

        *self.data.borrow_mut() = serde_json::to_value(local).unwrap();
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::id::{EntityId, EntityTypeId};

    use super::{Entity, IsEntity};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub(crate) struct TestEntity {
        id: EntityId,
        s: String,
        i: i32,
        t: Option<usize>,
        ne: Option<NestedEntity>,
    }

    impl TestEntity {
        pub(crate) fn new() -> Self {
            Self {
                id: EntityId::new(),
                s: String::new(),
                i: 0,
                t: None,
                ne: None,
            }
        }

        pub fn i32_val(&self) -> i32 {
            self.i
        }

        pub fn set_i32_val(&mut self, next: i32) {
            self.i = next;
        }
    }

    impl IsEntity for TestEntity {
        fn entity_type_id() -> crate::id::EntityTypeId {
            EntityTypeId::parse_str("d85d8676-9c49-464c-8d14-4bb7d76f9c57")
        }
    }

    #[derive(Default, Serialize, Deserialize)]
    pub(crate) struct NestedEntity {
        s: String,
        i: i32,
        t: Option<usize>,
    }

    #[derive(Serialize, Deserialize)]
    pub(crate) struct AnotherTestEntity;

    impl IsEntity for AnotherTestEntity {
        fn entity_type_id() -> EntityTypeId {
            EntityTypeId::parse_str("25068336-7a85-42cf-bf2f-d168bfb81692")
        }
    }

    #[test]
    fn test_entity_can_be_stored_as_entity() {
        let test_entity = TestEntity::new();

        let _entity = Entity::new(test_entity);
    }

    #[test]
    fn test_entity_can_be_read_as_entity() {
        let mut test_entity = TestEntity::new();
        test_entity.set_i32_val(99);

        let entity = Entity::new(test_entity);

        let read = entity.into_typed::<TestEntity>().get(|e| e.i);

        assert_eq!(99, read);
    }

    #[test]
    fn test_entity_can_be_updated_as_entity() {
        let test_entity = TestEntity::new();

        let entity = Entity::new(test_entity);

        let mut typed_entity = entity.into_typed::<TestEntity>();

        typed_entity.get_mut(|e| e.i = 100);
        let read = typed_entity.get(|e| e.i);

        assert_eq!(100, read);
    }

    #[test]
    fn test_entity_as_typed_can_be_read() {
        let mut test_entity = TestEntity::new();

        test_entity.set_i32_val(99);

        let entity = Entity::new(test_entity);

        let typed_entity = entity.as_typed::<TestEntity>();

        let read = typed_entity.get(|e| e.i);

        assert_eq!(99, read);
    }

    #[test]
    fn test_entity_as_typed_mut_can_be_updated() {
        let test_entity = TestEntity::new();

        let mut entity = Entity::new(test_entity);

        let mut typed_entity = entity.as_typed_mut::<TestEntity>();

        typed_entity.get_mut(|e| e.i = 100);
        let read = typed_entity.get(|e| e.i);

        assert_eq!(100, read);
    }
}
