use id::Id;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreatureAttacksTarget {
    pub attacker: Id,
    pub target: Id,
}
