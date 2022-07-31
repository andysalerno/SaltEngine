use serde::{Deserialize, Serialize};

use crate::event::EventMessage;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FromClient {
    EndTurn,
}

pub trait ClientChannel {
    fn push_message(&self, message: &EventMessage);
    fn try_receive_message(&self) -> Option<FromClient>;
}
