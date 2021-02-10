mod emotional_support_dog;
mod prawn;
mod really_big_rock;
mod rickety_cannon;

pub use emotional_support_dog::EmotionalSupportDog;
pub use prawn::Prawn;
pub use really_big_rock::ReallyBigRock;
pub use rickety_cannon::RicketyCannon;

use crate::game_state::UnitCardInstance;

use super::PassiveEffectDefinition;

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
