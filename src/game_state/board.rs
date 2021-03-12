use super::{card_instance::UnitCardInstance, PlayerId, UnitCardInstanceId};

const BOARD_WIDTH: usize = 6;
const SLOTS_COUNT: usize = BOARD_WIDTH * 4;

enum PlayerAB {
    PlayerA,
    PlayerB,
}

#[derive(Debug)]
pub struct BoardSlot {
    pos: BoardPos,
    creature: Option<UnitCardInstance>,
}

impl BoardSlot {
    fn new(pos: BoardPos) -> Self {
        Self {
            pos,
            creature: None,
        }
    }

    pub fn has_creature(&self) -> bool {
        self.maybe_creature().is_some()
    }

    pub fn take_creature(&mut self) -> UnitCardInstance {
        self.creature.take().unwrap()
    }

    pub fn maybe_creature(&self) -> Option<&UnitCardInstance> {
        self.creature.as_ref()
    }

    pub fn maybe_creature_mut(&mut self) -> Option<&mut UnitCardInstance> {
        self.creature.as_mut()
    }

    pub fn set_creature(&mut self, creature: UnitCardInstance) {
        self.creature = Some(creature);
    }

    pub fn pos(&self) -> BoardPos {
        self.pos
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RowId {
    FrontRow,
    BackRow,
    Hero,
}

impl RowId {
    pub fn is_back(&self) -> bool {
        match self {
            RowId::BackRow => true,
            _ => false,
        }
    }

    pub fn is_front(&self) -> bool {
        match self {
            RowId::FrontRow => true,
            _ => false,
        }
    }

    pub fn is_hero(&self) -> bool {
        match self {
            RowId::Hero => true,
            _ => false,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BoardPos {
    pub player_id: PlayerId,
    pub row_id: RowId,
    pub row_index: usize,
}

impl BoardPos {
    pub fn new(player_id: PlayerId, row_id: RowId, row_index: usize) -> Self {
        Self {
            player_id,
            row_id,
            row_index,
        }
    }

    pub fn hero_pos(player_id: PlayerId) -> Self {
        Self {
            player_id,
            row_id: RowId::Hero,
            row_index: 0,
        }
    }

    pub fn row(&self) -> RowId {
        self.row_id
    }
}

pub struct Board {
    player_a_id: PlayerId,
    player_b_id: PlayerId,
    slots: Vec<BoardSlot>,
}

impl Board {
    pub fn new(_size: usize, player_a_id: PlayerId, player_b_id: PlayerId) -> Self {
        let mut slots = Vec::with_capacity(SLOTS_COUNT + 4);

        // playber b
        for i in 0..BOARD_WIDTH {
            let pos = BoardPos::new(player_b_id, RowId::BackRow, i);
            slots.push(BoardSlot::new(pos));
        }
        for i in 0..BOARD_WIDTH {
            let pos = BoardPos::new(player_b_id, RowId::FrontRow, i);
            slots.push(BoardSlot::new(pos));
        }

        // playber a
        for i in 0..BOARD_WIDTH {
            let pos = BoardPos::new(player_a_id, RowId::FrontRow, i);
            slots.push(BoardSlot::new(pos));
        }
        for i in 0..BOARD_WIDTH {
            let pos = BoardPos::new(player_a_id, RowId::BackRow, i);
            slots.push(BoardSlot::new(pos));
        }

        // player a hero
        for i in 0..2 {
            let pos = BoardPos::new(player_a_id, RowId::Hero, i);
            slots.push(BoardSlot::new(pos));
        }

        // player b hero
        for i in 0..2 {
            let pos = BoardPos::new(player_b_id, RowId::Hero, i);
            slots.push(BoardSlot::new(pos));
        }

        Self {
            player_a_id,
            player_b_id,
            slots,
        }
    }

    fn player_ab(&self, player_id: PlayerId) -> PlayerAB {
        if player_id == self.player_a_id {
            PlayerAB::PlayerA
        } else if player_id == self.player_b_id {
            PlayerAB::PlayerB
        } else {
            panic!("Unknown player id: {:?}", player_id)
        }
    }

    /// The range for the entire board.
    fn board_range(&self, _player_id: PlayerId) -> std::ops::Range<usize> {
        0..SLOTS_COUNT
    }

    /// The range for the given player.
    fn player_range(&self, player_id: PlayerId) -> std::ops::Range<usize> {
        match self.player_ab(player_id) {
            PlayerAB::PlayerB => front_half(0..SLOTS_COUNT),
            PlayerAB::PlayerA => end_half(0..SLOTS_COUNT),
        }
    }

    /// The range for the given player's row.
    fn row_range(&self, player_id: PlayerId, row_id: RowId) -> std::ops::Range<usize> {
        // Hero row is a special case
        if row_id == RowId::Hero {
            let player_offset = match self.player_ab(player_id) {
                PlayerAB::PlayerA => 0,
                PlayerAB::PlayerB => 2,
            };

            let start = SLOTS_COUNT + player_offset;
            let end = start + 2;

            return start..end;
        }

        let player_range = self.player_range(player_id);

        match self.player_ab(player_id) {
            PlayerAB::PlayerB => match row_id {
                RowId::BackRow => front_half(player_range),
                RowId::FrontRow => end_half(player_range),
                RowId::Hero => panic!("hero is not part of row range"),
            },
            PlayerAB::PlayerA => match row_id {
                RowId::FrontRow => front_half(player_range),
                RowId::BackRow => end_half(player_range),
                RowId::Hero => panic!("hero is not part of row range"),
            },
        }
    }

    /// True if there are 'n_slots' starting at 'pos' within one row.
    /// Does not consider whether the slots are occupied or not.
    pub fn check_slots_row_boundary(&self, pos: BoardPos, n_slots: usize) -> bool {
        let row = self.player_row(pos.player_id, pos.row());
        let space_in_row = row.len() - pos.row_index;
        space_in_row >= n_slots
    }

    /// An iterator over all slots on the entire board (even empty ones).
    pub fn slots_iter(&self) -> impl Iterator<Item = &BoardSlot> {
        self.slots.iter()
    }

    /// An iterator over all slots on the entire board (even empty ones).
    pub fn slots_iter_mut(&mut self) -> impl Iterator<Item = &mut BoardSlot> {
        self.slots.iter_mut()
    }

    /// An iterator over all the creatures on the board.
    pub fn creatures_iter(&self) -> impl Iterator<Item = &UnitCardInstance> {
        self.slots_iter().filter_map(|s| s.maybe_creature())
    }

    /// An iterator over all the creatures on the board.
    pub fn creatures_iter_mut(&mut self) -> impl Iterator<Item = &mut UnitCardInstance> {
        self.slots_iter_mut().filter_map(|s| s.maybe_creature_mut())
    }

    pub fn player_side(&self, player_id: PlayerId) -> &[BoardSlot] {
        &self.slots[self.player_range(player_id)]
    }

    pub fn player_side_mut(&mut self, player_id: PlayerId) -> &mut [BoardSlot] {
        let range = self.player_range(player_id);
        &mut self.slots[range]
    }

    pub fn player_row(&self, player_id: PlayerId, row: RowId) -> &[BoardSlot] {
        &self.slots[self.row_range(player_id, row)]
    }

    pub fn player_row_mut(&mut self, player_id: PlayerId, row: RowId) -> &mut [BoardSlot] {
        let range = self.row_range(player_id, row);
        &mut self.slots[range]
    }

    /// An iterator over the slots on a player's side that have creatures.
    /// Creatures of width > 1 only appear once, in their leftmost slot.
    pub fn player_creatures(&self, player_id: PlayerId) -> impl Iterator<Item = &BoardSlot> {
        self.player_side(player_id)
            .iter()
            .filter(|s| s.has_creature())
    }

    pub fn take_creature_by_id(&mut self, id: UnitCardInstanceId) -> UnitCardInstance {
        self.slot_with_creature_mut(id).take_creature()
    }

    pub fn companion_slot(&self, pos: BoardPos) -> &BoardSlot {
        let mut companion_pos = pos.clone();

        let companion_row = match pos.row() {
            RowId::BackRow => RowId::FrontRow,
            RowId::FrontRow => RowId::BackRow,
            RowId::Hero => panic!("Hero slot has no companion."),
        };

        companion_pos.row_id = companion_row;

        self.slot_at_pos(companion_pos)
    }

    pub fn companion_creature(&self, pos: BoardPos) -> Option<&UnitCardInstance> {
        self.companion_slot(pos).maybe_creature()
    }

    pub fn creature_at_pos(&self, pos: BoardPos) -> Option<&UnitCardInstance> {
        if pos.row() == RowId::Hero {
            let player_offset = match self.player_ab(pos.player_id) {
                PlayerAB::PlayerA => 0,
                PlayerAB::PlayerB => 2,
            };
        }

        let row = self.player_row(pos.player_id, pos.row_id);

        // start at pos.row_index, and work back, in case there's
        // a creature taking up multiple rows
        for i in (0..=pos.row_index).rev() {
            let distance = pos.row_index - i;
            let occupant = &row[i];

            if let Some(occupant) = occupant.maybe_creature() {
                if occupant.width() > distance {
                    return Some(occupant);
                } else {
                    return None;
                }
            }
        }

        return None;
    }

    pub fn hero(&self, player_id: PlayerId) -> &UnitCardInstance {
        let pos = BoardPos::hero_pos(player_id);
        self.creature_at_pos(pos).expect("must have a hero")
    }

    pub fn set_creature_at_pos(&mut self, pos: BoardPos, card_instance: UnitCardInstance) {
        if let Some(existing) = self.creature_at_pos(pos) {
            panic!(
                "Could not set at pos {:?} due to existing occupant: {:?}",
                pos,
                existing.id()
            );
        }

        if pos.row() == RowId::Hero {
            let slot = self.slot_at_pos_mut(pos);
            slot.set_creature(card_instance);
        } else {
            let row = self.player_row_mut(pos.player_id, pos.row_id);
            row[pos.row_index].set_creature(card_instance);
        }
    }

    /// The single slot where the creature instance exists.
    pub fn slot_with_creature(&self, id: UnitCardInstanceId) -> &BoardSlot {
        self.slots_iter()
            .filter(|s| s.maybe_creature().map(|c| c.id()) == Some(id))
            .next()
            .expect(&format!("Creature instance with id {:?} not found.", id))
    }

    /// A slice starting at the slot where the creature instance exists,
    /// and including all subsequent slots it occupies (if the creature has a Width of more than 1 slot).
    pub fn slots_with_creature(&self, id: UnitCardInstanceId) -> &[BoardSlot] {
        let (start_index, slot) = self
            .slots
            .iter()
            .enumerate()
            .filter(|(_, slot)| slot.maybe_creature().map(|c| c.id()) == Some(id))
            .next()
            .expect(&format!("Creature instance with id {:?} not found.", id));

        let creature_width = slot.maybe_creature().unwrap().width();

        &self.slots[start_index..start_index + creature_width]
    }

    pub fn slot_with_creature_mut(&mut self, id: UnitCardInstanceId) -> &mut BoardSlot {
        self.slots_iter_mut()
            .collect::<Vec<_>>()
            .into_iter()
            .filter(|s| s.maybe_creature().map(|c| c.id()) == Some(id))
            .next()
            .expect(&format!("Creature instance with id {:?} not found.", id))
    }

    pub fn slot_at_pos_mut(&mut self, pos: BoardPos) -> &mut BoardSlot {
        self.slots_iter_mut()
            .filter(|s| s.pos() == pos)
            .next()
            .expect("The position was not a valid board slot.")
    }

    pub fn slot_at_pos(&self, pos: BoardPos) -> &BoardSlot {
        self.slots_iter()
            .filter(|s| s.pos() == pos)
            .next()
            .expect("The position was not a valid board slot.")
    }

    pub fn position_with_creature(&self, id: UnitCardInstanceId) -> BoardPos {
        self.slot_with_creature(id).pos()
    }

    pub fn creature_instance(&self, id: UnitCardInstanceId) -> &UnitCardInstance {
        self.slot_with_creature(id).maybe_creature().unwrap()
    }

    pub fn creature_instance_mut(&mut self, id: UnitCardInstanceId) -> &mut UnitCardInstance {
        self.slot_with_creature_mut(id)
            .maybe_creature_mut()
            .unwrap()
    }
}

fn front_half(ops: std::ops::Range<usize>) -> std::ops::Range<usize> {
    // ex: 10..20. includes 10 thru 19, with 10 total elements

    // 20 - 10 = 10
    let len = ops.end - ops.start;
    assert!(len % 2 == 0);

    // 10..15 (5 elements, 10-14 inclusive)
    ops.start..ops.start + len / 2
}

fn end_half(ops: std::ops::Range<usize>) -> std::ops::Range<usize> {
    // ex: 10..20. includes 10 thru 19, with 10 total elements

    // 20 - 10 = 10
    let len = ops.end - ops.start;
    assert!(len % 2 == 0);

    // 10 + (10 / 2) .. 20 => 15..20
    ops.start + len / 2..ops.end
}
