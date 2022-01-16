use std::collections::HashMap;

use protocol::entities::Id;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Entity {
    id: Id,
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct LocalState {
    name: String,
    entities: HashMap<Id, Entity>,
}

impl LocalState {
    pub fn find_entity(&self, id: Id) -> &Entity {
        self.entities.get(&id).unwrap()
    }

    pub fn add<T: Serialize + DeserializeOwned>(&mut self, to_add: T) {}
}

// find the entity with ID == given
// serialize the entity to json
// find the property given in the request
// set the value to the new value
// deserialize back to the 'real' type
// another option is to have a macro that will `match` on the string value name and map it to an update of the corresponding property
fn update_value<T: Serialize + DeserializeOwned>(
    state: &mut T,
    val_name: String,
    next_val: String,
) {
    let mut serialized = serde_json::to_value(&state).unwrap();
    let value = serialized.get_mut(val_name).unwrap();
    *value = json!(next_val);

    *state = serde_json::from_value(serialized).unwrap();
}

#[cfg(test)]
mod test {
    use super::LocalState;

    #[test]
    fn can_update_value() {
        let mut state = LocalState {
            name: "Hello".to_string(),
            entities: Default::default(),
        };

        assert_eq!("Hello", state.name.as_str());

        super::update_value(&mut state, "name".to_string(), "goodbye".to_string());

        assert_eq!("goodbye", state.name.as_str());
    }
}
