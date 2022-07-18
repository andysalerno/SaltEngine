use crate::CardDefinition;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<CardDefinition>,
}

impl Deck {
    pub fn new(cards: Vec<CardDefinition>) -> Self {
        Self { cards }
    }

    pub fn add_card(&mut self, card: CardDefinition) {
        self.cards.push(card);
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}
