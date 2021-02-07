use crate::game_logic::cards::UnitCardDefinition;

#[derive(Debug, Default)]
pub struct Deck {
    cards: Vec<Box<dyn UnitCardDefinition>>,
}

impl Deck {
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn draw_card(&mut self) -> Option<Box<dyn UnitCardDefinition>> {
        self.cards.pop()
    }

    pub fn new(cards: Vec<Box<dyn UnitCardDefinition>>) -> Self {
        Self { cards }
    }
}
