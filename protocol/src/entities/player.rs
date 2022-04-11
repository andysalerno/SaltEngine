use super::{AsId, Id};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
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
