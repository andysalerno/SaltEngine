use protocol::entities::{Id, Position};
use salt_engine::cards::{CardDefinition, UnitCardDefinition};

fn create() -> Box<dyn UnitCardDefinition> {
    Box::new(AttackDog)
}

#[derive(Debug, Clone)]
pub struct AttackDog;

impl AttackDog {
    fn id(&self) -> Id {
        // id::parse("...")
        todo!()
    }
}

impl CardDefinition for AttackDog {
    fn title(&self) -> &str {
        "Attack Dog"
    }

    fn cost(&self) -> i32 {
        3
    }

    fn flavor_text(&self) -> &str {
        "todo"
    }

    fn text(&self) -> &str {
        ""
    }
}

impl UnitCardDefinition for AttackDog {
    fn attack(&self) -> i32 {
        5
    }

    fn health(&self) -> i32 {
        3
    }

    fn row_width(&self) -> usize {
        1
    }

    fn placeable_at(&self) -> Position {
        Position::Front
    }
}
