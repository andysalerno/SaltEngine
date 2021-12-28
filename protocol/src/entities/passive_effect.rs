use super::{BuffSourceId, Id, UnitCardInstanceId};
use serde::{Deserialize, Serialize};

/// An ID representing a unique instance of a passive effect.
#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct PassiveEffectInstanceId(Id);

impl PassiveEffectInstanceId {
    #[must_use]
    pub fn new() -> Self {
        Self(Id::new())
    }
}

impl Default for PassiveEffectInstanceId {
    fn default() -> Self {
        Self::new()
    }
}
