use std::borrow::Borrow;

use super::{
    creature_instance::CreatureInstance,
    game_state::{self, GameState},
};

pub struct Board<T>
where
    T: Borrow<GameState>,
{
    game_state: T,
}

impl<T> Board<T>
where
    T: Borrow<GameState>,
{
    pub fn new(game_state: T) -> Self {
        Self { game_state }
    }

    pub fn creature_at_pos(&self) {
        let game_state: &GameState = self.game_state.borrow();

        // game_state.entity_arena().of_type::<CreatureInstance>().find(|c|);
    }
}
