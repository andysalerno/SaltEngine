use crate::entities::PlayerId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EndTurn {
    pub player_id: PlayerId,
}
