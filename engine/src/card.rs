#[derive(Debug)]
pub struct Card {}

#[derive(Debug)]
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
    use crate::CardDefinition;

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
}
