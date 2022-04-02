use crate::entities::{CreatureInstance, CreatureInstanceId, PlayerId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CardAddedToHand {
    pub player_id: PlayerId,
    pub card_id: CreatureInstanceId,
    pub card: Option<CreatureInstance>,
}
