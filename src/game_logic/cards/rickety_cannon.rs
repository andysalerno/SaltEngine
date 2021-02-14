use crate::{
    game_logic::EventDispatcher,
    game_state::{GameState, UnitCardInstance, UnitCardInstanceId},
    id::Id,
};

use super::{CardDefinition, Position, UnitCardDefinition};

#[derive(Debug, Clone)]
pub struct RicketyCannon;

impl RicketyCannon {
    fn id(&self) -> Id {
        // id::parse("...")
        todo!()
    }
}

impl CardDefinition for RicketyCannon {
    fn title(&self) -> &str {
        "Rickety cannon"
    }

    fn cost(&self) -> i32 {
        2
    }

    fn flavor_text(&self) -> &str {
        "yep"
    }

    fn text(&self) -> &str {
        "Back\nSummon: pick a slot.\nAt the start of\nyour turn, deal\n1 damage there."
    }
}

impl UnitCardDefinition for RicketyCannon {
    fn attack(&self) -> i32 {
        0
    }

    fn health(&self) -> i32 {
        2
    }

    fn row_width(&self) -> usize {
        3
    }

    fn placeable_at(&self) -> Position {
        Position::Back
    }

    fn upon_summon(
        &self,
        _own_id: crate::game_state::UnitCardInstanceId,
        _game_state: &crate::game_state::GameState,
    ) -> Vec<crate::game_logic::GameEvent> {
        // 1. prompt user for the position they want to target
        // 2. set state on _own_id with that pos
        // 3. impl an "upon_turn_start" that does the damage to that pos
        Vec::new()
    }

    fn upon_summonx(
        &self,
    ) -> Box<dyn FnOnce(&mut UnitCardInstance, &GameState, &mut EventDispatcher)> {
        Box::new(|instance, _game_state, _dispatcher| {})
    }

    fn upon_summonz(
        &self,
    ) -> Box<dyn FnOnce(UnitCardInstanceId, &GameState, &mut EventDispatcher)> {
        Box::new(|id, game_state, dispatcher| {
            // 1. prompt user for the position they want to target
            // 2. set state on _own_id with that pos
            // 3. impl an "upon_turn_start" that does the damage to that pos
        })
    }
}
