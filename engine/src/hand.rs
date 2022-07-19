use crate::CardDefinition;

#[derive(Debug)]
pub struct Hand {
    cards: Vec<CardDefinition>,
}

impl Hand {
    pub fn new(cards: Vec<CardDefinition>) -> Self {
        Self { cards }
    }

    pub const fn new_empty() -> Self {
        Self { cards: Vec::new() }
    }

    /// Add a card to the right side of the hand.
    pub fn add_to_right(&mut self, card: CardDefinition) {
        self.cards.push(card);
    }
}
