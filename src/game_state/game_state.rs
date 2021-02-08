use super::{
    board::{Board, BoardPos},
    Deck, Hand, UnitCardInstance, UnitCardInstanceId,
};
use crate::id::Id;

pub struct GameState {
    player_b_id: Id,
    player_a_id: Id,
    cur_player_turn: Id,

    player_a_hand: Hand,
    player_a_deck: Deck,

    player_b_hand: Hand,
    player_b_deck: Deck,

    player_a_health: i32,
    player_b_health: i32,

    player_a_mana: u32,
    player_b_mana: u32,

    board: Box<Board>,
}

enum PlayerAB {
    PlayerA,
    PlayerB,
}

const BOARD_LEN: usize = 6;
const STARTING_HEALTH: i32 = 30;

impl GameState {
    pub fn is_game_over(&self) -> bool {
        self.player_a_health <= 0 || self.player_b_health <= 0
    }

    pub fn initial_state(
        player_a_id: Id,
        player_a_deck: Deck,
        player_b_id: Id,
        player_b_deck: Deck,
    ) -> Self {
        Self {
            player_a_id,
            player_b_id,
            cur_player_turn: player_a_id,
            player_a_health: STARTING_HEALTH,
            player_b_health: STARTING_HEALTH,

            player_a_hand: Hand::default(),
            player_a_deck,

            player_b_hand: Hand::default(),
            player_b_deck,

            player_a_mana: 0,
            player_b_mana: 0,
            board: Box::new(Board::new(BOARD_LEN, player_a_id, player_b_id)),
        }
    }

    pub fn cur_player_id(&self) -> Id {
        self.cur_player_turn
    }

    pub fn set_next_player_turn(&mut self) -> Id {
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

    pub fn get_at(&self, pos: BoardPos) -> Option<&UnitCardInstance> {
        self.board.get_at(pos)
    }

    pub fn set_at(&mut self, pos: BoardPos, card_instance: UnitCardInstance) {
        self.board.set_at(pos, card_instance)
    }

    pub fn player_a_id(&self) -> Id {
        self.player_a_id
    }

    pub fn player_b_id(&self) -> Id {
        self.player_b_id
    }

    /// Given the ID of a player, returns the ID of the other player.
    pub fn other_player(&self, player_id: Id) -> Id {
        match self.player_ab(player_id) {
            PlayerAB::PlayerA => self.player_b_id(),
            PlayerAB::PlayerB => self.player_a_id(),
        }
    }

    pub fn hand(&self, player_id: Id) -> &Hand {
        match self.player_ab(player_id) {
            PlayerAB::PlayerA => &self.player_a_hand,
            PlayerAB::PlayerB => &self.player_b_hand,
        }
    }

    pub fn hand_mut(&mut self, player_id: Id) -> &mut Hand {
        match self.player_ab(player_id) {
            PlayerAB::PlayerA => &mut self.player_a_hand,
            PlayerAB::PlayerB => &mut self.player_b_hand,
        }
    }

    pub fn deck(&self, player_id: Id) -> &Deck {
        match self.player_ab(player_id) {
            PlayerAB::PlayerA => &self.player_a_deck,
            PlayerAB::PlayerB => &self.player_b_deck,
        }
    }

    fn deck_mut(&mut self, player_id: Id) -> &mut Deck {
        match self.player_ab(player_id) {
            PlayerAB::PlayerA => &mut self.player_a_deck,
            PlayerAB::PlayerB => &mut self.player_b_deck,
        }
    }

    pub fn player_mana(&self, player_id: Id) -> u32 {
        match self.player_ab(player_id) {
            PlayerAB::PlayerA => self.player_a_mana,
            PlayerAB::PlayerB => self.player_b_mana,
        }
    }

    pub fn reduce_mana(&mut self, player_id: Id, mana_count: u32) {
        let player_mana = match self.player_ab(player_id) {
            PlayerAB::PlayerA => &mut self.player_a_mana,
            PlayerAB::PlayerB => &mut self.player_b_mana,
        };

        *player_mana = *player_mana - mana_count;
    }

    pub fn gain_mana(&mut self, player_id: Id, mana_count: u32) {
        let player_mana = match self.player_ab(player_id) {
            PlayerAB::PlayerA => &mut self.player_a_mana,
            PlayerAB::PlayerB => &mut self.player_b_mana,
        };

        *player_mana = *player_mana + mana_count;
    }

    pub fn get_by_id(&self, id: UnitCardInstanceId) -> &UnitCardInstance {
        self.board.get_by_id(id)
    }

    pub fn get_pos_by_id(&self, id: UnitCardInstanceId) -> BoardPos {
        self.board.get_position_by_id(id)
    }

    pub fn draw_card(&mut self, player_id: Id) -> Option<UnitCardInstance> {
        self.deck_mut(player_id).draw_card()
    }

    pub fn update_by_id(
        &mut self,
        id: UnitCardInstanceId,
        update: impl FnOnce(&mut UnitCardInstance),
    ) {
        self.board.update_by_id(id, update);
    }

    pub fn evaluate_passives(&mut self) {
        // clear out all passive buffs
        self.board_iter()
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
            .board_iter()
            .filter_map(|i| {
                let passive_instance = i.passive_effect_instance();
                passive_instance
                    .map(|p| (p.instance_id(), p.originator_id(), p.definition().update()))
            }) // i.passive_effect_instance().map(|p| p.definition().update()))
            .collect::<Vec<_>>();

        if !effects.is_empty() {
            println!("Evaluating {} passive effects.", effects.len());
        }

        effects
            .into_iter()
            .for_each(|(instance_id, originator_id, updater)| {
                (updater)(instance_id, originator_id, self);
            });
    }

    /// An iterator over all unit instances on the entire board.
    pub fn board_iter(&self) -> impl Iterator<Item = &UnitCardInstance> {
        self.board.iter()
    }

    fn player_ab(&self, player_id: Id) -> PlayerAB {
        if player_id == self.player_a_id() {
            PlayerAB::PlayerA
        } else if player_id == self.player_b_id() {
            PlayerAB::PlayerB
        } else {
            panic!("Unknown player id: {:?}", player_id)
        }
    }
}
