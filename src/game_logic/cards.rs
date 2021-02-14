mod emotional_support_dog;
mod prawn;
mod really_big_rock;
mod rickety_cannon;

use std::fmt::Display;

pub use emotional_support_dog::EmotionalSupportDog;
pub use prawn::Prawn;
pub use really_big_rock::ReallyBigRock;
pub use rickety_cannon::RicketyCannon;

use crate::game_state::{GameState, UnitCardInstance, UnitCardInstanceId};

use super::{EventDispatcher, GameEvent, PassiveEffectDefinition};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Position {
    Front,
    Back,
    Either,
}

pub trait CardDefinition: std::fmt::Debug {
    fn title(&self) -> &str;
    fn cost(&self) -> i32;
    fn text(&self) -> &str;
    fn flavor_text(&self) -> &str;
}

/// A `Card` that can be placed as a unit on the board.
pub trait UnitCardDefinition: CardDefinition {
    fn attack(&self) -> i32;
    fn health(&self) -> i32;
    fn row_width(&self) -> usize;
    fn placeable_at(&self) -> Position;

    fn upon_summon(&self, _own_id: UnitCardInstanceId, _game_state: &GameState) -> Vec<GameEvent> {
        Vec::new()
    }

    fn upon_summonz(
        &self,
    ) -> Box<dyn FnOnce(UnitCardInstanceId, &GameState, &mut EventDispatcher)> {
        Box::new(|_id, _game_state, _dispatcher| {})
    }

    fn upon_summonx(
        &self,
    ) -> Box<dyn FnOnce(&mut UnitCardInstance, &GameState, &mut EventDispatcher)> {
        Box::new(|instance, _game_state, _dispatcher| {})
    }

    fn upon_death(&self, _own_id: UnitCardInstanceId, _game_state: &GameState) -> Vec<GameEvent> {
        Vec::new()
    }

    fn passive_effect(&self) -> Option<Box<dyn PassiveEffectDefinition>> {
        None
    }

    // TODO: or naming "guardian"?
    fn is_defender(&self) -> bool {
        false
    }

    fn boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }

    fn make_instance(self) -> UnitCardInstance
    where
        Self: Sized + 'static,
    {
        let boxed: Box<dyn UnitCardDefinition> = self.boxed();
        UnitCardInstance::new(boxed)
    }
}
