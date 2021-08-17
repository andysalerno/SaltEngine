use crate::game_state::UnitCardInstanceId;

use super::{Event, GameEvent};

#[derive(Debug, Clone)]
pub struct CreatureTakesDamageEvent {
    creature_id: UnitCardInstanceId,
    damage_amount: usize,
}

impl CreatureTakesDamageEvent {
    pub fn new(creature_id: UnitCardInstanceId, damage_amount: usize) -> Self {
        Self {
            creature_id,
            damage_amount,
        }
    }

    pub fn creature_id(&self) -> UnitCardInstanceId {
        self.creature_id
    }

    pub fn damage_amount(&self) -> usize {
        self.damage_amount
    }
}

impl Event for CreatureTakesDamageEvent {}

impl From<CreatureTakesDamageEvent> for GameEvent {
    fn from(val: CreatureTakesDamageEvent) -> Self {
        GameEvent::CreatureTakesDamage(val)
    }
}
