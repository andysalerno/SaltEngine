mod emotional_support_dog;
mod prawn;
mod rickety_cannon;

pub use emotional_support_dog::EmotionalSupportDog;
pub use prawn::Prawn;
pub use rickety_cannon::RicketyCannon;

use super::PassiveEffectDefinition;

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
    fn passive_effect(&self) -> Option<Box<dyn PassiveEffectDefinition>>;
}
