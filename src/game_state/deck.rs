use crate::game_logic::cards::UnitCardDefinition;

#[derive(Debug, Default)]
pub struct Deck {
    cards: Vec<Box<dyn UnitCardDefinition>>,
}

impl Deck {
    pub fn len(&self) -> usize {
        self.cards.len()
    }
}
