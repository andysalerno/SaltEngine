pub mod client_actions;
pub mod entities;
pub mod from_client;
pub mod from_server;
pub mod full_state;
pub mod visual_events;

use client_actions::*;
use entities::Id;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use visual_events::*;

/// A marker trait to indicate a type is a `GameMessage`.
pub trait GameMessage: Serialize + DeserializeOwned {}
