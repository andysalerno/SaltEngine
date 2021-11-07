use self::player_view::BoardSlotPlayerView;

use super::{
    card_instance::{UnitCardInstance, UnitCardInstanceView},
    hero::HeroDefinition,
    selector::{
        iter_helpers::{SlotCreatureFilter, SlotCreatureMap},
        Selector,
    },
    IterAddons, PlayerId, UnitCardInstanceId,
};
use crate::cards::UnitCardDefinition;
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, collections::HashMap, fmt::Debug};

const BOARD_WIDTH: usize = 6;
const SLOTS_COUNT: usize = BOARD_WIDTH * 4;

/// A view of a particular board pos.
pub trait BoardSlotView<'a> {
    type CardInstanceView: UnitCardInstanceView<'a>;

    /// The `BoardPos` this view represents.
    fn pos(&self) -> BoardPos;

    /// The view of the creature on this pos, if there is one.
    fn maybe_creature(&self) -> Option<&Self::CardInstanceView>;

    #[must_use]
    fn has_creature(&self) -> bool {
        self.maybe_creature().is_some()
    }
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

    #[must_use]
    pub fn has_creature(&self) -> bool {
        self.maybe_creature().is_some()
    }

    pub fn take_creature(&mut self) -> UnitCardInstance {
        self.creature.take().unwrap()
    }

    #[must_use]
    pub fn maybe_creature(&self) -> Option<&UnitCardInstance> {
        self.creature.as_ref()
    }

    pub fn maybe_creature_mut(&mut self) -> Option<&mut UnitCardInstance> {
        self.creature.as_mut()
    }

    pub fn set_creature(&mut self, creature: UnitCardInstance) {
        self.creature = Some(creature);
    }

    #[must_use]
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
    #[must_use]
    pub fn is_back(&self) -> bool {
        matches!(self, RowId::BackRow)
    }

    #[must_use]
    pub fn is_front(&self) -> bool {
        matches!(self, RowId::FrontRow)
    }

    #[must_use]
    pub fn is_hero(&self) -> bool {
        matches!(self, RowId::Hero)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoardPos {
    pub player_id: PlayerId,
    pub row_id: RowId,
    pub row_index: usize,
}

impl BoardPos {
    #[must_use]
    pub fn new(player_id: PlayerId, row_id: RowId, row_index: usize) -> Self {
        Self {
            player_id,
            row_id,
            row_index,
        }
    }

    #[must_use]
    pub fn hero_pos(player_id: PlayerId) -> Self {
        Self {
            player_id,
            row_id: RowId::Hero,
            row_index: 0,
        }
    }

    #[must_use]
    pub fn row(&self) -> RowId {
        self.row_id
    }
}

enum PlayerAB {
    PlayerA,
    PlayerB,
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

/// A trait representing a view on a `Board`.
pub trait BoardView<'a> {
    type SlotView: BoardSlotView<'a>;

    fn player_a_id(&self) -> PlayerId;
    fn player_b_id(&self) -> PlayerId;

    fn player_a_hero(&'a self) -> &'a Self::SlotView;
    fn player_b_hero(&'a self) -> &'a Self::SlotView;

    fn slots(&self) -> &[Self::SlotView];

    fn all_characters_slots(
        &'a self,
    ) -> std::iter::Chain<
        std::slice::Iter<'_, <Self as BoardView<'a>>::SlotView>,
        std::array::IntoIter<&<Self as BoardView<'a>>::SlotView, 2_usize>,
    > {
        let heroes = [self.player_a_hero(), self.player_b_hero()];
        self.slots().iter().chain(heroes)
    }

    fn slots_iter(&self) -> std::slice::Iter<'_, <Self as BoardView<'a>>::SlotView> {
        self.slots().iter()
    }

    /// True if there are `n_slots` starting at 'pos' within one row.
    /// Does not consider whether the slots are occupied or not.
    fn is_range_in_row(&self, pos: BoardPos, n_slots: usize) -> bool {
        let row = self.player_row(pos.player_id, pos.row());
        let space_in_row = row.len() - pos.row_index;
        space_in_row >= n_slots
    }

    fn player_row(&self, player_id: PlayerId, row: RowId) -> &[<Self as BoardView<'a>>::SlotView] {
        &self.slots()[row_range(self, player_id, row)]
    }

    fn hero(
        &'a self,
        player_id: PlayerId,
    ) -> &<<Self as BoardView<'a>>::SlotView as BoardSlotView<'a>>::CardInstanceView {
        match player_ab(self, player_id) {
            PlayerAB::PlayerA => self
                .player_a_hero()
                .maybe_creature()
                .expect("Expected a hero in the hero slot."),
            PlayerAB::PlayerB => self
                .player_b_hero()
                .maybe_creature()
                .expect("Expected a hero in the hero slot"),
        }
    }

    /// A slice starting at the slot where the creature instance exists,
    /// and including all subsequent slots it occupies (if the creature has a Width of more than 1 slot).
    fn slots_with_creature(&self, id: UnitCardInstanceId) -> &[<Self as BoardView<'a>>::SlotView] {
        let (start_index, slot) = self
            .slots_iter()
            .enumerate()
            .find(|(_, slot)| slot.maybe_creature().map(|c| c.id()) == Some(id))
            .unwrap_or_else(|| panic!("Creature instance with id {:?} not found.", id));

        let creature_width = slot.maybe_creature().unwrap().width();

        &self.slots()[start_index..start_index + creature_width]
    }

    fn slot_with_creature(&'a self, id: UnitCardInstanceId) -> &<Self as BoardView<'a>>::SlotView {
        if self.player_a_hero().maybe_creature().unwrap().id() == id {
            self.player_a_hero()
        } else if self.player_b_hero().maybe_creature().unwrap().id() == id {
            self.player_b_hero()
        } else {
            self.slots_iter()
                .find(|s| s.maybe_creature().map(|c| c.id()) == Some(id))
                .unwrap_or_else(|| panic!("Creature instance with id {:?} not found.", id))
        }
    }

    fn slot_at_pos(&self, pos: BoardPos) -> &<Self as BoardView<'a>>::SlotView {
        self.slots_iter()
            .find(|s| s.pos() == pos)
            .expect("The position was not a valid board slot.")
    }

    fn pos_with_creature(&'a self, id: UnitCardInstanceId) -> BoardPos {
        self.slot_with_creature(id).pos()
    }

    fn creature_instance(
        &'a self,
        id: UnitCardInstanceId,
    ) -> &<<Self as BoardView<'a>>::SlotView as BoardSlotView<'a>>::CardInstanceView {
        self.slot_with_creature(id).maybe_creature().unwrap()
    }

    fn creature_at_pos(
        &'a self,
        pos: BoardPos,
    ) -> Option<&<<Self as BoardView<'a>>::SlotView as BoardSlotView<'a>>::CardInstanceView> {
        if pos.row() == RowId::Hero {
            return match player_ab(self, pos.player_id) {
                PlayerAB::PlayerA => self.player_a_hero().maybe_creature(),
                PlayerAB::PlayerB => self.player_b_hero().maybe_creature(),
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

        None
    }

    #[must_use]
    fn selector(&'a self) -> Selector<&Self>
    where
        Self: Sized,
    {
        let () = Selector::new(self.borrow());
        todo!()
    }
}

fn row_range<'a>(
    board_view: &(impl BoardView<'a> + ?Sized),
    player_id: PlayerId,
    row_id: RowId,
) -> std::ops::Range<usize> {
    // Hero row is a special case
    if row_id == RowId::Hero {
        panic!("RowId 'Hero' not valid for row_range() operation.");
    }

    let player_range = player_range(board_view, player_id);

    match player_ab(board_view, player_id) {
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

fn player_range<'a>(
    board_view: &(impl BoardView<'a> + ?Sized),
    player_id: PlayerId,
) -> std::ops::Range<usize> {
    match player_ab(board_view, player_id) {
        PlayerAB::PlayerB => front_half(0..SLOTS_COUNT),
        PlayerAB::PlayerA => end_half(0..SLOTS_COUNT),
    }
}

// impl<'a> BoardView<'a> for Board {
//     type SlotView = BoardSlot;

//     fn player_a_id(&self) -> PlayerId {
//         self.player_a_id
//     }

//     fn player_b_id(&self) -> PlayerId {
//         self.player_b_id
//     }

//     fn slots(&self) -> &[BoardSlot] {
//         self.slots.as_slice()
//     }

//     fn player_a_hero(&self) -> &Self::SlotView {
//         &self.player_a_hero_slot
//     }

//     fn player_b_hero(&self) -> &Self::SlotView {
//         &self.player_b_hero_slot
//     }
// }

// impl<'a> BoardView<'a> for &'a mut Board {
//     type SlotView = BoardSlot;

//     fn player_a_id(&self) -> PlayerId {
//         todo!()
//     }

//     fn player_b_id(&self) -> PlayerId {
//         todo!()
//     }

//     fn player_a_hero(&'a self) -> &'a Self::SlotView {
//         todo!()
//     }

//     fn player_b_hero(&'a self) -> &'a Self::SlotView {
//         todo!()
//     }

//     fn slots(&self) -> &[Self::SlotView] {
//         todo!()
//     }
// }

impl<'a, T> BoardView<'a> for T
where
    T: Borrow<Board>,
{
    type SlotView = BoardSlot;

    fn player_a_id(&self) -> PlayerId {
        self.borrow().player_a_id
    }

    fn player_b_id(&self) -> PlayerId {
        self.borrow().player_b_id
    }

    fn slots(&self) -> &[BoardSlot] {
        self.borrow().slots.as_slice()
    }

    fn player_a_hero(&self) -> &Self::SlotView {
        &self.borrow().player_a_hero_slot
    }

    fn player_b_hero(&self) -> &Self::SlotView {
        &self.borrow().player_b_hero_slot
    }
}

#[derive(Debug)]
pub struct Board {
    arena: HashMap<UnitCardInstanceId, UnitCardInstance>,
    player_a_id: PlayerId,
    player_a_hero_slot: BoardSlot,
    player_b_hero_slot: BoardSlot,
    player_b_id: PlayerId,
    pub(crate) slots: Vec<BoardSlot>,

    /// Cards that are not on a `BoardSlot`, but are known to the board and accessible from `Board` functions.
    /// Example: a card is summoned from a player's hand: it is removed from the hand, but while the "on summoned"
    /// event resolves, it is not yet in a board slot.
    /// So ownership of the card is tracked here.
    tracked_pending_cards: Vec<UnitCardInstance>,
    graveyard: Vec<UnitCardInstance>,
}

impl Board {
    fn player_ab(&self, player_id: PlayerId) -> PlayerAB {
        if player_id == self.player_a_id() {
            PlayerAB::PlayerA
        } else if player_id == self.player_b_id() {
            PlayerAB::PlayerB
        } else {
            panic!("Unknown player id: {:?}", player_id)
        }
    }

    #[must_use]
    pub fn new(_size: usize, player_a_id: PlayerId, player_b_id: PlayerId) -> Self {
        let mut slots = Vec::with_capacity(SLOTS_COUNT);

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

        let mut player_a_hero_slot = BoardSlot::new(BoardPos::hero_pos(player_a_id));
        player_a_hero_slot.set_creature(HeroDefinition.make_instance());

        let mut player_b_hero_slot = BoardSlot::new(BoardPos::hero_pos(player_b_id));
        player_b_hero_slot.set_creature(HeroDefinition.make_instance());

        Self {
            player_a_id,
            player_a_hero_slot,
            player_b_hero_slot,
            player_b_id,
            slots,
            tracked_pending_cards: Vec::new(),
            graveyard: Vec::new(),
            arena: HashMap::new(),
        }
    }

    pub fn track_pending_card(&mut self, card: UnitCardInstance) {
        self.tracked_pending_cards.push(card);
    }

    #[must_use]
    pub fn selector_mut<TRef>(&mut self) -> Selector<&mut Board> {
        Selector::new(self)
    }

    pub fn tracked_pending_cards(&self) -> impl Iterator<Item = &UnitCardInstance> {
        self.tracked_pending_cards.iter()
    }

    pub fn take_tracked_pending_card(
        &mut self,
        id: UnitCardInstanceId,
    ) -> Option<UnitCardInstance> {
        self.tracked_pending_cards
            .iter()
            .position(|c| c.id() == id)
            .map(|a| self.tracked_pending_cards.remove(a))
    }

    /// Adds the given dead card instance to the graveyard.
    pub fn add_to_graveyard(&mut self, dead_card: UnitCardInstance) {
        self.graveyard.push(dead_card);
    }

    /// An iterator over all slots on the entire board (even empty ones).
    pub fn slots_iter_mut(&mut self) -> impl Iterator<Item = &mut BoardSlot> {
        let heros = [&mut self.player_a_hero_slot, &mut self.player_b_hero_slot];
        self.slots.iter_mut().chain(heros)
    }

    pub fn im(&mut self) -> impl Iterator<Item = &mut BoardSlot> {
        self.slots.iter_mut()
    }

    /// An iterator over all the creatures on the board.
    pub fn creatures_iter_mut(&mut self) -> impl Iterator<Item = &mut UnitCardInstance> {
        self.slots_iter_mut()
            .filter_map(BoardSlot::maybe_creature_mut)
    }

    #[must_use]
    pub fn player_side(&self, player_id: PlayerId) -> &[BoardSlot] {
        &self.slots[player_range(self, player_id)]
    }

    pub fn player_row_mut(&mut self, player_id: PlayerId, row: RowId) -> &mut [BoardSlot] {
        let range = row_range(self, player_id, row);
        &mut self.slots[range]
    }

    pub fn take_creature_by_id(&mut self, id: UnitCardInstanceId) -> UnitCardInstance {
        self.slot_with_creature_mut(id).take_creature()
    }

    #[must_use]
    pub fn companion_slot(&self, pos: BoardPos) -> &BoardSlot {
        let mut companion_pos = pos;

        let companion_row = match pos.row() {
            RowId::BackRow => RowId::FrontRow,
            RowId::FrontRow => RowId::BackRow,
            RowId::Hero => panic!("Hero slot has no companion."),
        };

        companion_pos.row_id = companion_row;

        self.slot_at_pos(companion_pos)
    }

    #[must_use]
    pub fn companion_creature(&self, pos: BoardPos) -> Option<&UnitCardInstance> {
        self.companion_slot(pos).maybe_creature()
    }

    #[must_use]
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
            // let slot = self.slot_at_pos_mut(pos);
            // slot.set_creature(card_instance);
            let slot = match self.player_ab(pos.player_id) {
                PlayerAB::PlayerA => &mut self.player_a_hero_slot,
                PlayerAB::PlayerB => &mut self.player_b_hero_slot,
            };

            slot.set_creature(card_instance);
        } else {
            let row = self.player_row_mut(pos.player_id, pos.row_id);
            row[pos.row_index].set_creature(card_instance);
        }
    }

    pub fn slot_with_creature_mut(&mut self, id: UnitCardInstanceId) -> &mut BoardSlot {
        self.slots_iter_mut()
            .collect::<Vec<_>>()
            .into_iter()
            .find(|s| s.maybe_creature().map(UnitCardInstance::id) == Some(id))
            .unwrap_or_else(|| panic!("Creature instance with id {:?} not found.", id))
    }

    pub fn slot_at_pos_mut(&mut self, pos: BoardPos) -> &mut BoardSlot {
        self.slots_iter_mut()
            .find(|s| s.pos() == pos)
            .expect("The position was not a valid board slot.")
    }

    /// Returns a mutable reference to the `UnitCardInstance` on the board with the given ID.
    /// This includes searching the pre-summon section, unlike the other methods of this pattern.
    pub fn creature_instance_mut(&mut self, id: UnitCardInstanceId) -> &mut UnitCardInstance {
        let index = self.tracked_pending_cards.iter().position(|c| c.id() == id);
        if let Some(index) = index {
            return self.tracked_pending_cards.get_mut(index).unwrap();
        }

        return self
            .slot_with_creature_mut(id)
            .maybe_creature_mut()
            .unwrap();
    }

    /// Returns a mutable reference to the `UnitCardInstance` on the board with the given ID.
    /// This includes searching the pre-summon section, unlike the other methods of this pattern.
    #[must_use]
    pub fn creature_instance_all(&self, id: UnitCardInstanceId) -> &UnitCardInstance {
        let index = self.tracked_pending_cards.iter().position(|c| c.id() == id);
        if let Some(index) = index {
            return self.tracked_pending_cards.get(index).unwrap();
        }

        return self.slot_with_creature(id).maybe_creature().unwrap();
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
    use super::{
        Board, BoardPos, BoardSlot, BoardSlotView, BoardView, Deserialize, PlayerId, Serialize,
    };
    use crate::game_state::{card_instance::UnitCardInstancePlayerView, MakePlayerView};

    #[derive(Debug, Serialize, Clone, Deserialize)]
    pub struct BoardSlotPlayerView {
        pos: BoardPos,
        creature: Option<UnitCardInstancePlayerView>,
    }

    impl BoardSlotPlayerView {
        #[must_use]
        pub fn pos(&self) -> BoardPos {
            self.pos
        }

        #[must_use]
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
        player_a_hero: BoardSlotPlayerView,
        player_b_hero: BoardSlotPlayerView,
        slots: Vec<BoardSlotPlayerView>,
    }

    impl<'a> MakePlayerView<'a> for Board {
        type TOut = BoardPlayerView;

        fn player_view(&'a self, player_viewing: PlayerId) -> BoardPlayerView {
            BoardPlayerView {
                player_a_id: self.player_a_id,
                player_b_id: self.player_b_id,
                player_a_hero: self.player_a_hero().player_view(player_viewing),
                player_b_hero: self.player_b_hero().player_view(player_viewing),
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

        fn player_a_hero(&self) -> &Self::SlotView {
            &self.player_a_hero
        }

        fn player_b_hero(&self) -> &Self::SlotView {
            &self.player_b_hero
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Board, BoardView};
    use crate::game_state::PlayerId;

    #[test]
    fn board_selector_iter() {
        let board = Board::new(8, PlayerId::new(), PlayerId::new());

        for _slot in board.selector().iter() {}
    }

    #[test]
    fn board_selector_iter_mut() {
        let mut board = Board::new(8, PlayerId::new(), PlayerId::new());

        for _slot in board.selector_mut().iter_mut() {}
    }
}
