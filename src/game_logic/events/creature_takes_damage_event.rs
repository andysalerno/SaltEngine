use crate::id::Id;

use super::{attack, Event};

#[derive(Debug)]
pub struct CreatureTakesDamageEvent {
    creature_id: Id,
    attacker_id: Id,
    damage_amount: usize,
}

impl CreatureTakesDamageEvent {
    pub fn new(creature_id: Id, attacker_id: Id, damage_amount: usize) -> Self {
        Self {
            creature_id,
            attacker_id,
            damage_amount,
        }
    }

    pub fn creature_id(&self) -> Id {
        self.creature_id
    }

    pub fn attacker_id(&self) -> Id {
        self.attacker_id
    }

    pub fn damage_amount(&self) -> usize {
        self.damage_amount
    }
}

impl Event for CreatureTakesDamageEvent {}
