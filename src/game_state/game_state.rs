use super::{
    board::{Board, BoardPos, BoardSlot, RowId},
    Deck, Hand, PlayerId, UnitCardInstance, UnitCardInstanceId,
};

pub struct GameState {
    player_b_id: PlayerId,
    player_a_id: PlayerId,
    cur_player_turn: PlayerId,

    player_a_hand: Hand,
    player_a_deck: Deck,

    player_b_hand: Hand,
    player_b_deck: Deck,

    player_a_health: i32,
    player_b_health: i32,

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
const STARTING_HEALTH: i32 = 30;

impl GameState {
    pub fn is_game_over(&self) -> bool {
        self.player_a_health <= 0 || self.player_b_health <= 0
    }

    pub fn initial_state(
        player_a_id: PlayerId,
        player_a_deck: Deck,
        player_b_id: PlayerId,
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
            player_a_mana_limit: 0,
            player_b_mana: 0,
            player_b_mana_limit: 0,
            board: Box::new(Board::new(BOARD_LEN, player_a_id, player_b_id)),
        }
    }

    pub fn cur_player_id(&self) -> PlayerId {
        self.cur_player_turn
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

    pub fn creature_at_pos(&self, pos: BoardPos) -> Option<&UnitCardInstance> {
        self.board.creature_at_pos(pos)
    }

    pub fn set_creature_at_pos(&mut self, pos: BoardPos, card_instance: UnitCardInstance) {
        self.board.set_creature_at_pos(pos, card_instance)
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

    pub fn player_mana(&self, player_id: PlayerId) -> u32 {
        match self.player_ab(player_id) {
            PlayerAB::PlayerA => self.player_a_mana,
            PlayerAB::PlayerB => self.player_b_mana,
        }
    }

    /// Resets a player's mana count back to their limit.
    pub fn refresh_player_mana(&mut self, player_id: PlayerId) {
        match self.player_ab(player_id) {
            PlayerAB::PlayerA => self.player_a_mana = self.player_a_mana_limit,
            PlayerAB::PlayerB => self.player_b_mana = self.player_a_mana_limit,
        }
    }

    pub fn player_mana_limit(&self, player_id: PlayerId) -> u32 {
        match self.player_ab(player_id) {
            PlayerAB::PlayerA => self.player_a_mana_limit,
            PlayerAB::PlayerB => self.player_b_mana_limit,
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

    pub fn creature_instance(&self, id: UnitCardInstanceId) -> &UnitCardInstance {
        self.board.creature_instance(id)
    }

    pub fn creature_instance_mut(&mut self, id: UnitCardInstanceId) -> &mut UnitCardInstance {
        self.board.creature_instance_mut(id)
    }

    pub fn position_with_creature(&self, id: UnitCardInstanceId) -> BoardPos {
        self.board.position_with_creature(id)
    }

    pub fn draw_card(&mut self, player_id: PlayerId) -> Option<UnitCardInstance> {
        self.deck_mut(player_id).draw_card()
    }

    pub fn take_creature(&mut self, id: UnitCardInstanceId) -> UnitCardInstance {
        todo!()
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

        match self.creature_at_pos(front_pos) {
            Some(creature) => creature.definition().is_defender(),
            _ => false,
        }
    }

    pub fn evaluate_passives(&mut self) {
        // clear out all passive buffs
        self.creatures_iter()
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
            .creatures_iter()
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

    /// An iterator over all unit instances on the entire board.
    pub fn creatures_iter(&self) -> impl Iterator<Item = &UnitCardInstance> {
        self.board.creatures_iter()
    }

    pub fn creatures_iter_mut(&mut self) -> impl Iterator<Item = &mut UnitCardInstance> {
        self.board.creatures_iter_mut()
    }

    pub fn player_side(&self, player_id: PlayerId) -> impl Iterator<Item = &BoardSlot> {
        self.board.player_side(player_id).iter()
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
}
