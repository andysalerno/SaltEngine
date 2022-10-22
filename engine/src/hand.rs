use crate::{Card, CardDefinition, CardId};

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

    pub fn card(&self, card_id: CardId) -> Option<&Card> {
        self.cards.iter().find(|c| c.id() == card_id)
    }

    pub fn cards(&self) -> impl Iterator<Item = &Card> {
        self.cards.iter()
    }

    pub fn take_card(&mut self, card_id: CardId) -> Option<Card> {
        self.cards
            .iter()
            .position(|c| c.id() == card_id)
            .map(|index| self.cards.remove(index))
    }

    /// Add a card to the right side of the hand.
    pub fn add_to_right(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }
}
