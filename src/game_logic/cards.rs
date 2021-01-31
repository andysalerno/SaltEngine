pub mod prawn;

pub trait CardDefinition: std::fmt::Debug {
    fn title(&self) -> &str;
    fn cost(&self) -> i32;
    fn flavor_text(&self) -> &str;
}

/// A `Card` that can be placed as a unit on the board.
pub trait UnitCardDefinition: CardDefinition {
    fn attack(&self) -> i32;
    fn health(&self) -> i32;
    fn row_width(&self) -> i32;
}
