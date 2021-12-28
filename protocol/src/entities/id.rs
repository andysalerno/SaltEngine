use serde::{Deserialize, Serialize};
use std::fmt::Display;
use uuid::Uuid;

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

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub struct UnitCardInstanceId(Id);

impl UnitCardInstanceId {
    #[must_use]
    pub fn new() -> Self {
        Self(Id::new())
    }
}

impl Default for UnitCardInstanceId {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for UnitCardInstanceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Debug;
        self.0.fmt(f)
    }
}
