use std::borrow::Borrow;

use crate::{
    game_state::{GameState, UnitCardInstanceId},
    id::Id,
};

pub trait PassiveEffectDefinition: std::fmt::Debug {
    fn definition_id(&self) -> Id;
    fn update(
        &self,
    ) -> Box<dyn FnOnce(PassiveEffectInstanceId, UnitCardInstanceId, &mut GameState)>;
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
    originator_id: UnitCardInstanceId,
}

impl PassiveEffectInstance {
    pub fn new(
        definition: Box<dyn PassiveEffectDefinition>,
        originator_id: UnitCardInstanceId,
    ) -> Self {
        Self {
            definition,
            instance_id: PassiveEffectInstanceId::new(),
            originator_id,
        }
    }

    pub fn instance_id(&self) -> PassiveEffectInstanceId {
        self.instance_id
    }

    pub fn originator_id(&self) -> UnitCardInstanceId {
        self.originator_id
    }

    pub fn definition(&self) -> &dyn PassiveEffectDefinition {
        self.definition.borrow()
    }
}
