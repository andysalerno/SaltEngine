use crate::event::EventMessage;

pub struct FromClient;

pub trait ClientChannel {
    fn push_message(&self, message: &EventMessage);
    fn try_receive_message(&self) -> Option<FromClient>;
}
