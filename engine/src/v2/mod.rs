use id_macro::id;
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

pub struct CreatureInstance {
    instance_id: CreatureDefinitionId,
    definition_id: CreatureDefinitionId,
    cost: i32,
    attack: i32,
    health: i32,
}

mod builder {
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
