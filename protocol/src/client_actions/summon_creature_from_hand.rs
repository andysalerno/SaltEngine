use serde::{Deserialize, Serialize};

use crate::entities::{BoardPos, CreatureInstanceId, PlayerId};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SummonCreatureFromHand {
    pub player_id: PlayerId,
    pub board_pos: BoardPos,
    pub card_id: CreatureInstanceId,
}
