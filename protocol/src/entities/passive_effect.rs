use super::{Id, UnitCardInstanceId};
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

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PassiveEffectInstancePlayerView {
    /// The definition of the passive effect.
    definition: PassiveEffectDefinitionPlayerView,

    /// The unique ID of this instance of the passive effect.
    instance_id: PassiveEffectInstanceId,

    /// The ID of the card instance that originated this passive effect.
    originator_id: UnitCardInstanceId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PassiveEffectDefinitionPlayerView {
    definition_id: Id,
}
