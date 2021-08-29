use super::{
    card_instance::{UnitCardInstance, UnitCardInstanceView},
    PlayerId, UnitCardInstanceId,
};
use serde::{Deserialize, Serialize};

const BOARD_WIDTH: usize = 6;
const SLOTS_COUNT: usize = BOARD_WIDTH * 4;

enum PlayerAB {
    PlayerA,
    PlayerB,
}

/// A view of a particular board pos.
pub trait BoardSlotView<'a> {
    type CardInstanceView: UnitCardInstanceView<'a>;

    /// The BoardPos this view represents.
    fn pos(&self) -> BoardPos;

    /// The view of the creature on this pos, if there is one.
    fn maybe_creature(&self) -> Option<&Self::CardInstanceView>;
}

/// A slot on the board, which has a position
/// and possibly a creature.
#[derive(Debug)]
pub struct BoardSlot {
    pos: BoardPos,
    creature: Option<UnitCardInstance>,
}

impl<'a> BoardSlotView<'a> for BoardSlot {
    type CardInstanceView = UnitCardInstance;

    fn pos(&self) -> BoardPos {
        self.pos()
    }

    fn maybe_creature(&self) -> Option<&Self::CardInstanceView> {
        self.maybe_creature()
    }
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

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
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

fn player_ab<'a, B>(board: &B, player_id: PlayerId) -> PlayerAB
where
    B: BoardView<'a> + ?Sized,
{
    if player_id == board.player_a_id() {
        PlayerAB::PlayerA
    } else if player_id == board.player_b_id() {
        PlayerAB::PlayerB
    } else {
        panic!("Player is was neither player A nor player B, and therefore is not valid")
    }
}

pub trait BoardView<'a> {
    type SlotView: BoardSlotView<'a>;

    fn player_a_id(&self) -> PlayerId;
    fn player_b_id(&self) -> PlayerId;
    fn slots(&self) -> &[Self::SlotView];

    /// True if there are 'n_slots' starting at 'pos' within one row.
    /// Does not consider whether the slots are occupied or not.
    fn is_range_in_row(&self, pos: BoardPos, n_slots: usize) -> bool {
        let row = self.player_row(pos.player_id, pos.row());
        let space_in_row = row.len() - pos.row_index;
        space_in_row >= n_slots
    }

    fn slots_iter(&self) -> std::slice::Iter<'_, <Self as BoardView<'a>>::SlotView> {
        self.slots().iter()
    }

    fn slot_with_creature(&self, id: UnitCardInstanceId) -> &<Self as BoardView<'a>>::SlotView {
        self.slots_iter()
            .filter(|s| s.maybe_creature().map(|c| c.id()) == Some(id))
            .next()
            .expect(&format!("Creature instance with id {:?} not found.", id))
    }

    /// A slice starting at the slot where the creature instance exists,
    /// and including all subsequent slots it occupies (if the creature has a Width of more than 1 slot).
    fn slots_with_creature(&self, id: UnitCardInstanceId) -> &[<Self as BoardView<'a>>::SlotView] {
        let (start_index, slot) = self
            .slots_iter()
            .enumerate()
            .filter(|(_, slot)| slot.maybe_creature().map(|c| c.id()) == Some(id))
            .next()
            .expect(&format!("Creature instance with id {:?} not found.", id));

        let creature_width = slot.maybe_creature().unwrap().width();

        &self.slots()[start_index..start_index + creature_width]
    }

    fn slot_at_pos(&self, pos: BoardPos) -> &<Self as BoardView<'a>>::SlotView {
        self.slots_iter()
            .filter(|s| s.pos() == pos)
            .next()
            .expect("The position was not a valid board slot.")
    }

    fn pos_with_creature(&self, id: UnitCardInstanceId) -> BoardPos {
        self.slot_with_creature(id).pos()
    }

    fn creature_instance(
        &'a self,
        id: UnitCardInstanceId,
    ) -> &<<Self as BoardView<'a>>::SlotView as BoardSlotView<'a>>::CardInstanceView {
        self.slot_with_creature(id).maybe_creature().unwrap()
    }

    /// The range for the entire board.
    fn board_range(&self, _player_id: PlayerId) -> std::ops::Range<usize> {
        0..SLOTS_COUNT
    }

    /// The range for the given player.
    fn player_range(&self, player_id: PlayerId) -> std::ops::Range<usize> {
        match player_ab(self, player_id) {
            PlayerAB::PlayerB => front_half(0..SLOTS_COUNT),
            PlayerAB::PlayerA => end_half(0..SLOTS_COUNT),
        }
        // match self.player_ab(player_id) {
        //     PlayerAB::PlayerB => front_half(0..SLOTS_COUNT),
        //     PlayerAB::PlayerA => end_half(0..SLOTS_COUNT),
        // }
    }

    /// The range for the given player's row.
    fn row_range(&self, player_id: PlayerId, row_id: RowId) -> std::ops::Range<usize> {
        // Hero row is a special case
        if row_id == RowId::Hero {
            let player_offset = match player_ab(self, player_id) {
                PlayerAB::PlayerA => 0,
                PlayerAB::PlayerB => 2,
            };

            let start = SLOTS_COUNT + player_offset;
            let end = start + 2;

            return start..end;
        }

        let player_range = self.player_range(player_id);

        match player_ab(self, player_id) {
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

    fn player_row(&self, player_id: PlayerId, row: RowId) -> &[<Self as BoardView<'a>>::SlotView] {
        &self.slots()[self.row_range(player_id, row)]
    }

    fn hero(
        &'a self,
        player_id: PlayerId,
    ) -> &<<Self as BoardView<'a>>::SlotView as BoardSlotView<'a>>::CardInstanceView {
        let pos = BoardPos::hero_pos(player_id);
        self.creature_at_pos(pos).expect("must have a hero")
    }

    fn creature_at_pos(
        &'a self,
        pos: BoardPos,
    ) -> Option<&<<Self as BoardView<'a>>::SlotView as BoardSlotView<'a>>::CardInstanceView> {
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
}

impl<'a> BoardView<'a> for Board {
    type SlotView = BoardSlot;

    fn player_a_id(&self) -> PlayerId {
        self.player_a_id
    }

    fn player_b_id(&self) -> PlayerId {
        self.player_b_id
    }

    fn slots(&self) -> &[BoardSlot] {
        self.slots.as_slice()
    }
}

#[derive(Debug)]
pub struct Board {
    player_a_id: PlayerId,
    player_b_id: PlayerId,
    slots: Vec<BoardSlot>,
}

impl Board {
    pub fn new(_size: usize, player_a_id: PlayerId, player_b_id: PlayerId) -> Self {
        let mut slots = Vec::with_capacity(SLOTS_COUNT + 4);

        // player b
        for i in 0..BOARD_WIDTH {
            let pos = BoardPos::new(player_b_id, RowId::BackRow, i);
            slots.push(BoardSlot::new(pos));
        }
        for i in 0..BOARD_WIDTH {
            let pos = BoardPos::new(player_b_id, RowId::FrontRow, i);
            slots.push(BoardSlot::new(pos));
        }

        // player a
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
        if player_id == self.player_a_id() {
            PlayerAB::PlayerA
        } else if player_id == self.player_b_id() {
            PlayerAB::PlayerB
        } else {
            panic!("Unknown player id: {:?}", player_id)
        }
    }

    // fn player_ab(&self, player_id: PlayerId) -> PlayerAB {
    //     if player_id == self.player_a_id {
    //         PlayerAB::PlayerA
    //     } else if player_id == self.player_b_id {
    //         PlayerAB::PlayerB
    //     } else {
    //         panic!("Unknown player id: {:?}", player_id)
    //     }
    // }

    // /// The range for the entire board.
    // fn board_range(&self, _player_id: PlayerId) -> std::ops::Range<usize> {
    //     0..SLOTS_COUNT
    // }

    // /// The range for the given player.
    // fn player_range(&self, player_id: PlayerId) -> std::ops::Range<usize> {
    //     match self.player_ab(player_id) {
    //         PlayerAB::PlayerB => front_half(0..SLOTS_COUNT),
    //         PlayerAB::PlayerA => end_half(0..SLOTS_COUNT),
    //     }
    // }

    /// The range for the given player's row.
    // fn row_range(&self, player_id: PlayerId, row_id: RowId) -> std::ops::Range<usize> {
    //     // Hero row is a special case
    //     if row_id == RowId::Hero {
    //         let player_offset = match self.player_ab(player_id) {
    //             PlayerAB::PlayerA => 0,
    //             PlayerAB::PlayerB => 2,
    //         };

    //         let start = SLOTS_COUNT + player_offset;
    //         let end = start + 2;

    //         return start..end;
    //     }

    //     let player_range = self.player_range(player_id);

    //     match self.player_ab(player_id) {
    //         PlayerAB::PlayerB => match row_id {
    //             RowId::BackRow => front_half(player_range),
    //             RowId::FrontRow => end_half(player_range),
    //             RowId::Hero => panic!("hero is not part of row range"),
    //         },
    //         PlayerAB::PlayerA => match row_id {
    //             RowId::FrontRow => front_half(player_range),
    //             RowId::BackRow => end_half(player_range),
    //             RowId::Hero => panic!("hero is not part of row range"),
    //         },
    //     }
    // }

    /// An iterator over all slots on the entire board (even empty ones).
    // pub fn slots_iter(&self) -> impl Iterator<Item = &BoardSlot> {
    //     self.slots.iter()
    // }

    /// An iterator over all slots on the entire board (even empty ones).
    pub fn slots_iter_mut(&mut self) -> impl Iterator<Item = &mut BoardSlot> {
        self.slots.iter_mut()
    }

    /// An iterator over all the creatures on the board.
    // pub fn creatures_iter(&self) -> impl Iterator<Item = &UnitCardInstance> {
    //     self.slots_iter().creatures()
    // }

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

    pub fn hero(&self, player_id: PlayerId) -> &UnitCardInstance {
        BoardView::hero(self, player_id)
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
    // pub fn slot_with_creature(&self, id: UnitCardInstanceId) -> &BoardSlot {
    //     self.slots_iter()
    //         .filter(|s| s.maybe_creature().map(|c| c.id()) == Some(id))
    //         .next()
    //         .expect(&format!("Creature instance with id {:?} not found.", id))
    // }

    /// A slice starting at the slot where the creature instance exists,
    /// and including all subsequent slots it occupies (if the creature has a Width of more than 1 slot).
    // pub fn slots_with_creature(&self, id: UnitCardInstanceId) -> &[BoardSlot] {
    //     let (start_index, slot) = self
    //         .slots
    //         .iter()
    //         .enumerate()
    //         .filter(|(_, slot)| slot.maybe_creature().map(|c| c.id()) == Some(id))
    //         .next()
    //         .expect(&format!("Creature instance with id {:?} not found.", id));

    //     let creature_width = slot.maybe_creature().unwrap().width();

    //     &self.slots[start_index..start_index + creature_width]
    // }

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

    // pub fn slot_at_pos(&self, pos: BoardPos) -> &BoardSlot {
    //     self.slots_iter()
    //         .filter(|s| s.pos() == pos)
    //         .next()
    //         .expect("The position was not a valid board slot.")
    // }

    // pub fn pos_with_creature(&self, id: UnitCardInstanceId) -> BoardPos {
    //     self.slot_with_creature(id).pos()
    // }

    // pub fn creature_instance(&self, id: UnitCardInstanceId) -> &UnitCardInstance {
    //     self.slot_with_creature(id).maybe_creature().unwrap()
    // }

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

pub mod player_view {
    use super::*;
    use crate::game_state::{card_instance::UnitCardInstancePlayerView, MakePlayerView};

    #[derive(Debug, Serialize, Clone, Deserialize)]
    pub struct BoardSlotPlayerView {
        pos: BoardPos,
        creature: Option<UnitCardInstancePlayerView>,
    }

    impl BoardSlotPlayerView {
        pub fn pos(&self) -> BoardPos {
            self.pos
        }

        pub fn maybe_creature(&self) -> Option<&UnitCardInstancePlayerView> {
            self.creature.as_ref()
        }
    }

    impl<'a> MakePlayerView<'a> for BoardSlot {
        type TOut = BoardSlotPlayerView;

        fn player_view(&'a self, player_viewing: PlayerId) -> BoardSlotPlayerView {
            BoardSlotPlayerView {
                pos: self.pos,
                creature: self
                    .creature
                    .as_ref()
                    .map(|c| c.player_view(player_viewing)),
            }
        }
    }

    #[derive(Debug, Serialize, Clone, Deserialize)]
    pub struct BoardPlayerView {
        player_a_id: PlayerId,
        player_b_id: PlayerId,
        slots: Vec<BoardSlotPlayerView>,
    }

    impl<'a> MakePlayerView<'a> for Board {
        type TOut = BoardPlayerView;

        fn player_view(&'a self, player_viewing: PlayerId) -> BoardPlayerView {
            BoardPlayerView {
                player_a_id: self.player_a_id,
                player_b_id: self.player_b_id,
                slots: self
                    .slots
                    .iter()
                    .map(|s| s.player_view(player_viewing))
                    .collect(),
            }
        }
    }

    impl<'a> BoardSlotView<'a> for BoardSlotPlayerView {
        type CardInstanceView = UnitCardInstancePlayerView;

        fn pos(&self) -> BoardPos {
            BoardSlotPlayerView::pos(self)
        }

        fn maybe_creature(&self) -> Option<&Self::CardInstanceView> {
            BoardSlotPlayerView::maybe_creature(self)
        }
    }

    impl<'a> BoardView<'a> for BoardPlayerView {
        type SlotView = BoardSlotPlayerView;

        fn player_a_id(&self) -> PlayerId {
            self.player_a_id
        }

        fn player_b_id(&self) -> PlayerId {
            self.player_b_id
        }

        fn slots(&self) -> &[Self::SlotView] {
            self.slots.as_slice()
        }
    }
}
