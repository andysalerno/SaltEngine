use super::{
    board::{BoardSlot, RowId},
    GameState, PlayerId, UnitCardInstance, UnitCardInstanceId,
};

pub trait AsSelector<'a> {
    fn selector(&'a self) -> BoardSelector<'a>;
}

impl<'a> AsSelector<'a> for GameState {
    fn selector(&'a self) -> BoardSelector<'a> {
        self.into()
    }
}

impl<'a> From<&'a GameState> for BoardSelector<'a> {
    fn from(game_state: &'a GameState) -> Self {
        BoardSelector::new(game_state)
    }
}

pub struct BoardSelector<'a> {
    game_state: &'a GameState,
    player_id: Option<PlayerId>,
    with_creature: bool,
    include_heroes: bool,
}

impl<'a> BoardSelector<'a> {
    pub fn new(game_state: &'a GameState) -> Self {
        Self {
            game_state,
            player_id: None,
            with_creature: false,
            include_heroes: false,
        }
    }

    pub fn for_player(mut self, player_id: PlayerId) -> Self {
        assert!(self.player_id.is_none());
        self.player_id = Some(player_id);
        self
    }

    pub fn with_creature(mut self) -> Self {
        self.with_creature = true;
        self
    }

    pub fn include_heroes(mut self) -> Self {
        self.include_heroes = true;
        self
    }

    pub fn slots(self) -> impl Iterator<Item = &'a BoardSlot> {
        let player_id_filter = self.player_id.clone();
        let with_creature = self.with_creature;
        let include_heroes = self.include_heroes;

        let iter = self
            .game_state
            .board()
            .slots_iter()
            .filter(move |s| match player_id_filter {
                Some(player_id) => s.pos().player_id == player_id,
                _ => true,
            })
            .filter(move |s| match with_creature {
                true => s.maybe_creature().is_some(),
                _ => true,
            })
            .filter(move |s| match s.pos().row() {
                RowId::Hero => include_heroes,
                _ => true,
            });

        iter
    }

    pub fn creatures(self) -> impl Iterator<Item = &'a UnitCardInstance> {
        let include_heroes = self.include_heroes;

        self.slots()
            .filter(move |s| match s.pos().row() {
                RowId::Hero => include_heroes,
                _ => true,
            })
            .filter_map(|s| s.maybe_creature())
    }

    pub fn creature_ids(self) -> Vec<UnitCardInstanceId> {
        self.creatures().map(|c| c.id()).collect()
    }
}
