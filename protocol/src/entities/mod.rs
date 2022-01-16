mod board;
mod buff;
mod hand;
mod hideable;
mod id;
mod passive_effect;
mod player;
mod unit_card_definition;
mod unit_card_instance_view;

use std::collections::HashMap;

pub use board::*;
pub use buff::*;
pub use hand::*;
pub use hideable::*;
pub use id::*;
pub use passive_effect::*;
pub use player::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub use unit_card_instance_view::*;

/// A trait that indicates a type should be considered an `Entity`, and can be transformed into an `Entity`.
pub trait IsEntity: HasId + Serialize + DeserializeOwned + 'static {
    /// The type of `Id` that describes this entity.
    type IdType: EntityId;

    fn type_id(&self) -> EntityTypeId;

    /// Creates an `Entity` representation of this object.
    fn as_entity(&self) -> Entity {
        Entity {
            id: self.id().as_id(),
            type_id: self.type_id(),
            data: serde_json::to_string(self).unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Entity {
    id: Id,
    type_id: EntityTypeId,
    data: String,
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

#[derive(Serialize, Deserialize, Default)]
pub struct LocalState {
    name: String,
    entities: HashMap<Id, Entity>,
}

impl LocalState {
    pub fn find_entity<T: EntityId>(&self, id: T) -> &Entity {
        let id = id.as_id();
        self.entities.get(&id).unwrap()
    }

    pub fn find<T: EntityId>(&self, id: T) -> T::EntityType {
        let id = id.as_id();
        let entity = self.entities.get(&id).unwrap();

        let deserialized: T::EntityType = serde_json::from_str(entity.data.as_str()).unwrap();

        deserialized
    }

    pub fn add<T: IsEntity>(&mut self, to_add: T) {
        let entity = to_add.as_entity();
        self.entities.insert(entity.id, entity);
    }

    pub fn update<T: IsEntity>(&mut self, to_update: T) {
        let found = self.entities.get_mut(&to_update.id().as_id()).unwrap();
        *found = to_update.as_entity();
    }
}

#[cfg(test)]
mod test {
    use super::{BuffInstanceId, BuffPlayerView, BuffSourceId, Id, LocalState};

    #[test]
    fn can_add_entity() {
        let mut state = LocalState::default();

        let buff_view = BuffPlayerView {
            attack_amount: 10,
            health_amount: 11,
            source_id: BuffSourceId::Other(Id::new()),
            instance_id: BuffInstanceId::new(),
            definition_id: Id::new(),
            is_from_passive: false,
        };

        state.add(buff_view);
    }

    #[test]
    fn can_retrieve_entity() {
        let mut state = LocalState::default();

        let buff_view = BuffPlayerView {
            attack_amount: 10,
            health_amount: 11,
            source_id: BuffSourceId::Other(Id::new()),
            instance_id: BuffInstanceId::new(),
            definition_id: Id::new(),
            is_from_passive: false,
        };

        let id = buff_view.instance_id;

        state.add(buff_view);

        let retrieved = state.find(id);

        assert_eq!(10, retrieved.attack_amount);
        assert_eq!(11, retrieved.health_amount);
    }

    #[test]
    fn can_update_entity() {
        let mut state = LocalState::default();

        let buff_view = BuffPlayerView {
            attack_amount: 10,
            health_amount: 11,
            source_id: BuffSourceId::Other(Id::new()),
            instance_id: BuffInstanceId::new(),
            definition_id: Id::new(),
            is_from_passive: false,
        };

        let id = buff_view.instance_id;

        state.add(buff_view);

        let mut retrieved = state.find(id);

        retrieved.attack_amount = 9;

        state.update(retrieved);

        let retrieved = state.find(id);

        assert_eq!(9, retrieved.attack_amount);
    }
}
