use crate::id::Id;
use serde::{de::DeserializeOwned, Serialize};

/// A marker trait indicating that the tyep is considered an entity.
/// Implies the type is serializeable / deserializeable.
pub trait IsEntity: Serialize + DeserializeOwned {}

#[derive(Clone, Debug)]
pub struct Entity {
    id: Id,
    data: serde_json::Value,
}

impl Entity {
    pub fn new(data: impl IsEntity) -> Self {
        Self {
            id: Id::new(),
            data: serde_json::to_value(data).expect("The given entity could not be serialized"),
        }
    }

    pub fn typed<T: IsEntity>(self) -> TypedEntity<T> {
        TypedEntity::<T> {
            id: self.id,
            data: self.data,
            _phantom: std::marker::PhantomData::default(),
        }
    }
}

pub struct TypedEntity<T: IsEntity> {
    id: Id,
    data: serde_json::Value,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: IsEntity> TypedEntity<T> {
    pub fn get<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        let local: T = serde_json::from_value(self.data.clone()).unwrap();
        f(&local)
    }

    pub fn get_mut(&mut self, f: impl FnOnce(&mut T)) {
        let mut local: T = serde_json::from_value(self.data.clone()).unwrap();
        f(&mut local);

        self.data = serde_json::to_value(local).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::{Entity, IsEntity};
    use serde::{Deserialize, Serialize};

    #[derive(Default, Serialize, Deserialize)]
    struct TestEntity {
        s: String,
        i: i32,
        t: Option<usize>,
        ne: Option<NestedEntity>,
    }

    impl IsEntity for TestEntity {}

    #[derive(Default, Serialize, Deserialize)]
    struct NestedEntity {
        s: String,
        i: i32,
        t: Option<usize>,
    }

    #[test]
    fn test_entity_can_be_stored_as_entity() {
        let test_entity = TestEntity::default();

        let _entity = Entity::new(test_entity);
    }

    #[test]
    fn test_entity_can_be_read_as_entity() {
        let test_entity = TestEntity {
            i: 99,
            ..Default::default()
        };

        let entity = Entity::new(test_entity);

        let read = entity.typed::<TestEntity>().get(|e| e.i);

        assert_eq!(99, read);
    }

    #[test]
    fn test_entity_can_be_updated_as_entity() {
        let test_entity = TestEntity {
            i: 99,
            ..Default::default()
        };

        let entity = Entity::new(test_entity);

        let mut typed_entity = entity.typed::<TestEntity>();

        typed_entity.get_mut(|e| e.i = 100);
        let read = typed_entity.get(|e| e.i);

        assert_eq!(100, read);
    }
}
