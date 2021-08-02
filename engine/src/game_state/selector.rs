use super::{
    board::{BoardSlotView, RowId},
    card_instance::UnitCardInstanceView,
    PlayerId, UnitCardInstanceId,
};

pub mod iter_helpers {
    use super::*;

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
    }

    impl<'a, I, S: 'a> IterAddons<'a, I, S> for I
    where
        I: Iterator<Item = &'a S>,
        S: BoardSlotView<'a>,
    {
        fn for_player(self, player_id: PlayerId) -> PlayerFilter<'a, I, S> {
            PlayerFilter {
                iter: self,
                player_id: player_id,
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

    pub trait IteratorAny {
        fn has_any(self) -> bool;
    }

    impl<T: IntoIterator> IteratorAny for T {
        fn has_any(self) -> bool {
            self.into_iter().next().is_some()
        }
    }
}
