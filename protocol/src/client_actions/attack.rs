use crate::entities::UnitCardInstanceId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attack {
    pub attacker: UnitCardInstanceId,
    pub target: UnitCardInstanceId,
}
