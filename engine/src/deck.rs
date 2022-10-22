use crate::{Card, CardDefinition};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    #[must_use]
    pub fn new(cards: Vec<Card>) -> Self {
        Self { cards }
    }

    #[must_use]
    pub const fn new_empty() -> Self {
        Self { cards: Vec::new() }
    }

    /// Add a card to the top of the deck.
    pub fn take_from_top(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /// Add a card to the top of the deck.
    pub fn add_card_to_top(&mut self, card: Card) {
        self.cards.push(card);
    }

    /// Add a card to the bottom of the deck.
    pub fn add_card_to_bottom(&mut self, card: Card) {
        self.cards.insert(0, card);
    }

    /// Randomly reorders all the cards in this deck.
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

#[cfg(test)]
mod tests {
    use crate::{card::CardDefinitionBuilder, Card};

    use super::Deck;

    #[test]
    fn card_can_add_to_top() {
        let mut deck = Deck::new_empty();

        for i in 0..10 {
            let mut builder = CardDefinitionBuilder::new();
            let definition = builder.cost(i).build();

            let card = Card::new(Box::new(definition));

            deck.add_card_to_bottom(card);
        }

        let popped_card = deck.take_from_top();
        assert_eq!(0, popped_card.unwrap().definition().cost());

        let popped_card = deck.take_from_top();
        assert_eq!(1, popped_card.unwrap().definition().cost());

        let popped_card = deck.take_from_top();
        assert_eq!(2, popped_card.unwrap().definition().cost());

        let mut builder = CardDefinitionBuilder::new();
        let definition = builder.cost(100).build();
        deck.add_card_to_top(Card::new(Box::new(definition)));

        let popped_card = deck.take_from_top();
        assert_eq!(100, popped_card.unwrap().definition().cost());
    }
}
