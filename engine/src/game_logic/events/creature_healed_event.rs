use crate::game_state::UnitCardInstanceId;

use super::{Event, GameEvent};

#[derive(Debug)]
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

impl Into<GameEvent> for CreatureHealedEvent {
    fn into(self) -> GameEvent {
        GameEvent::CreatureHealed(self)
    }
}
