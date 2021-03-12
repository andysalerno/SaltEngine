use crate::{game_state::UnitCardInstanceId, id::Id};

use super::passive_effect::PassiveEffectInstanceId;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BuffInstanceId(Id);

impl BuffInstanceId {
    pub fn new() -> Self {
        Self(Id::new())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BuffSourceId {
    Passive(PassiveEffectInstanceId),
    CreatureInstance(UnitCardInstanceId),
    Other(Id),
}

pub trait Buff: std::fmt::Debug {
    fn attack_amount(&self) -> i32;
    fn health_amount(&self) -> i32;
    fn source_id(&self) -> BuffSourceId;
    fn instance_id(&self) -> BuffInstanceId;
    fn definition_id(&self) -> Id;

    fn is_from_passive(&self) -> bool {
        match self.source_id() {
            BuffSourceId::Passive(_) => true,
            _ => false,
        }
    }
}

pub struct BuffBuilder {
    attack_amount: i32,
    health_amount: i32,
    source: BuffSourceId,
    instance_id: BuffInstanceId,
    definition_id: Id,
}

impl BuffBuilder {
    pub fn new(source: BuffSourceId, definition_id: Id) -> Self {
        Self {
            attack_amount: 0,
            health_amount: 0,
            instance_id: BuffInstanceId::new(),
            source,
            definition_id,
        }
    }

    pub fn attack(mut self, attack_buff_amount: i32) -> Self {
        self.attack_amount = attack_buff_amount;
        self
    }

    pub fn health(mut self, health_buff_amount: i32) -> Self {
        self.health_amount = health_buff_amount;
        self
    }

    pub fn build(self) -> impl Buff {
        BuiltBuff {
            attack_amount: self.attack_amount,
            health_amount: self.health_amount,
        }
    }
}

#[derive(Debug)]
struct BuiltBuff {
    attack_amount: i32,
    health_amount: i32,
}

impl Buff for BuiltBuff {
    fn attack_amount(&self) -> i32 {
        self.attack_amount
    }

    fn health_amount(&self) -> i32 {
        self.health_amount
    }

    fn source_id(&self) -> BuffSourceId {
        todo!()
    }

    fn instance_id(&self) -> BuffInstanceId {
        todo!()
    }

    fn definition_id(&self) -> Id {
        todo!()
    }
}
