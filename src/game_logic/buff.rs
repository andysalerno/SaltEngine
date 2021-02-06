use crate::id::Id;

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
