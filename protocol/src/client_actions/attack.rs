use crate::entities::CreatureInstanceId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attack {
    pub attacker: CreatureInstanceId,
    pub target: CreatureInstanceId,
}
