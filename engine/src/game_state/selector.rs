use super::{
    board::{Board, BoardSlot, BoardSlotView, BoardView, RowId},
    card_instance::UnitCardInstanceView,
    PlayerId, UnitCardInstanceId,
};
use std::borrow::{Borrow, BorrowMut};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Selection {
    MustInclude,
    MustExclude,
    IncludeOrExclude,
}

pub struct Selector<'a, TBoard>
where
    TBoard: 'a,
{
    _phantom: std::marker::PhantomData<&'a TBoard>,
    board: TBoard,
    heroes: Selection,
    creatures: Selection,
    player_id: Option<PlayerId>,
}

impl<'a, TBoard> Selector<'a, &'a TBoard>
where
    TBoard: BoardView<'a> + 'a,
{
    pub fn new(board: &'a TBoard) -> Self {
        Self {
            _phantom: std::marker::PhantomData::default(),
            board,
            heroes: Selection::IncludeOrExclude,
            creatures: Selection::IncludeOrExclude,
            player_id: None,
        }
    }

    pub fn iter(&'a self) -> impl Iterator<Item = &<TBoard as BoardView<'a>>::SlotView> + 'a {
        let heroes = self.heroes;
        let creatures = self.creatures;
        let player_id = self.player_id;

        self.board
            .slots()
            .iter()
            .filter(move |&s| match_slot_view(s, heroes, creatures, player_id))
    }
}

impl<'a> Selector<'a, &'a mut Board> {
    pub fn new_mut(board: &'a mut Board) -> Self {
        Self {
            _phantom: std::marker::PhantomData::default(),
            board,
            heroes: Selection::IncludeOrExclude,
            creatures: Selection::IncludeOrExclude,
            player_id: None,
        }
    }

    pub fn iter_mut(&'a mut self) -> impl Iterator<Item = &'a mut BoardSlot> + 'a {
        let heroes = self.heroes;
        let creatures = self.creatures;
        let player_id = self.player_id;

        self.board
            .slots
            .iter_mut()
            .filter(move |s| match_slot_view(s as &BoardSlot, heroes, creatures, player_id))
    }
}

impl<'a, TBoard> Selector<'a, TBoard>
where
    TBoard: BoardView<'a>,
{
    pub fn from_boardview(board: TBoard) -> Self {
        Self {
            _phantom: std::marker::PhantomData::default(),
            board,
            heroes: Selection::IncludeOrExclude,
            creatures: Selection::IncludeOrExclude,
            player_id: None,
        }
    }
}

fn match_slot_view<'a, TSlotView: BoardSlotView<'a>>(
    slot: &'a TSlotView,
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

#[cfg(test)]
mod test {
    use super::Selector;
    use crate::game_state::{board::Board, PlayerId};

    // #[test]
    // fn selector_consumes_board() {
    //     let board = Board::new(8, PlayerId::new(), PlayerId::new());
    //     let selector = Selector::new(board);
    //     let _vec = selector.iter().collect::<Vec<_>>();
    // }

    #[test]
    fn selector_consumes_board_ref() {
        let board = Board::new(8, PlayerId::new(), PlayerId::new());
        let selector = Selector::new(&board);
        let _vec = selector.iter().collect::<Vec<_>>();
    }

    // #[test]
    // fn selector_consumes_board_mut_ref() {
    //     let mut board = Board::new(8, PlayerId::new(), PlayerId::new());
    //     let selector = Selector::new_mut(&mut board);
    //     let _vec = selector.iter().collect::<Vec<_>>();
    // }

    #[test]
    fn selector_consumes_board_mut_ref_iter_mut() {
        let mut board = Board::new(8, PlayerId::new(), PlayerId::new());
        let mut selector = Selector::new_mut(&mut board);
        let _vec = selector.iter_mut().collect::<Vec<_>>();
    }

    // #[test]
    // fn selector_consumes_board_iter_mut() {
    //     let board = Board::new(8, PlayerId::new(), PlayerId::new());
    //     let mut selector = Selector::new(board);
    //     let _vec = selector.iter_mut().collect::<Vec<_>>();
    // }
}
