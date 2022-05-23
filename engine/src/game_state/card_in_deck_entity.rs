use crate::v2::CreatureDefinitionId;
use entity_arena::{id::EntityTypeId, IsEntity};
use isentity_macro_derive::entity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[entity("05ca041b-8779-4dcf-9e39-42ca823076fc")]
pub struct CardInDeck {
    definition_id: CreatureDefinitionId,
}

impl CardInDeck {
    pub fn new(definition_id: CreatureDefinitionId) -> Self {
        Self { definition_id }
    }
}

#[cfg(test)]
mod tests {
    use super::CardInDeck;
    use entity_arena::id::EntityTypeId;
    use entity_arena::IsEntity;

    #[test]
    fn entityid_is_correct_value() {
        let expected = EntityTypeId::parse_str("05ca041b-8779-4dcf-9e39-42ca823076fc");
        let actual = CardInDeck::entity_type_id();

        assert_eq!(expected, actual);
    }
}
