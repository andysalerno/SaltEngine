use std::borrow::Borrow;

use entity_arena::{id::EntityId, Entity, TypedEntity, Value};

use super::{
    creature_instance::CreatureInstance,
    game_state::{GameState, Position},
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

    pub fn creature_at_pos(
        &self,
        position: impl Borrow<Position>,
    ) -> Option<TypedEntity<CreatureInstance, &Value>> {
        let entity = self.entity_at_pos(position)?;

        Some(entity.as_typed::<CreatureInstance>())
    }

    pub fn entity_at_pos(&self, position: impl Borrow<Position>) -> Option<&Entity> {
        let game_state: &GameState = self.game_state.borrow();

        let entity_id = self.entity_id_at_pos(position)?;

        let entity = game_state.entity_arena().get(entity_id);

        Some(entity)
    }

    pub fn entity_id_at_pos(&self, position: impl Borrow<Position>) -> Option<EntityId> {
        let game_state: &GameState = self.game_state.borrow();

        game_state.positions_map().get(position.borrow()).copied()
    }
}
