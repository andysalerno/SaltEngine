use crate::CardDefinition;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<CardDefinition>,
}

impl Deck {
    #[must_use]
    pub fn new(cards: Vec<CardDefinition>) -> Self {
        Self { cards }
    }

    #[must_use]
    pub const fn new_empty() -> Self {
        Self { cards: Vec::new() }
    }

    /// Add a card to the top of the deck.
    pub fn take_from_top(&mut self) -> Option<CardDefinition> {
        self.cards.pop()
    }

    /// Add a card to the top of the deck.
    pub fn add_card_to_top(&mut self, card: CardDefinition) {
        self.cards.push(card);
    }

    /// Add a card to the bottom of the deck.
    pub fn add_card_to_bottom(&mut self, card: CardDefinition) {
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
    use crate::card::CardDefinitionBuilder;

    use super::Deck;

    #[test]
    fn card_can_add_to_top() {
        let mut deck = Deck::new_empty();

        for i in 0..10 {
            let mut builder = CardDefinitionBuilder::new();
            let card = builder.cost(i).build();

            deck.add_card_to_bottom(card);
        }

        let popped_card = deck.take_from_top();
        assert_eq!(0, popped_card.unwrap().cost());

        let popped_card = deck.take_from_top();
        assert_eq!(1, popped_card.unwrap().cost());

        let popped_card = deck.take_from_top();
        assert_eq!(2, popped_card.unwrap().cost());

        let mut builder = CardDefinitionBuilder::new();
        let card = builder.cost(100).build();
        deck.add_card_to_top(card);

        let popped_card = deck.take_from_top();
        assert_eq!(100, popped_card.unwrap().cost());
    }
}
