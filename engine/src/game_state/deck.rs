use super::UnitCardInstance;
use rand::seq::SliceRandom;

#[derive(Debug, Default)]
pub struct Deck {
    cards: Vec<UnitCardInstance>,
}

impl Deck {
    #[must_use]
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn draw_card(&mut self) -> Option<UnitCardInstance> {
        self.cards.pop()
    }

    #[must_use]
    pub fn new(cards: Vec<UnitCardInstance>) -> Self {
        Self { cards }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::thread_rng());
    }
}
