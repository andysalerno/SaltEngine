use crate::id::HasId;

pub trait CardDefinition: HasId {
    fn title(&self) -> &str;
    fn flavor_text(&self) -> &str;
    fn base_attack(&self) -> u32;
    fn base_health(&self) -> u32;
    fn base_cost(&self) -> u32;
    fn base_width(&self) -> u32;
}
