use crate::game_logic::cards::UnitCardDefinition;

#[derive(Debug, Default)]
pub struct Hand {
    cards: Vec<Box<dyn UnitCardDefinition>>,
}

impl Hand {
    pub fn len(&self) -> usize {
        self.cards.len()
    }
}
