use std::borrow::Borrow;

use crate::{
    game_state::{GameState, UnitCardInstanceId},
    id::Id,
};

use super::Buff;

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

// #[derive(Debug)]
// struct PassiveCompanionBuff {
//     definition_id: Id,
//     buff: Box<dyn Buff>,
// }
#[derive(Debug)]
pub struct PassiveCompanionBuff<T: Buff + Clone> {
    definition_id: Id,
    buff: Box<T>,
}

impl<T: Buff + Clone> PassiveCompanionBuff<T> {
    pub fn new(definition_id: Id, buff: Box<T>) -> Self {
        Self {
            definition_id,
            buff,
        }
    }
}

impl<T: Buff + Clone + 'static> PassiveEffectDefinition for PassiveCompanionBuff<T> {
    fn definition_id(&self) -> Id {
        self.definition_id
    }

    fn update(
        &self,
    ) -> Box<dyn FnOnce(PassiveEffectInstanceId, UnitCardInstanceId, &mut GameState)> {
        let buff = self.buff.clone();
        Box::new(move |instance_id, originator_id, game_state| {
            let doggy_pos = game_state.board().position_with_creature(originator_id);

            if let Some(companion) = game_state.board().companion_creature(doggy_pos) {
                let id = companion.id();

                game_state.update_by_id(id, |c| {
                    c.add_buff(buff);
                });
            }
        })
    }
}
