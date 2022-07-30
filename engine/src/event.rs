use crate::{Dispatcher, GameState};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventMessage {
    kind: EventType,
    body: String,
}

impl EventMessage {
    #[must_use]
    pub const fn event_type(&self) -> &EventType {
        &self.kind
    }

    #[must_use]
    pub fn unpack<T: Event>(&self) -> T {
        serde_json::from_str(&self.body).expect("Unpacking (deserializion) failed for this entity")
    }
}

/// A trait for any kind of event body, which can be serialized and deserialized,
/// and provides a unique `EventType` so that exactly one correct handler can be picked.
pub trait Event: Serialize + DeserializeOwned {
    fn event_type(&self) -> EventType;
}

impl<T: Event> From<T> for EventMessage {
    fn from(e: T) -> Self {
        Self {
            kind: e.event_type(),
            // currently we just hard-code serde_json as the global serializer
            body: serde_json::to_string(&e).expect("Could not serialize event"),
        }
    }
}

/// An identifier for a type of event.
/// Each `Event` must have a unique `EventType`.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct EventType(String);

impl EventType {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}

/// A trait for a handler that can handle an `EventMessage` with a given `EventType`.
pub trait EventHandler {
    /// The `EventType` of the event that his handler supports.
    fn event_type(&self) -> EventType;

    /// Handle the given event.
    fn handle(&self, event: &EventMessage, game_state: &mut GameState, dispatcher: &Dispatcher);
}
