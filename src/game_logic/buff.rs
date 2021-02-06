use crate::id::Id;

use super::passive_effect::PassiveEffectInstanceId;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BuffSourceId {
    Passive(PassiveEffectInstanceId),
    Other(Id),
}

pub trait Buff: std::fmt::Debug {
    fn attack_amount(&self) -> i32;
    fn health_amount(&self) -> i32;
    fn source_id(&self) -> BuffSourceId;
    fn instance_id(&self) -> Id;
    fn definition_id(&self) -> Id;
}
