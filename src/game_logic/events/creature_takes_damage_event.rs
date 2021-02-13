use crate::game_state::UnitCardInstanceId;

use super::{Event, GameEvent};

#[derive(Debug)]
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

impl Into<GameEvent> for CreatureTakesDamageEvent {
    fn into(self) -> GameEvent {
        GameEvent::CreatureTakesDamage(self)
    }
}
