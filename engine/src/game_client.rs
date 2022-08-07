use crate::{event::EventMessage, PlayerId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FromClient {
    EndTurn,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FromServer {
    Event(EventMessage),
    Hello(PlayerId, PlayerId),
}

pub trait ClientChannel {
    fn push_message(&self, message: FromServer);
    fn try_receive_message(&self) -> Option<FromClient>;
}
