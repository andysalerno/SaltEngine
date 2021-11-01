use std::iter::Filter;

use futures::future::Select;

use super::{
    board::{Board, BoardSlot, BoardSlotView, BoardView, RowId},
    card_instance::UnitCardInstanceView,
    PlayerId, UnitCardInstance, UnitCardInstanceId,
};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Selection {
    MustInclude,
    MustExclude,
    IncludeOrExclude,
}

struct Selectorz<'a, TBoard>
where
    TBoard: 'a,
{
    _phantom: std::marker::PhantomData<&'a TBoard>,
    board: TBoard,
    heroes: Selection,
    creatures: Selection,
    player_id: Option<PlayerId>,
}

impl<'a, TBoard> Selectorz<'a, TBoard>
where
    TBoard: AsRef<&'a Board>,
{
    pub fn new(board: TBoard) -> Self {
        Self {
            _phantom: std::marker::PhantomData::default(),
            board,
            heroes: Selection::IncludeOrExclude,
            creatures: Selection::IncludeOrExclude,
            player_id: None,
        }
    }
}

impl<'a, TBoard> Selectorz<'a, TBoard>
where
    TBoard: AsRef<&'a Board> + 'a,
{
    pub fn iter(self) -> impl Iterator<Item = &'a BoardSlot> + 'a {
        self.board
            .as_ref()
            .slots()
            .iter()
            .filter(move |s| match_slot(s, self.heroes, self.creatures, self.player_id))
    }
}

impl<'a> Selectorz<'a, &'a mut Board> {
    pub fn iter_mut(self) -> impl Iterator<Item = &'a mut BoardSlot> + 'a {
        let board = self.board;
        let heroes = self.heroes;
        let creatures = self.creatures;
        let player_id = self.player_id;

        board
            .slots
            .iter_mut()
            .filter(move |s| match_slot(s, heroes, creatures, player_id))
    }
}

fn match_slot(
    slot: &BoardSlot,
    heroes: Selection,
    creatures: Selection,
    player_id: Option<PlayerId>,
) -> bool {
    if heroes == Selection::MustExclude && slot.pos().row().is_hero() {
        return false;
    }

    if heroes == Selection::MustInclude && !slot.pos().row().is_hero() {
        return false;
    }

    if creatures == Selection::MustExclude && slot.has_creature() {
        return false;
    }

    if creatures == Selection::MustInclude && !slot.has_creature() {
        return false;
    }

    if let Some(player_id) = player_id {
        if slot.pos().player_id != player_id {
            return false;
        }
    }

    true
}

struct Selector<'a> {
    board: &'a Board,
    heroes: Selection,
    creatures: Selection,
    player_id: Option<PlayerId>,
}

impl<'a> Selector<'a> {
    pub fn new(board: &'a Board) -> Self {
        Self {
            board,
            heroes: Selection::IncludeOrExclude,
            creatures: Selection::IncludeOrExclude,
            player_id: None,
        }
    }

    pub fn creature_with_id(&self, id: UnitCardInstanceId) -> Option<&UnitCardInstance> {
        self.slot_with_id(id).and_then(BoardSlot::maybe_creature)
    }

    pub fn slot_with_id(&self, id: UnitCardInstanceId) -> Option<&BoardSlot> {
        self.board
            .slots()
            .iter()
            .find(|s| s.maybe_creature().map_or(false, |c| c.id() == id))
    }

    pub fn thing(&self, id: UnitCardInstanceId) -> Option<&BoardSlot> {
        self.board
            .slots()
            .iter()
            .filter(|s| s.has_creature())
            .find(|s| s.maybe_creature().map_or(false, |c| c.id() == id))
    }

    pub fn iter(self) -> impl Iterator<Item = &'a BoardSlot> {
        self.board
            .slots()
            .iter()
            .filter(move |s| self.match_slot(s))
    }

    fn match_slot(&self, slot: &BoardSlot) -> bool {
        if self.heroes == Selection::MustExclude && slot.pos().row().is_hero() {
            return false;
        }

        if self.heroes == Selection::MustInclude && !slot.pos().row().is_hero() {
            return false;
        }

        if self.creatures == Selection::MustExclude && slot.has_creature() {
            return false;
        }

        if self.creatures == Selection::MustInclude && !slot.has_creature() {
            return false;
        }

        if let Some(player_id) = self.player_id {
            if slot.pos().player_id != player_id {
                return false;
            }
        }

        true
    }
}

pub mod iter_helpers {
    use super::{BoardSlotView, PlayerId, RowId, UnitCardInstanceId, UnitCardInstanceView};

    /// A trait containing helpful methods when iterating over board slots.
    pub trait IterAddons<'a, I, S: 'a>
    where
        I: Iterator<Item = &'a S>,
        S: BoardSlotView<'a>,
    {
        fn for_player(self, player_id: PlayerId) -> PlayerFilter<'a, I, S>;

        /// A filter that only selects slots with a creature present.
        fn with_creature(self) -> SlotCreatureFilter<'a, I, S>;

        /// A filter that only selects slots with a creature present, and then maps to that creature instance.
        fn creatures(self) -> SlotCreatureMap<'a, I, S>;
        fn creature_ids(self) -> SlotCreatureIdMap<'a, I, S>;

        /// A filter that excludes hero slots from the iteration.
        fn exclude_heroes(self) -> ExcludeHeroesFilter<'a, I, S>;

        fn heroes_only(self) -> HeroesOnlyFilter<'a, I, S>;
    }

    impl<'a, I, S: 'a> IterAddons<'a, I, S> for I
    where
        I: Iterator<Item = &'a S>,
        S: BoardSlotView<'a>,
    {
        fn for_player(self, player_id: PlayerId) -> PlayerFilter<'a, I, S> {
            PlayerFilter {
                iter: self,
                player_id,
            }
        }

        fn with_creature(self) -> SlotCreatureFilter<'a, I, S> {
            SlotCreatureFilter { iter: self }
        }

        fn creatures(self) -> SlotCreatureMap<'a, I, S> {
            SlotCreatureMap { iter: self }
        }

        fn creature_ids(self) -> SlotCreatureIdMap<'a, I, S> {
            SlotCreatureIdMap { iter: self }
        }

        fn exclude_heroes(self) -> ExcludeHeroesFilter<'a, I, S> {
            ExcludeHeroesFilter { iter: self }
        }

        fn heroes_only(self) -> HeroesOnlyFilter<'a, I, S> {
            HeroesOnlyFilter { iter: self }
        }
    }

    pub struct SlotCreatureIdMap<'a, I, S: 'a>
    where
        I: Iterator<Item = &'a S>,
        S: BoardSlotView<'a>,
    {
        iter: I,
    }

    impl<'a, I, S: 'a> Iterator for SlotCreatureIdMap<'a, I, S>
    where
        I: Iterator<Item = &'a S>,
        S: BoardSlotView<'a>,
    {
        type Item = UnitCardInstanceId;

        fn next(&mut self) -> Option<Self::Item> {
            self.iter.find_map(|s| s.maybe_creature()).map(|c| c.id())
        }
    }

    pub struct SlotCreatureMap<'a, I, S: 'a>
    where
        I: Iterator<Item = &'a S>,
        S: BoardSlotView<'a>,
    {
        iter: I,
    }

    impl<'a, I, S: 'a> Iterator for SlotCreatureMap<'a, I, S>
    where
        I: Iterator<Item = &'a S>,
        S: BoardSlotView<'a>,
    {
        type Item = &'a <S as BoardSlotView<'a>>::CardInstanceView;

        fn next(&mut self) -> Option<Self::Item> {
            self.iter.find_map(|s| s.maybe_creature())
        }
    }

    pub struct SlotCreatureFilter<'a, I, S: 'a>
    where
        I: Iterator<Item = &'a S>,
        S: BoardSlotView<'a>,
    {
        iter: I,
    }

    impl<'a, I, S: 'a> Iterator for SlotCreatureFilter<'a, I, S>
    where
        I: Iterator<Item = &'a S>,
        S: BoardSlotView<'a>,
    {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            self.iter.find(|s| s.maybe_creature().is_some())
        }
    }

    pub struct PlayerFilter<'a, I, S: 'a>
    where
        I: Iterator<Item = &'a S>,
        S: BoardSlotView<'a>,
    {
        iter: I,
        player_id: PlayerId,
    }

    impl<'a, I, S: 'a> Iterator for PlayerFilter<'a, I, S>
    where
        I: Iterator<Item = &'a S>,
        S: BoardSlotView<'a>,
    {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            let id = self.player_id;
            self.iter.find(|s| s.pos().player_id == id)
        }
    }

    pub struct ExcludeHeroesFilter<'a, I, S: 'a>
    where
        I: Iterator<Item = &'a S>,
        S: BoardSlotView<'a>,
    {
        iter: I,
    }

    impl<'a, I, S: 'a> Iterator for ExcludeHeroesFilter<'a, I, S>
    where
        I: Iterator<Item = &'a S>,
        S: BoardSlotView<'a>,
    {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            self.iter.find(|s| s.pos().row() != RowId::Hero)
        }
    }

    pub struct HeroesOnlyFilter<'a, I, S: 'a>
    where
        I: Iterator<Item = &'a S>,
        S: BoardSlotView<'a>,
    {
        iter: I,
    }

    impl<'a, I, S: 'a> Iterator for HeroesOnlyFilter<'a, I, S>
    where
        I: Iterator<Item = &'a S>,
        S: BoardSlotView<'a>,
    {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            self.iter.find(|s| s.pos().row() == RowId::Hero)
        }
    }

    pub trait IteratorAny {
        fn has_any(self) -> bool;
    }

    impl<T: IntoIterator> IteratorAny for T {
        fn has_any(self) -> bool {
            self.into_iter().next().is_some()
        }
    }

    pub trait IteratorExpectSingle<TItem> {
        fn expect_single(self) -> TItem;
    }

    impl<T, TItem> IteratorExpectSingle<TItem> for T
    where
        T: IntoIterator<Item = TItem>,
    {
        fn expect_single(self) -> TItem {
            let mut iter = self.into_iter();

            let item = iter
                .next()
                .expect("Expected a single item, but there were none.");

            if iter.next().is_some() {
                panic!("Expected a single item, but there were multiple.");
            }

            item
        }
    }
}
