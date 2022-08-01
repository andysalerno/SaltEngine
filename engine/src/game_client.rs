use serde::{Deserialize, Serialize};

use crate::event::EventMessage;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FromClient {
    EndTurn,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FromServer {
    Event(EventMessage),
}

pub trait ClientChannel {
    fn push_message(&self, message: FromServer);
    fn try_receive_message(&self) -> Option<FromClient>;
}
