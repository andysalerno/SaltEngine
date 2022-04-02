use protocol::entities::CreatureInstanceId;

use super::Event;

#[derive(Debug, Clone)]
pub struct CreatureDealsDamageEvent {
    creature_id: CreatureInstanceId,
    damage_amount: usize,
    target_id: CreatureInstanceId,
}

impl CreatureDealsDamageEvent {
    #[must_use]
    pub fn new(
        creature_id: CreatureInstanceId,
        target_id: CreatureInstanceId,
        damage_amount: usize,
    ) -> Self {
        Self {
            creature_id,
            damage_amount,
            target_id,
        }
    }

    #[must_use]
    pub fn creature_id(&self) -> CreatureInstanceId {
        self.creature_id
    }

    #[must_use]
    pub fn target_id(&self) -> CreatureInstanceId {
        self.target_id
    }

    #[must_use]
    pub fn damage_amount(&self) -> usize {
        self.damage_amount
    }
}

impl Event for CreatureDealsDamageEvent {}
