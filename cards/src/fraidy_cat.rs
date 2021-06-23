use salt_engine::{
    cards::{CardDefinition, Position, UnitCardDefinition},
    id::Id,
};

#[derive(Debug, Clone)]
pub struct FraidyCat;

impl FraidyCat {
    fn id(&self) -> Id {
        // id::parse("...")
        todo!()
    }
}

impl CardDefinition for FraidyCat {
    fn title(&self) -> &str {
        "Fraidy Cat"
    }

    fn cost(&self) -> i32 {
        3
    }

    fn flavor_text(&self) -> &str {
        "todo"
    }

    fn text(&self) -> &str {
        "Front.
Hidden."
    }
}

impl UnitCardDefinition for FraidyCat {
    fn attack(&self) -> i32 {
        2
    }

    fn health(&self) -> i32 {
        4
    }

    fn row_width(&self) -> usize {
        1
    }

    fn placeable_at(&self) -> Position {
        Position::Front
    }

    fn is_hidden(&self) -> bool {
        true
    }
}
