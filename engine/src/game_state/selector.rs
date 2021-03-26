use super::{
    board::{BoardSlotView, BoardView, RowId},
    card_instance::UnitCardInstanceView,
    game_state::GameStateView,
    PlayerId, UnitCardInstanceId,
};

// pub trait AsSelector<'a, TState>
// where
//     TState: GameStateView<'a>,
// {
//     fn selector(&'a self) -> BoardSelector<'a, TState>;
// }

// impl<'a, T> From<&'a T> for BoardSelector<'a, T>
// where
//     T: GameStateView<'a>,
// {
//     fn from(game_state: &'a T) -> Self {
//         BoardSelector::new(game_state)
//     }
// }

// impl<'a, T> AsSelector<'a, T> for T
// where
//     T: GameStateView<'a>,
// {
//     fn selector(&'a self) -> BoardSelector<'a, T> {
//         self.into()
//     }
// }

pub mod iter_helpers {
    use super::*;

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

    pub trait IterAddons<'a, I, S: 'a>
    where
        I: Iterator<Item = &'a S>,
        S: BoardSlotView<'a>,
    {
        fn for_player(self, player_id: PlayerId) -> PlayerFilter<'a, I, S>;
        fn with_creature(self) -> SlotCreatureFilter<'a, I, S>;
        fn creatures(self) -> SlotCreatureMap<'a, I, S>;
        fn creature_ids(self) -> SlotCreatureIdMap<'a, I, S>;
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

    pub trait IteratorAny {
        fn has_any(self) -> bool;
    }

    impl<T: IntoIterator> IteratorAny for T {
        fn has_any(self) -> bool {
            self.into_iter().next().is_some()
        }
    }
}

// pub struct BoardSelector<'a, TState>
// where
//     TState: GameStateView<'a>,
// {
//     game_state: &'a TState,
//     player_id: Option<PlayerId>,
//     with_creature: bool,
//     include_heroes: bool,
// }

// impl<'a, TState> BoardSelector<'a, TState>
// where
//     TState: GameStateView<'a>,
// {
//     pub fn new(game_state: &'a TState) -> Self {
//         Self {
//             game_state,
//             player_id: None,
//             with_creature: false,
//             include_heroes: false,
//         }
//     }

//     pub fn for_player(mut self, player_id: PlayerId) -> Self {
//         assert!(self.player_id.is_none());
//         self.player_id = Some(player_id);
//         self
//     }

//     pub fn with_creature(mut self) -> Self {
//         self.with_creature = true;
//         self
//     }

//     pub fn include_heroes(mut self) -> Self {
//         self.include_heroes = true;
//         self
//     }

//     pub fn my_slots(self) -> bool {
//         self.game_state
//             .board()
//             .slots_iter()
//             .for_player(PlayerId::new())
//             .has_any()
//     }

//     pub fn slots(
//         self,
//     ) -> impl Iterator<Item = &'a <<TState as GameStateView<'a>>::BoardView as BoardView<'a>>::SlotView>
//     {
//         let player_id_filter = self.player_id.clone();
//         let with_creature = self.with_creature;
//         let include_heroes = self.include_heroes;

//         let iter = self
//             .game_state
//             .board()
//             .slots_iter()
//             .filter(move |s| match player_id_filter {
//                 Some(player_id) => s.pos().player_id == player_id,
//                 _ => true,
//             })
//             .filter(move |s| match with_creature {
//                 true => s.maybe_creature().is_some(),
//                 _ => true,
//             })
//             .filter(move |s| match s.pos().row() {
//                 RowId::Hero => include_heroes,
//                 _ => true,
//             });

//         iter
//     }

//     pub fn creatures(self) -> impl Iterator<Item = &'a <<<TState as GameStateView<'a>>::BoardView as BoardView<'a>>::SlotView as BoardSlotView<'a>>::CardInstanceView>{
//         let include_heroes = self.include_heroes;

//         self.slots()
//             .filter(move |s| match s.pos().row() {
//                 RowId::Hero => include_heroes,
//                 _ => true,
//             })
//             .filter_map(|s| s.maybe_creature())
//     }

//     pub fn creature_ids(self) -> Vec<UnitCardInstanceId> {
//         self.creatures().map(|c| c.id()).collect()
//     }
// }
