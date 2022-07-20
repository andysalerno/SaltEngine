use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A unique ID for addressing players.
#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PlayerId {
    guid: Uuid,
}

impl PlayerId {
    #[must_use]
    pub fn new() -> Self {
        Self {
            guid: Uuid::new_v4(),
        }
    }

    pub fn parse(s: impl AsRef<str>) -> Self {
        let guid =
            Uuid::parse_str(s.as_ref()).expect("Expected the input to be a valid, parseable guid.");

        Self { guid }
    }
}

impl Default for PlayerId {
    fn default() -> Self {
        Self::new()
    }
}
