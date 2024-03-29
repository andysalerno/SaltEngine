use protocol::entities::CreatureInstanceId;

use super::Event;

#[derive(Debug, Clone)]
pub struct CreatureTakesDamageEvent {
    creature_id: CreatureInstanceId,
    damage_amount: usize,
}

impl CreatureTakesDamageEvent {
    #[must_use]
    pub fn new(creature_id: CreatureInstanceId, damage_amount: usize) -> Self {
        Self {
            creature_id,
            damage_amount,
        }
    }

    #[must_use]
    pub fn creature_id(&self) -> CreatureInstanceId {
        self.creature_id
    }

    #[must_use]
    pub fn damage_amount(&self) -> usize {
        self.damage_amount
    }
}

impl Event for CreatureTakesDamageEvent {}
