use id_macro::id;
use serde::{Deserialize, Serialize};

#[id]
pub struct CardId;

/// An instance of a card in the game,
/// created from some definition.
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Card {
    id: CardId,
    definition: Box<CardDefinition>,
    title: String,
    current_cost: usize,
    current_attack: i16,
    current_health: i16,
}

impl Card {
    #[must_use]
    pub fn new(definition: impl Into<Box<CardDefinition>>) -> Self {
        let definition = definition.into();
        Self {
            id: CardId::new(),
            title: definition.title().to_owned(),
            current_attack: definition.attack(),
            current_cost: definition.cost(),
            current_health: definition.health(),
            definition,
        }
    }

    #[must_use]
    pub fn definition(&self) -> &CardDefinition {
        self.definition.as_ref()
    }

    #[must_use]
    pub const fn current_cost(&self) -> usize {
        self.current_cost
    }

    #[must_use]
    pub const fn current_attack(&self) -> i16 {
        self.current_attack
    }

    #[must_use]
    pub const fn current_health(&self) -> i16 {
        self.current_health
    }

    pub fn set_health(&mut self, next_health: i16) {
        self.current_health = next_health;
    }

    #[must_use]
    pub const fn id(&self) -> CardId {
        self.id
    }
}

/// The definition of a card,
/// including its title, attack, cost, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardDefinition {
    title: String,
    cost: usize,
    attack: i16,
    health: i16,
}

impl CardDefinition {
    #[must_use]
    pub fn builder() -> CardDefinitionBuilder {
        CardDefinitionBuilder::new()
    }

    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    #[must_use]
    pub const fn cost(&self) -> usize {
        self.cost
    }

    #[must_use]
    pub const fn attack(&self) -> i16 {
        self.attack
    }

    #[must_use]
    pub const fn health(&self) -> i16 {
        self.health
    }
}

/// A builder for `CardDefinition`s.
pub struct CardDefinitionBuilder {
    title: String,
    cost: usize,
    attack: i16,
    health: i16,
}

impl CardDefinitionBuilder {
    pub fn new() -> Self {
        Self {
            title: "<unspecified>".into(),
            cost: 0,
            attack: 0,
            health: 0,
        }
    }

    pub fn build(&self) -> CardDefinition {
        CardDefinition {
            title: self.title.clone(),
            cost: self.cost,
            attack: self.attack,
            health: self.health,
        }
    }

    pub fn health(&mut self, health: i16) -> &mut Self {
        self.health = health;
        self
    }

    pub fn attack(&mut self, attack: i16) -> &mut Self {
        self.attack = attack;
        self
    }

    pub fn cost(&mut self, cost: usize) -> &mut Self {
        self.cost = cost;
        self
    }

    pub fn title(&mut self, title: impl AsRef<str>) -> &mut Self {
        let title = title.as_ref().into();
        self.title = title;

        self
    }
}

impl Default for CardDefinitionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Card, CardDefinition};

    #[test]
    fn card_definition_builder_can_build_definition() {
        let mut builder = CardDefinition::builder();
        builder.title("TestCard").cost(3).attack(1).health(4);

        let built_definition = builder.build();

        assert_eq!(3, built_definition.cost());
        assert_eq!(1, built_definition.attack());
        assert_eq!(4, built_definition.health());
        assert_eq!("TestCard", built_definition.title());
    }

    #[test]
    fn card_can_be_created_from_definition() {
        let mut builder = CardDefinition::builder();
        builder.title("TestCard").cost(3).attack(1).health(4);

        let built_definition = builder.build();

        let card = Card::new(Box::new(built_definition));

        assert_eq!(3, card.current_cost());
        assert_eq!(1, card.current_attack());
        assert_eq!(4, card.current_health());
    }
}
