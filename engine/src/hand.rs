use crate::{Card, CardDefinition};

#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new(cards: Vec<Card>) -> Self {
        Self { cards }
    }

    pub const fn new_empty() -> Self {
        Self { cards: Vec::new() }
    }

    /// Add a card to the right side of the hand.
    pub fn add_to_right(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }
}
