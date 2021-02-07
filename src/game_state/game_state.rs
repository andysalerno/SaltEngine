use super::{
    board::{Board, BoardPos},
    Deck, Hand, UnitCardBoardInstance, UnitCardBoardInstanceId,
};
use crate::{game_logic::cards::UnitCardDefinition, id::Id};

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

const BOARD_LEN: usize = 6;
const STARTING_HEALTH: i32 = 30;

impl GameState {
    pub fn is_game_over(&self) -> bool {
        self.player_a_health <= 0 || self.player_b_health <= 0
    }

    pub fn new(player_a_id: Id, player_b_id: Id) -> Self {
        Self {
            player_a_id,
            player_b_id,
            cur_player_turn: player_a_id,
            player_a_health: STARTING_HEALTH,
            player_b_health: STARTING_HEALTH,

            player_a_hand: Hand::default(),
            player_a_deck: Deck::default(),

            player_b_hand: Hand::default(),
            player_b_deck: Deck::default(),

            player_a_mana: 0,
            player_b_mana: 0,
            board: Box::new(Board::new(BOARD_LEN, player_a_id, player_b_id)),
        }
    }

    pub fn cur_player_turn(&self) -> Id {
        self.cur_player_turn
    }

    pub fn set_next_player_turn(&mut self) -> Id {
        let cur_player = self.cur_player_turn();

        let next_player = if cur_player == self.player_a_id() {
            self.player_b_id()
        } else if cur_player == self.player_b_id() {
            self.player_a_id()
        } else {
            panic!()
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

    pub fn get_at(&self, pos: BoardPos) -> Option<&UnitCardBoardInstance> {
        self.board.get_at(pos)
    }

    pub fn set_at(&mut self, pos: BoardPos, card_instance: UnitCardBoardInstance) {
        self.board.set_at(pos, card_instance)
    }

    pub fn player_a_id(&self) -> Id {
        self.player_a_id
    }

    pub fn player_b_id(&self) -> Id {
        self.player_b_id
    }

    pub fn player_mana(&self, player_id: Id) -> u32 {
        if player_id == self.player_a_id() {
            self.player_a_mana
        } else if player_id == self.player_b_id() {
            self.player_b_mana
        } else {
            panic!("Unknown player id: {:?}", player_id)
        }
    }

    pub fn get_by_id(&self, id: UnitCardBoardInstanceId) -> &UnitCardBoardInstance {
        self.board.get_by_id(id)
    }

    pub fn get_pos_by_id(&self, id: UnitCardBoardInstanceId) -> BoardPos {
        self.board.get_position_by_id(id)
    }

    pub fn update_by_id(
        &mut self,
        id: UnitCardBoardInstanceId,
        update: impl FnOnce(&mut UnitCardBoardInstance),
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
    pub fn board_iter(&self) -> impl Iterator<Item = &UnitCardBoardInstance> {
        self.board.iter()
    }
}
