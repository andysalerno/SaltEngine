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
    cur_player_turn: PlayerId,

    // todo: replace with board entities
    player_a_health: i32,
    player_b_health: i32,
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
            cur_player_turn: player_id_a,
            player_a_health: 15,
            player_b_health: 15,
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

    #[must_use]
    pub const fn player_id_a(&self) -> PlayerId {
        self.player_id_a
    }

    #[must_use]
    pub const fn player_id_b(&self) -> PlayerId {
        self.player_id_b
    }

    #[must_use]
    pub fn deck(&self, player_id: PlayerId) -> &Deck {
        match self.player(player_id) {
            Player::PlayerA => &self.deck_player_a,
            Player::PlayerB => &self.deck_player_b,
        }
    }

    #[must_use]
    pub fn deck_mut(&mut self, player_id: PlayerId) -> &mut Deck {
        match self.player(player_id) {
            Player::PlayerA => &mut self.deck_player_a,
            Player::PlayerB => &mut self.deck_player_b,
        }
    }

    #[must_use]
    pub fn hand(&self, player_id: PlayerId) -> &Hand {
        match self.player(player_id) {
            Player::PlayerA => &self.hand_player_a,
            Player::PlayerB => &self.hand_player_b,
        }
    }

    #[must_use]
    pub fn hand_mut(&mut self, player_id: PlayerId) -> &mut Hand {
        match self.player(player_id) {
            Player::PlayerA => &mut self.hand_player_a,
            Player::PlayerB => &mut self.hand_player_b,
        }
    }

    /// Inserts the `Card` at the given `GamePos`. Returns the previous `Card` in that position
    /// if there was one.
    #[must_use]
    pub fn set_card_at_pos(&mut self, pos: GamePos, card: Card) -> Option<Card> {
        self.cards_on_board.insert(pos, card)
    }

    #[must_use]
    pub const fn cur_player_turn(&self) -> PlayerId {
        self.cur_player_turn
    }

    pub fn set_next_cur_player_turn(&mut self) {
        let next_player_turn = match self.player(self.cur_player_turn()) {
            Player::PlayerA => self.player_id_b(),
            Player::PlayerB => self.player_id_a(),
        };

        self.cur_player_turn = next_player_turn;
    }

    #[must_use]
    pub fn player_health(&self, player_id: PlayerId) -> i32 {
        match self.player(player_id) {
            Player::PlayerA => self.player_a_health,
            Player::PlayerB => self.player_b_health,
        }
    }

    #[must_use]
    pub const fn builder(player_id_a: PlayerId, player_id_b: PlayerId) -> GameStateBuilder {
        GameStateBuilder::new(player_id_a, player_id_b)
    }

    #[must_use]
    pub const fn is_game_over(&self) -> bool {
        false
    }

    fn player(&self, player_id: PlayerId) -> Player {
        if player_id == self.player_id_a {
            Player::PlayerA
        } else if player_id == self.player_id_b {
            Player::PlayerB
        } else {
            panic!("Player id was not recognized: {player_id:?}")
        }
    }
}

pub struct GameStateBuilder {
    deck_player_a: Deck,
    deck_player_b: Deck,
    player_id_a: PlayerId,
    player_id_b: PlayerId,
}

impl GameStateBuilder {
    pub const fn new(player_id_a: PlayerId, player_id_b: PlayerId) -> Self {
        Self {
            player_id_a,
            player_id_b,
            deck_player_a: Deck::new_empty(),
            deck_player_b: Deck::new_empty(),
        }
    }

    pub fn with_player_a_deck(&mut self, deck: Deck) -> &mut Self {
        self.deck_player_a = deck;
        self
    }

    pub fn with_player_b_deck(&mut self, deck: Deck) -> &mut Self {
        self.deck_player_b = deck;
        self
    }

    pub fn build(self) -> GameState {
        let mut gamestate = GameState::new(self.player_id_a, self.player_id_b);

        *gamestate.deck_mut(self.player_id_a) = self.deck_player_a;
        *gamestate.deck_mut(self.player_id_b) = self.deck_player_b;

        gamestate
    }
}

enum Player {
    PlayerA,
    PlayerB,
}
