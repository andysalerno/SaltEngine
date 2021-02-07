use crate::game_logic::cards::UnitCardDefinition;

#[derive(Debug, Default)]
pub struct Hand {
    cards: Vec<Box<dyn UnitCardDefinition>>,
}

impl Hand {
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn cards(&self) -> &[Box<dyn UnitCardDefinition>] {
        self.cards.as_slice()
    }

    pub fn add_card(&mut self, card: Box<dyn UnitCardDefinition>) {
        self.cards.push(card);
    }
}
