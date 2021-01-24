use std::any::Any;

pub trait GameEvent {
    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
}

pub struct AttackEvent;
impl GameEvent for AttackEvent {}

pub struct EndTurnEvent;
impl GameEvent for EndTurnEvent {}
