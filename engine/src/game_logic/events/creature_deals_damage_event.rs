use crate::game_state::UnitCardInstanceId;

use super::Event;

#[derive(Debug, Clone)]
pub struct CreatureDealsDamageEvent {
    creature_id: UnitCardInstanceId,
    damage_amount: usize,
    target_id: UnitCardInstanceId,
}

impl CreatureDealsDamageEvent {
    #[must_use]
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

    #[must_use]
    pub fn creature_id(&self) -> UnitCardInstanceId {
        self.creature_id
    }

    #[must_use]
    pub fn target_id(&self) -> UnitCardInstanceId {
        self.target_id
    }

    #[must_use]
    pub fn damage_amount(&self) -> usize {
        self.damage_amount
    }
}

impl Event for CreatureDealsDamageEvent {}
