use crate::{game_state::GameState, id::Id};

pub trait PassiveEffect {
    fn originator(&self) -> Id;
    fn definition_id(&self) -> Id;
    fn instance_id(&self) -> Id;
    fn update(&self) -> Box<dyn FnOnce(&mut GameState)>;
}
