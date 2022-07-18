use crate::{deck::Deck, Card};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum GamePos {}

#[derive(Debug)]
pub struct GameState {
    creatures: HashMap<GamePos, Card>,
    deck_player_a: Deck,
    deck_player_b: Deck,
}

impl GameState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            creatures: HashMap::new(),
            deck_player_a: Deck::new(Vec::new()),
            deck_player_b: Deck::new(Vec::new()),
        }
    }

    #[must_use]
    pub fn card_at_pos(&self, pos: GamePos) -> Option<&Card> {
        self.creatures.get(&pos)
    }

    #[must_use]
    pub fn card_at_pos_mut(&mut self, pos: GamePos) -> Option<&mut Card> {
        self.creatures.get_mut(&pos)
    }

    /// Inserts the `Card` at the given `GamePos`. Returns the previous `Card` in that position
    /// if there was one.
    #[must_use]
    pub fn set_card_at_pos(&mut self, pos: GamePos, card: Card) -> Option<Card> {
        self.creatures.insert(pos, card)
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
