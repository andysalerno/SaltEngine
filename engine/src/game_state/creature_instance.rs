use super::creature_definition::CreatureDefinitionId;
use entity_arena::{
    id::{EntityId, EntityTypeId},
    IsEntity,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Boon {
    Swift,
    Invisible,
    FastAttack,
    Etc,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatureInstance {
    id: EntityId,
    definition: CreatureDefinitionId,
    attack: i32,
    health: i32,
    boons: Vec<Boon>,
}

impl IsEntity for CreatureInstance {
    fn id(&self) -> EntityId {
        self.id
    }

    fn entity_type_id() -> EntityTypeId {
        EntityTypeId::parse_str("f8891722-a910-47cd-8220-b10368b0b537")
    }
}
