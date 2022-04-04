use crate::{
    entity::{Entity, IsEntity},
    id::EntityId,
    TypedEntity,
};
use std::{borrow::Borrow, collections::HashMap};

/// A global state, defining all known entities mapped from their ID.
#[derive(Clone, Debug, Default)]
pub struct EntityArena {
    entities: HashMap<EntityId, Entity>,
}

impl EntityArena {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
        }
    }

    pub fn add(&mut self, entity: impl IsEntity) -> EntityId {
        let entity = Entity::new(entity);

        let id = entity.id();

        let already_existed = self.entities.insert(id, entity).is_some();

        if already_existed {
            panic!("An entity with key {id:?} already existed");
        }

        id
    }

    pub fn remove(&mut self, id: impl Borrow<EntityId>) {
        self.entities
            .remove(id.borrow())
            .expect("Could not remove with an ID that did not exist.");
    }

    pub fn get(&self, id: impl Borrow<EntityId>) -> &Entity {
        self.entities.get(id.borrow()).unwrap()
    }

    pub fn get_mut(&mut self, id: impl Borrow<EntityId>) -> &mut Entity {
        self.entities.get_mut(id.borrow()).unwrap()
    }

    pub fn entities(&self) -> impl Iterator<Item = &Entity> {
        self.entities.values()
    }

    pub fn entities_mut(&mut self) -> impl Iterator<Item = &mut Entity> {
        self.entities.values_mut()
    }

    pub fn of_type<T: IsEntity>(&self) -> impl Iterator<Item = TypedEntity<T, &serde_json::Value>> {
        self.entities
            .values()
            .filter(|e| e.entity_type_id() == T::entity_type_id())
            .map(|e| e.as_typed::<T>())
    }

    pub fn of_type_mut<T: IsEntity>(
        &mut self,
    ) -> impl Iterator<Item = TypedEntity<T, &mut serde_json::Value>> {
        self.entities
            .values_mut()
            .filter(|e| e.entity_type_id() == T::entity_type_id())
            .map(|e| e.as_typed_mut::<T>())
    }
}

#[cfg(test)]
mod tests {
    use super::EntityArena;
    use crate::{entity::tests::TestEntity, id::EntityId};

    #[test]
    fn entity_can_be_added() {
        let mut state = EntityArena::new();

        let test_entity = TestEntity::new();

        state.add(test_entity);
    }

    #[test]
    fn entity_can_be_added_retrieved() {
        let mut state = EntityArena::new();

        let test_entity = TestEntity::new();

        let id = state.add(test_entity);

        let _retrieved = state.get(id);
    }

    #[test]
    #[should_panic]
    fn expect_panic_when_entity_not_exist() {
        let mut state = EntityArena::new();

        let test_entity = TestEntity::new();

        let _id = state.add(test_entity);

        let some_other_id = EntityId::new();

        let _retrieved = state.get(some_other_id);
    }

    #[test]
    fn entity_can_be_added_retrieved_and_read() {
        let mut state = EntityArena::new();

        let mut test_entity = TestEntity::new();

        test_entity.set_i32_val(49);

        let id = state.add(test_entity);

        let retrieved = state.get(id).as_typed::<TestEntity>();

        let read = retrieved.get(|e| e.i32_val());

        assert_eq!(49, read);
    }

    #[test]
    fn entity_can_be_added_retrieved_and_updated() {
        let mut state = EntityArena::new();

        let mut test_entity = TestEntity::new();

        test_entity.set_i32_val(49);

        let id = state.add(test_entity);

        let mut retrieved = state.get_mut(id).as_typed_mut::<TestEntity>();

        retrieved.get_mut(|e| e.set_i32_val(100));
        let read = retrieved.get(|e| e.i32_val());

        assert_eq!(100, read);
    }
}
