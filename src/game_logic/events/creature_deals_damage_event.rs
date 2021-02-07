use crate::{
    game_logic::event_handlers::CreatureDealsDamageHandler, game_state::UnitCardBoardInstanceId,
};

use super::{Event, GameEvent};

#[derive(Debug)]
pub struct CreatureDealsDamageEvent {
    creature_id: UnitCardBoardInstanceId,
    damage_amount: usize,
    target_id: UnitCardBoardInstanceId,
}

impl CreatureDealsDamageEvent {
    pub fn new(
        creature_id: UnitCardBoardInstanceId,
        target_id: UnitCardBoardInstanceId,
        damage_amount: usize,
    ) -> Self {
        Self {
            creature_id,
            damage_amount,
            target_id,
        }
    }

    pub fn creature_id(&self) -> UnitCardBoardInstanceId {
        self.creature_id
    }

    pub fn target_id(&self) -> UnitCardBoardInstanceId {
        self.target_id
    }

    pub fn damage_amount(&self) -> usize {
        self.damage_amount
    }
}

impl Event for CreatureDealsDamageEvent {}

impl Into<GameEvent> for CreatureDealsDamageEvent {
    fn into(self) -> GameEvent {
        GameEvent::CreatureDealsDamage(self)
    }
}
