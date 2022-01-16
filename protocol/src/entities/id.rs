use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::IsEntity;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Id(Uuid);

impl Id {
    #[must_use]
    pub fn new() -> Self {
        Id(Uuid::new_v4())
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

/// A trait providing an entity's ID.
pub(crate) trait HasId {
    type IdType: EntityId;

    fn id(&self) -> Self::IdType;
}

/// A trait describing a type that represents an ID of some kind.
pub trait AsId {
    fn as_id(&self) -> Id;
}

/// A trait describing an entity's ID of some kind, with an associated type for the specific entity type it represents.
pub(crate) trait EntityId: AsId {
    type EntityType: IsEntity;
}

#[derive(Debug, Eq, Hash, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PlayerId(Id);

impl PlayerId {
    #[must_use]
    pub fn new() -> Self {
        Self(Id::new())
    }
}

impl Default for PlayerId {
    fn default() -> Self {
        Self::new()
    }
}

impl AsId for PlayerId {
    fn as_id(&self) -> Id {
        self.0
    }
}
