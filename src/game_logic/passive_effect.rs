use crate::{game_state::GameState, id::Id};

pub trait PassiveEffectDefinition: std::fmt::Debug {
    fn definition_id(&self) -> Id;
    fn update(&self) -> Box<dyn FnOnce(&PassiveEffectInstance, &mut GameState)>;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PassiveEffectInstanceId(Id);

impl PassiveEffectInstanceId {
    pub fn new() -> Self {
        Self(Id::new())
    }
}

#[derive(Debug)]
pub struct PassiveEffectInstance {
    definition: Box<dyn PassiveEffectDefinition>,
    instance_id: PassiveEffectInstanceId,
    originator_id: Id,
}

impl PassiveEffectInstance {
    pub fn instance_id(&self) -> PassiveEffectInstanceId {
        self.instance_id
    }

    pub fn originator_id(&self) -> Id {
        self.originator_id
    }
}
