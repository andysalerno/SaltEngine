use crate::v2::CreatureDefinitionId;
use entity_arena::{id::EntityTypeId, IsEntity};
use isentity_macro_derive::entity;
use serde::{Deserialize, Serialize};

/// An entity representing a card in a hand.
#[derive(Debug, Serialize, Deserialize)]
#[entity("abd58415-88ea-4f27-9d9b-05602ed75b6b")]
pub struct CardInHand {
    definition_id: CreatureDefinitionId,
}

impl CardInHand {
    pub fn new(definition_id: CreatureDefinitionId) -> Self {
        Self { definition_id }
    }
}
