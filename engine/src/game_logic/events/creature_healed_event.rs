use crate::game_state::UnitCardInstanceId;

use super::{Event, GameEvent};

#[derive(Debug, Clone)]
pub struct CreatureHealedEvent {
    creature_id: UnitCardInstanceId,
    heal_amount: usize,
}

impl CreatureHealedEvent {
    pub fn new(creature_id: UnitCardInstanceId, heal_amount: usize) -> Self {
        Self {
            creature_id,
            heal_amount,
        }
    }

    pub fn creature_id(&self) -> UnitCardInstanceId {
        self.creature_id
    }

    pub fn heal_amount(&self) -> usize {
        self.heal_amount
    }
}

impl Event for CreatureHealedEvent {}

impl From<CreatureHealedEvent> for GameEvent {
    fn from(val: CreatureHealedEvent) -> Self {
        GameEvent::CreatureHealed(val)
    }
}
