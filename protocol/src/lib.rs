pub mod client_actions;
pub mod entities;
pub mod from_client;
pub mod from_server;
pub mod visual_events;

use serde::{de::DeserializeOwned, Serialize};

/// A marker trait to indicate a type is a `GameMessage`.
pub trait GameMessage: Serialize + DeserializeOwned {}
