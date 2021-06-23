use salt_engine::{
    cards::{CardDefinition, Position, UnitCardDefinition},
    id::Id,
};

#[derive(Debug, Clone)]
pub struct OutdoorCat;

impl OutdoorCat {
    fn id(&self) -> Id {
        // id::parse("...")
        todo!()
    }
}

impl CardDefinition for OutdoorCat {
    fn title(&self) -> &str {
        "Outdoor Cat"
    }

    fn cost(&self) -> i32 {
        3
    }

    fn flavor_text(&self) -> &str {
        "todo"
    }

    fn text(&self) -> &str {
        "Front.
Hidden.
If revealed during
the enemy's turn, gains
+1/+1."
    }
}

impl UnitCardDefinition for OutdoorCat {
    fn attack(&self) -> i32 {
        3
    }

    fn health(&self) -> i32 {
        2
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
