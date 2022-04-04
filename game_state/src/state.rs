use crate::{
    entity::{Entity, IsEntity},
    id::Id,
};
use std::{borrow::Borrow, collections::HashMap};

/// A global state, defining all known entities mapped from their ID.
#[derive(Clone, Debug, Default)]
pub struct State {
    entities: HashMap<Id, Entity>,
}

impl State {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
        }
    }

    pub fn add(&mut self, entity: impl IsEntity) -> Id {
        let entity = Entity::new(entity);

        let id = entity.id();

        let already_existed = self.entities.insert(id, entity).is_some();

        if already_existed {
            panic!("An entity with key {id:?} already existed");
        }

        id
    }

    pub fn get(&self, id: impl Borrow<Id>) -> &Entity {
        self.entities.get(id.borrow()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::State;
    use crate::{entity::tests::TestEntity, id::Id};

    #[test]
    fn entity_can_be_added() {
        let mut state = State::new();

        let test_entity = TestEntity::default();

        state.add(test_entity);
    }

    #[test]
    fn entity_can_be_added_retrieved() {
        let mut state = State::new();

        let test_entity = TestEntity::default();

        let id = state.add(test_entity);

        let _retrieved = state.get(id);
    }

    #[test]
    #[should_panic]
    fn expect_panic_when_entity_not_exist() {
        let mut state = State::new();

        let test_entity = TestEntity::default();

        let _id = state.add(test_entity);

        let some_other_id = Id::new();

        let _retrieved = state.get(some_other_id);
    }
}
