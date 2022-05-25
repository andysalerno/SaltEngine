use entity_arena::{id::EntityTypeId, IsEntity};
use id_macro::id;
use isentity_macro_derive::entity;
use serde::{Deserialize, Serialize};

#[id]
pub struct CreatureDefinitionId;

pub struct CreatureDefinition {
    definition_id: CreatureDefinitionId,
    title: String,
    base_cost: i32,
    text: String,
    flavor_text: String,
    base_attack: i32,
    base_health: i32,
    width: usize,
    // placeable_at: Position,
}

#[id]
pub struct CreatureInstanceId;

#[derive(Clone, Serialize, Deserialize)]
#[entity("95b2cf09-d49a-4a18-93f2-5a75e8e13d38")]
pub struct CreatureInstance {
    instance_id: CreatureInstanceId,
    definition_id: CreatureDefinitionId,
    cost: i32,
    attack: i32,
    health: i32,
}

impl CreatureInstance {
    pub fn new_from_definition_id(definition_id: CreatureDefinitionId) -> Self {
        Self {
            instance_id: CreatureInstanceId::new(),
            definition_id,
            cost: 0,
            attack: 0,
            health: 0,
        }
    }

    pub fn definition_id(&self) -> CreatureDefinitionId {
        self.definition_id
    }
}

pub mod builder {
    use super::{CreatureDefinition, CreatureDefinitionId};

    pub struct CreatureDefinitionBuilder {
        definition_id: CreatureDefinitionId,
        title: String,
        base_cost: i32,
        text: String,
        flavor_text: String,
        base_attack: i32,
        base_health: i32,
        width: usize,
    }

    impl CreatureDefinitionBuilder {
        pub fn new() -> Self {
            Self {
                definition_id: CreatureDefinitionId::new(),
                title: String::new(),
                base_cost: 0,
                text: String::new(),
                flavor_text: String::new(),
                base_attack: 0,
                base_health: 0,
                width: 0,
            }
        }

        pub fn definition_id(&mut self, id: CreatureDefinitionId) -> &mut Self {
            self.definition_id = id;
            self
        }

        pub fn title(&mut self, title: String) -> &mut Self {
            self.title = title;
            self
        }

        pub fn cost(&mut self, cost: i32) -> &mut Self {
            self.base_cost = cost;
            self
        }

        pub fn text(&mut self, text: String) -> &mut Self {
            self.text = text;
            self
        }

        pub fn flavor_text(&mut self, flavor_text: String) -> &mut Self {
            self.flavor_text = flavor_text;
            self
        }

        pub fn attack(&mut self, attack: i32) -> &mut Self {
            self.base_attack = attack;
            self
        }

        pub fn health(&mut self, health: i32) -> &mut Self {
            self.base_health = health;
            self
        }

        pub fn width(&mut self, width: usize) -> &mut Self {
            self.width = width;
            self
        }

        pub fn build(&self) -> CreatureDefinition {
            CreatureDefinition {
                definition_id: self.definition_id.clone(),
                title: self.title.clone(),
                base_cost: self.base_cost,
                text: self.text.clone(),
                flavor_text: self.flavor_text.clone(),
                base_attack: self.base_attack,
                base_health: self.base_health,
                width: self.width,
            }
        }
    }
}
