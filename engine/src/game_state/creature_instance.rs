use super::creature_definition::CreatureDefinitionId;
use entity_arena::{
    id::{EntityId, EntityTypeId},
    IsEntity,
};
use isentity_macro_derive::entity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Boon {
    Swift,
    Invisible,
    FastAttack,
    Etc,
}

#[derive(Debug, Serialize, Deserialize)]
#[entity("f8891722-a910-47cd-8220-b10368b0b537")]
pub struct CreatureInstance {
    id: EntityId,
    definition: CreatureDefinitionId,
    attack: i32,
    health: i32,
    boons: Vec<Boon>,
}
