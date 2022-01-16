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

    #[must_use]
    pub fn parse_str(s: &str) -> Self {
        Id(Uuid::parse_str(s).unwrap())
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

/// A trait providing an entity's ID.
pub trait HasId {
    type IdType: EntityId;

    fn id(&self) -> Self::IdType;
}

/// A trait describing a type that represents an ID of some kind.
pub trait AsId {
    fn as_id(&self) -> Id;
}

/// A trait describing an entity's ID of some kind, with an associated type for the specific entity type it represents.
pub trait EntityId: AsId {
    type EntityType: IsEntity;
}

#[cfg(test)]
mod test {
    use super::Id;

    #[test]
    fn can_parse() {
        let guid_to_parse = "9f19a122-b52f-43b7-b5f4-632d2defb828";

        let parsed = Id::parse_str(guid_to_parse);

        // implicit assert: no panic
    }
}
