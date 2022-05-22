use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Id(Uuid);

impl Id {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    #[must_use]
    pub fn parse_str(s: &str) -> Self {
        Id(Uuid::parse_str(s).unwrap())
    }
}

impl std::fmt::Debug for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.to_string()[0..8].fmt(f)
    }
}
