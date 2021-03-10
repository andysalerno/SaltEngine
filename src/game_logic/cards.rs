mod attack_dog;
mod emotional_support_dog;
mod pawn;
mod popcorn_vendor;
mod really_big_rock;
mod rickety_cannon;
mod sleeping_dog;

pub use attack_dog::AttackDog;
pub use emotional_support_dog::EmotionalSupportDog;
pub use pawn::Pawn;
pub use popcorn_vendor::PopcornVendor;
pub use really_big_rock::ReallyBigRock;
pub use rickety_cannon::RicketyCannon;
pub use sleeping_dog::SleepingDog;

use crate::game_state::{board::BoardPos, GameState, UnitCardInstance, UnitCardInstanceId};

use super::{EventDispatcher, PassiveEffectDefinition};

/// Describes which board positions
/// this creature card may occupy.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Position {
    /// The front side of the board.
    Front,

    /// The back side of the board.
    Back,

    /// Either the front or the back sides of the board..
    Either,
}

/// The most general definition that cards of all types must implement.
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

    /// The card may provide logic that is executed when it is summoned from a the player's hand.
    /// The boxed function is provided the instance of the card being summoned,
    /// the current game state of the board as it was summoned,
    /// and the event dispatcher, in case the card's summoning effect requries dispatching more events.
    fn upon_summon(
        &self,
    ) -> Box<dyn FnOnce(&mut UnitCardInstance, BoardPos, &mut GameState, &mut EventDispatcher)>
    {
        Box::new(|_instance, _summoned_to_pos, _game_state, _dispatcher| {})
    }

    fn upon_death(
        &self,
    ) -> Box<dyn FnOnce(&mut UnitCardInstance, BoardPos, &mut GameState, &mut EventDispatcher)>
    {
        Box::new(|_instance, _destroyed_at_pos, _game_state, _dispatcher| {})
    }

    fn upon_receive_damage(
        &self,
    ) -> Box<dyn FnOnce(UnitCardInstanceId, &mut GameState, &mut EventDispatcher)> {
        Box::new(|_id, _game_state, _dispatcher| {})
    }

    fn upon_turn_start(
        &self,
    ) -> Box<dyn FnOnce(UnitCardInstanceId, &mut GameState, &mut EventDispatcher)> {
        Box::new(|_id, _game_state, _dispatcher| {})
    }

    fn upon_turn_end(
        &self,
    ) -> Box<dyn FnOnce(UnitCardInstanceId, &mut GameState, &mut EventDispatcher)> {
        Box::new(|_id, _game_state, _dispatcher| {})
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
