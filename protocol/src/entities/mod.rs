mod board;
mod buff;
mod hideable;
mod id;
mod passive_effect;
mod unit_card_definition;
mod unit_card_instance_view;

use std::collections::HashMap;

pub use board::*;
pub use buff::*;
pub use hideable::*;
pub use id::*;
pub use passive_effect::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub use unit_card_instance_view::*;

// trait Entity<'a>: Serialize + Deserialize<'a> {
//     fn entity_id(&self) -> Id;
// }

pub(crate) trait IsEntity: HasId + Serialize + DeserializeOwned {
    type IdType: EntityId;

    fn as_entity(&self) -> Entity {
        Entity {
            id: self.id().as_id(),
            data: serde_json::to_string(self).unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Entity {
    id: Id,
    data: String,
}

#[derive(Serialize, Deserialize, Default)]
struct LocalState {
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
}

#[cfg(test)]
mod test {
    use super::{BuffInstanceId, BuffPlayerView, BuffSourceId, Id, LocalState};

    #[test]
    fn wtf() {
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
    fn wtf_2() {
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
}
