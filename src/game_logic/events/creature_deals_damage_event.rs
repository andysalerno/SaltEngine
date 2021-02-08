use crate::game_state::UnitCardInstanceId;

use super::{Event, GameEvent};

#[derive(Debug)]
pub struct CreatureDealsDamageEvent {
    creature_id: UnitCardInstanceId,
    damage_amount: usize,
    target_id: UnitCardInstanceId,
}

impl CreatureDealsDamageEvent {
    pub fn new(
        creature_id: UnitCardInstanceId,
        target_id: UnitCardInstanceId,
        damage_amount: usize,
    ) -> Self {
        Self {
            creature_id,
            damage_amount,
            target_id,
        }
    }

    pub fn creature_id(&self) -> UnitCardInstanceId {
        self.creature_id
    }

    pub fn target_id(&self) -> UnitCardInstanceId {
        self.target_id
    }

    pub fn damage_amount(&self) -> usize {
        self.damage_amount
    }
}

impl Event for CreatureDealsDamageEvent {}

impl Into<GameEvent> for CreatureDealsDamageEvent {
    fn into(self) -> GameEvent {
        GameEvent::CreatureDealsDamage(self)
    }
}
