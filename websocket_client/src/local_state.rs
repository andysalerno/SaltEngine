use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
struct State {
    name: String,
}

fn testing() {
    let x = State {
        name: "hello".to_string(),
    };

    let x = serde_json::to_value(x).unwrap();
}

// find the entity with ID == given
// serialize the entity to json
// find the property given in the request
// set the value to the new value
// deserialize back to the 'real' type

fn update_value(state: &mut State, val_name: String, next_val: String) {
    let mut serialized = serde_json::to_value(&state).unwrap();
    let value = serialized.get_mut(val_name).unwrap();
    *value = json!(next_val);

    *state = serde_json::from_value(serialized).unwrap();
}

#[cfg(test)]
mod test {
    use super::State;

    #[test]
    fn can_update_value() {
        let mut state = State {
            name: "Hello".to_string(),
        };

        assert_eq!("Hello", state.name.as_str());

        super::update_value(&mut state, "name".to_string(), "goodbye".to_string());

        assert_eq!("goodbye", state.name.as_str());
    }
}
