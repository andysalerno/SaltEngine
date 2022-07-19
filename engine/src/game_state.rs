use crate::{deck::Deck, hand::Hand, Card, PlayerId};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum GamePos {}

#[derive(Debug)]
pub struct GameState {
    cards_on_board: HashMap<GamePos, Card>,
    deck_player_a: Deck,
    deck_player_b: Deck,
    hand_player_a: Hand,
    hand_player_b: Hand,
    player_id_a: PlayerId,
    player_id_b: PlayerId,
}

impl GameState {
    #[must_use]
    pub fn new(player_id_a: PlayerId, player_id_b: PlayerId) -> Self {
        Self {
            cards_on_board: HashMap::new(),
            deck_player_a: Deck::new(Vec::new()),
            deck_player_b: Deck::new(Vec::new()),
            hand_player_a: Hand::new_empty(),
            hand_player_b: Hand::new_empty(),
            player_id_a,
            player_id_b,
        }
    }

    #[must_use]
    pub fn card_at_pos(&self, pos: GamePos) -> Option<&Card> {
        self.cards_on_board.get(&pos)
    }

    #[must_use]
    pub fn card_at_pos_mut(&mut self, pos: GamePos) -> Option<&mut Card> {
        self.cards_on_board.get_mut(&pos)
    }

    /// Inserts the `Card` at the given `GamePos`. Returns the previous `Card` in that position
    /// if there was one.
    #[must_use]
    pub fn set_card_at_pos(&mut self, pos: GamePos, card: Card) -> Option<Card> {
        self.cards_on_board.insert(pos, card)
    }
}
