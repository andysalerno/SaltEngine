use crate::id::HasId;

struct Prawn;

impl HasId for Prawn {
    fn id(&self) -> &uuid::Uuid {
        // id::parse("...")
        todo!()
    }
}

impl super::super::card_definition::CardDefinition for Prawn {
    fn title(&self) -> &str {
        "Prawn"
    }
    fn flavor_text(&self) -> &str {
        todo!()
    }
    fn base_attack(&self) -> u32 {
        1
    }
    fn base_health(&self) -> u32 {
        1
    }
    fn base_cost(&self) -> u32 {
        1
    }
    fn base_width(&self) -> u32 {
        1
    }
}
