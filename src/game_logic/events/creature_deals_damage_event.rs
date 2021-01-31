use crate::id::Id;

use super::Event;

#[derive(Debug)]
pub struct CreatureDealsDamageEvent {
    creature_id: Id,
    damage_amount: usize,
    target_id: Id,
}

impl CreatureDealsDamageEvent {
    pub fn new(creature_id: Id, target_id: Id, damage_amount: usize) -> Self {
        Self {
            creature_id,
            damage_amount,
            target_id,
        }
    }

    pub fn creature_id(&self) -> Id {
        self.creature_id
    }

    pub fn target_id(&self) -> Id {
        self.target_id
    }

    pub fn damage_amount(&self) -> usize {
        self.damage_amount
    }
}

impl Event for CreatureDealsDamageEvent {}
