use crate::game_state::UnitCardInstanceId;

use super::Event;

#[derive(Debug, Clone)]
pub struct CreatureHealedEvent {
    creature_id: UnitCardInstanceId,
    heal_amount: usize,
}

impl CreatureHealedEvent {
    #[must_use]
    pub fn new(creature_id: UnitCardInstanceId, heal_amount: usize) -> Self {
        Self {
            creature_id,
            heal_amount,
        }
    }

    #[must_use]
    pub fn creature_id(&self) -> UnitCardInstanceId {
        self.creature_id
    }

    #[must_use]
    pub fn heal_amount(&self) -> usize {
        self.heal_amount
    }
}

impl Event for CreatureHealedEvent {}
