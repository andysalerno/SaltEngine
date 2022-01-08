use crate::entities::{PlayerId, UnitCardInstance, UnitCardInstanceId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CardAddedToHand {
    pub player_id: PlayerId,
    pub card_id: UnitCardInstanceId,
    pub card: Option<UnitCardInstance>,
}
