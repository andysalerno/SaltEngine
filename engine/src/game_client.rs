use crate::{event::EventMessage, PlayerId};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FromClient {
    EndTurn,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FromServer {
    Event(EventMessage),
    Hello(PlayerId, PlayerId),
}

pub trait MessageChannel {
    type Send: Serialize + DeserializeOwned;
    type Receive: Serialize + DeserializeOwned;

    fn send(&self, message: Self::Send);
    fn try_receive(&self) -> Option<Self::Receive>;
}
