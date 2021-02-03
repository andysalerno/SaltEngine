use crate::id::Id;

pub trait Buff: std::fmt::Debug {
    fn attack_amount(&self) -> i32;
    fn health_amount(&self) -> i32;
    fn instance_id(&self) -> Id;
    fn definition_id(&self) -> Id;
}
