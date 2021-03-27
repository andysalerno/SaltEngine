use super::{
    board::{Board, BoardPos, BoardView, RowId},
    hand::HandView,
    Deck, Hand, IterAddons, IteratorAny, PlayerId, UnitCardInstance, UnitCardInstanceId,
};
use serde::{Deserialize, Serialize};
use std::slice::Iter;

pub trait GameStateView<'a> {
    type HandView: HandView<'a>;
    type BoardView: BoardView<'a>;

    fn cur_player_turn(&self) -> PlayerId;

    fn player_mana(&self, player_id: PlayerId) -> u32;
    fn player_mana_limit(&self, player_id: PlayerId) -> u32;

    fn board(&self) -> &Self::BoardView;
    fn hand(&self, player_id: PlayerId) -> &Self::HandView;

    fn iter(&'a self) -> Iter<'_, <Self::BoardView as BoardView<'a>>::SlotView> {
        self.board().slots_iter()
    }

    fn opponent_id(&self, player_id: PlayerId) -> PlayerId;
}

impl<'a> GameStateView<'a> for GameState {
    type BoardView = Board;
    type HandView = Hand;

    fn board(&self) -> &Board {
        self.board.as_ref()
    }

    fn hand(&self, player_id: PlayerId) -> &Hand {
        match self.player_ab(player_id) {
            PlayerAB::PlayerA => &self.player_a_hand,
            PlayerAB::PlayerB => &self.player_b_hand,
        }
    }

    fn cur_player_turn(&self) -> PlayerId {
        self.cur_player_turn
    }

    fn player_mana(&self, player_id: PlayerId) -> u32 {
        GameState::player_mana(self, player_id)
    }

    fn player_mana_limit(&self, player_id: PlayerId) -> u32 {
        GameState::player_mana_limit(self, player_id)
    }

    fn opponent_id(&self, player_id: PlayerId) -> PlayerId {
        GameState::opponent_id(self, player_id)
    }
}

pub struct GameState {
    player_b_id: PlayerId,
    player_a_id: PlayerId,
    cur_player_turn: PlayerId,

    player_a_hand: Hand,
    player_a_deck: Deck,

    player_b_hand: Hand,
    player_b_deck: Deck,

    player_a_mana: u32,
    player_a_mana_limit: u32,
    player_b_mana: u32,
    player_b_mana_limit: u32,

    board: Box<Board>,
}

enum PlayerAB {
    PlayerA,
    PlayerB,
}

const BOARD_LEN: usize = 6;

impl GameState {
    pub fn is_game_over(&self) -> bool {
        self.board().hero(self.player_a_id).health() <= 0
            || self.board().hero(self.player_b_id).health() <= 0
    }

    fn player_ab(&self, player_id: PlayerId) -> PlayerAB {
        if player_id == self.player_a_id() {
            PlayerAB::PlayerA
        } else if player_id == self.player_b_id() {
            PlayerAB::PlayerB
        } else {
            panic!("Unknown player id: {:?}", player_id)
        }
    }

    fn player_mana(&self, player_id: PlayerId) -> u32 {
        match self.player_ab(player_id) {
            PlayerAB::PlayerA => self.player_a_mana(),
            PlayerAB::PlayerB => self.player_b_mana(),
        }
    }

    fn player_mana_limit(&self, player_id: PlayerId) -> u32 {
        match self.player_ab(player_id) {
            PlayerAB::PlayerA => self.player_a_mana_limit,
            PlayerAB::PlayerB => self.player_b_mana_limit,
        }
    }

    pub fn initial_state(
        player_a_id: PlayerId,
        player_a_deck: Deck,
        player_b_id: PlayerId,
        player_b_deck: Deck,
    ) -> Self {
        let hero_a = super::hero::make_hero_instance();
        let hero_b = super::hero::make_hero_instance();
        let mut board = Box::new(Board::new(BOARD_LEN, player_a_id, player_b_id));

        board.set_creature_at_pos(BoardPos::hero_pos(player_a_id), hero_a);
        board.set_creature_at_pos(BoardPos::hero_pos(player_b_id), hero_b);

        Self {
            player_a_id,
            player_b_id,
            cur_player_turn: player_a_id,

            player_a_hand: Hand::default(),
            player_a_deck,

            player_b_hand: Hand::default(),
            player_b_deck,

            player_a_mana: 0,
            player_a_mana_limit: 0,
            player_b_mana: 0,
            player_b_mana_limit: 0,
            board,
        }
    }

    pub fn cur_player_id(&self) -> PlayerId {
        self.cur_player_turn
    }

    pub fn opponent_id(&self, player_id: PlayerId) -> PlayerId {
        match self.player_ab(player_id) {
            PlayerAB::PlayerA => self.player_b_id(),
            PlayerAB::PlayerB => self.player_a_id(),
        }
    }

    pub fn set_next_player_turn(&mut self) -> PlayerId {
        let next_player = match self.player_ab(self.cur_player_id()) {
            PlayerAB::PlayerA => self.player_b_id(),
            PlayerAB::PlayerB => self.player_a_id(),
        };

        self.cur_player_turn = next_player;
        next_player
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    pub fn player_a_id(&self) -> PlayerId {
        self.player_a_id
    }

    pub fn player_b_id(&self) -> PlayerId {
        self.player_b_id
    }

    /// Given the ID of a player, returns the ID of the other player.
    pub fn other_player(&self, player_id: PlayerId) -> PlayerId {
        match self.player_ab(player_id) {
            PlayerAB::PlayerA => self.player_b_id(),
            PlayerAB::PlayerB => self.player_a_id(),
        }
    }

    pub fn hand(&self, player_id: PlayerId) -> &Hand {
        match self.player_ab(player_id) {
            PlayerAB::PlayerA => &self.player_a_hand,
            PlayerAB::PlayerB => &self.player_b_hand,
        }
    }

    pub fn hand_mut(&mut self, player_id: PlayerId) -> &mut Hand {
        match self.player_ab(player_id) {
            PlayerAB::PlayerA => &mut self.player_a_hand,
            PlayerAB::PlayerB => &mut self.player_b_hand,
        }
    }

    pub fn deck(&self, player_id: PlayerId) -> &Deck {
        match self.player_ab(player_id) {
            PlayerAB::PlayerA => &self.player_a_deck,
            PlayerAB::PlayerB => &self.player_b_deck,
        }
    }

    fn deck_mut(&mut self, player_id: PlayerId) -> &mut Deck {
        match self.player_ab(player_id) {
            PlayerAB::PlayerA => &mut self.player_a_deck,
            PlayerAB::PlayerB => &mut self.player_b_deck,
        }
    }

    /// Resets a player's mana count back to their limit.
    pub fn refresh_player_mana(&mut self, player_id: PlayerId) {
        match self.player_ab(player_id) {
            PlayerAB::PlayerA => self.player_a_mana = self.player_a_mana_limit,
            PlayerAB::PlayerB => self.player_b_mana = self.player_a_mana_limit,
        }
    }

    pub fn reduce_mana(&mut self, player_id: PlayerId, mana_count: u32) {
        let player_mana = match self.player_ab(player_id) {
            PlayerAB::PlayerA => &mut self.player_a_mana,
            PlayerAB::PlayerB => &mut self.player_b_mana,
        };

        *player_mana = *player_mana - mana_count;
    }

    pub fn raise_mana_limit(&mut self, player_id: PlayerId, mana_count: u32) {
        let player_mana_limit = match self.player_ab(player_id) {
            PlayerAB::PlayerA => &mut self.player_a_mana_limit,
            PlayerAB::PlayerB => &mut self.player_b_mana_limit,
        };

        *player_mana_limit = *player_mana_limit + mana_count;
    }

    pub fn draw_card(&mut self, player_id: PlayerId) -> Option<UnitCardInstance> {
        self.deck_mut(player_id).draw_card()
    }

    pub fn update_by_id(
        &mut self,
        id: UnitCardInstanceId,
        update: impl FnOnce(&mut UnitCardInstance),
    ) {
        let creature = self
            .board_mut()
            .creatures_iter_mut()
            .filter(|i| i.id() == id)
            .next()
            .expect("Cannot update_by_id; creature with id not found");

        (update)(creature);
    }

    pub fn is_pos_defended(&self, pos: BoardPos) -> bool {
        if pos.row_id == RowId::FrontRow {
            return false;
        }

        let front_pos = BoardPos::new(pos.player_id, RowId::FrontRow, pos.row_index);

        match self.board().creature_at_pos(front_pos) {
            Some(creature) => creature.definition().is_defender(),
            _ => false,
        }
    }

    pub fn player_has_any_creature(&self, player_id: PlayerId) -> bool {
        self.board()
            .slots_iter()
            .for_player(player_id)
            .creatures()
            .has_any()
    }

    pub fn active_attackers(
        &self,
        player_id: PlayerId,
    ) -> impl Iterator<Item = UnitCardInstanceId> + '_ {
        self.board()
            .slots_iter()
            .for_player(player_id)
            .creature_ids()
    }

    pub fn evaluate_passives(&mut self) {
        // clear out all passive buffs
        self.iter()
            .creatures()
            .flat_map(|i| {
                let id = i.id();

                i.buffs()
                    .iter()
                    .filter(|b| b.is_from_passive())
                    .map(move |b| (id, b.instance_id()))
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(card_inst_id, buff_inst_id)| {
                self.update_by_id(card_inst_id, |i| {
                    i.remove_buff(buff_inst_id);
                });
            });

        let effects = self
            .iter()
            .creatures()
            .filter_map(|i| {
                let passive_instance = i.passive_effect_instance();
                passive_instance
                    .map(|p| (p.instance_id(), p.originator_id(), p.definition().update()))
            })
            .collect::<Vec<_>>();

        effects
            .into_iter()
            .for_each(|(instance_id, originator_id, updater)| {
                (updater)(instance_id, originator_id, self);
            });
    }

    /// Get a reference to the game state's player a mana.
    pub fn player_a_mana(&self) -> u32 {
        self.player_a_mana
    }

    /// Get a reference to the game state's player b mana.
    pub fn player_b_mana(&self) -> u32 {
        self.player_b_mana
    }
}

pub mod player_view {
    use crate::game_state::{
        board::player_view::BoardPlayerView, hand::HandPlayerView, MakePlayerView,
    };

    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct GameStatePlayerView {
        player_id: PlayerId,
        opponent_id: PlayerId,
        cur_player_turn: PlayerId,

        player_hand: HandPlayerView,
        player_deck_len: usize,

        opponent_hand_len: usize,
        opponent_deck_len: usize,

        player_mana: u32,
        player_mana_limit: u32,
        opponent_mana: u32,
        opponent_mana_limit: u32,

        board: BoardPlayerView,
    }

    impl GameStatePlayerView {
        /// Get the game state player view's player id.
        pub fn player_id(&self) -> PlayerId {
            self.player_id
        }

        /// Get the game state player view's opponent id.
        pub fn opponent_id(&self) -> PlayerId {
            self.opponent_id
        }
    }

    enum PlayerOpponent {
        Player,
        Opponent,
    }

    impl MakePlayerView for GameState {
        type TOut = GameStatePlayerView;

        fn player_view(&self, player_viewing: PlayerId) -> GameStatePlayerView {
            let enemy_id = self.other_player(player_viewing);
            GameStatePlayerView {
                player_id: player_viewing,
                opponent_id: enemy_id,
                cur_player_turn: self.cur_player_turn,
                player_hand: self.hand(player_viewing).player_view(player_viewing),
                player_deck_len: self.deck(player_viewing).len(),
                opponent_hand_len: self.hand(enemy_id).len(),
                opponent_deck_len: self.deck(enemy_id).len(),
                player_mana: self.player_mana(player_viewing),
                player_mana_limit: self.player_mana_limit(player_viewing),
                opponent_mana: self.player_mana(enemy_id),
                opponent_mana_limit: self.player_mana_limit(enemy_id),
                board: self.board().player_view(player_viewing),
            }
        }
    }

    impl GameStatePlayerView {
        pub fn board(&self) -> &BoardPlayerView {
            &self.board
        }

        pub fn hand(&self) -> &HandPlayerView {
            &self.player_hand
        }

        fn player_or_opponent(&self, player_id: PlayerId) -> PlayerOpponent {
            if player_id == self.player_id {
                PlayerOpponent::Player
            } else if player_id == self.opponent_id {
                PlayerOpponent::Opponent
            } else {
                panic!("Provided player_id is for neither the player nor the opponent, and therefore is not valid")
            }
        }
    }

    impl<'a> GameStateView<'a> for GameStatePlayerView {
        type HandView = HandPlayerView;
        type BoardView = BoardPlayerView;

        fn board(&self) -> &Self::BoardView {
            &self.board
        }

        fn hand(&self, player_id: PlayerId) -> &HandPlayerView {
            if player_id == self.player_id {
                return &self.player_hand;
            }

            panic!("Player view cannot show another player's hand");
        }

        fn cur_player_turn(&self) -> PlayerId {
            self.cur_player_turn
        }

        fn player_mana(&self, player_id: PlayerId) -> u32 {
            match self.player_or_opponent(player_id) {
                PlayerOpponent::Player => self.player_mana,
                PlayerOpponent::Opponent => self.opponent_mana,
            }
        }

        fn player_mana_limit(&self, player_id: PlayerId) -> u32 {
            match self.player_or_opponent(player_id) {
                PlayerOpponent::Player => self.player_mana_limit,
                PlayerOpponent::Opponent => self.opponent_mana_limit,
            }
        }

        fn opponent_id(&self, player_id: PlayerId) -> PlayerId {
            match self.player_or_opponent(player_id) {
                PlayerOpponent::Player => self.opponent_id,
                PlayerOpponent::Opponent => self.player_id,
            }
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::game_state::MakePlayerView;

    use super::*;

    pub fn make_test_state() -> GameState {
        let player_a_deck = Deck::new(Vec::new());
        let player_b_deck = Deck::new(Vec::new());

        let mut state = GameState::initial_state(
            PlayerId::new(),
            player_a_deck,
            PlayerId::new(),
            player_b_deck,
        );

        state.raise_mana_limit(state.player_a_id(), 10);
        state.raise_mana_limit(state.player_b_id(), 10);
        state.refresh_player_mana(state.player_a_id());
        state.refresh_player_mana(state.player_b_id());

        state
    }

    #[test]
    pub fn make_player_view_no_panic() {
        use serde_json;

        let state = make_test_state();

        let player_id = state.player_a_id();
        let player_view = state.player_view(player_id);

        let json = serde_json::to_string(&player_view).unwrap();
        println!("json: {}", json);
        panic!()
    }
}
