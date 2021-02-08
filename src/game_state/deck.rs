use super::UnitCardInstance;

#[derive(Debug, Default)]
pub struct Deck {
    cards: Vec<UnitCardInstance>,
}

impl Deck {
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn draw_card(&mut self) -> Option<UnitCardInstance> {
        self.cards.pop()
    }

    pub fn new(cards: Vec<UnitCardInstance>) -> Self {
        Self { cards }
    }
}
