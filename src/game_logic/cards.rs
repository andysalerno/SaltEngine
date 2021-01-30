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

pub trait UnitCardDefinitionClone {
    fn clone_box(&self) -> Box<dyn UnitCardDefinition>;
}

impl<T> UnitCardDefinitionClone for T
where
    T: 'static + UnitCardDefinition + Clone,
{
    fn clone_box(&self) -> Box<dyn UnitCardDefinition> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
// impl Clone for Box<dyn UnitCardDefinition> {
//     fn clone(&self) -> Box<dyn UnitCardDefinition> {
//         self.clone_box()
//     }
// }
