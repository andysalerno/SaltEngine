use crate::game_logic::cards::{CardDefinition, UnitCardDefinition};
use protocol::entities::Position;

#[derive(Debug)]
pub struct HeroDefinition;

impl CardDefinition for HeroDefinition {
    fn title(&self) -> &str {
        "Hero"
    }

    fn cost(&self) -> i32 {
        0
    }

    fn text(&self) -> &str {
        ""
    }

    fn flavor_text(&self) -> &str {
        ""
    }
}

impl UnitCardDefinition for HeroDefinition {
    fn attack(&self) -> i32 {
        0
    }

    fn health(&self) -> i32 {
        30
    }

    fn row_width(&self) -> usize {
        2
    }

    fn placeable_at(&self) -> Position {
        // todo: this is a hack,
        // heroes are not placeable
        // so this abstraction is wrong
        Position::Back
    }
}
