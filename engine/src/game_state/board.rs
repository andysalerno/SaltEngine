use super::game_state::GameState;
use crate::v2::CreatureInstance;
use entity_arena::{id::EntityId, Entity, IsEntity, TypedEntity, Value};
use protocol::entities::EntityPosition;
use std::borrow::{Borrow, BorrowMut};

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
        position: impl Borrow<EntityPosition>,
    ) -> Option<TypedEntity<CreatureInstance, &Value>> {
        let entity = self.entity_at_pos(position)?;

        assert!(
            (entity.entity_type_id() == CreatureInstance::entity_type_id()),
            "Expected entity {:?} to have type id {:?} but had type id {:?}",
            entity.id(),
            CreatureInstance::entity_type_id(),
            entity.entity_type_id()
        );

        Some(entity.as_typed::<CreatureInstance>())
    }

    pub fn entity_at_pos(&self, position: impl Borrow<EntityPosition>) -> Option<&Entity> {
        let game_state: &GameState = self.game_state.borrow();

        let entity_id = self.entity_id_at_pos(position)?;

        let entity = game_state.entity_arena().get(entity_id);

        Some(entity)
    }

    pub fn entity_id_at_pos(&self, position: impl Borrow<EntityPosition>) -> Option<EntityId> {
        let game_state: &GameState = self.game_state.borrow();

        game_state.positions_map().get(position.borrow()).copied()
    }
}

impl<T> Board<T>
where
    T: BorrowMut<GameState>,
{
    pub fn set_creature_at_pos(
        &mut self,
        creature: CreatureInstance,
        position: impl Borrow<EntityPosition>,
    ) {
        let game_state: &mut GameState = self.game_state.borrow_mut();
        let entity_arena = game_state.entity_arena_mut();
        let entity_id = entity_arena.add(creature);

        let position_mapping = game_state.positions_map_mut();
        position_mapping.insert(*position.borrow(), entity_id);
        // let pos_val = position_mapping
        //     .entry(*position.borrow())
        //     .or_insert(entity_id);
        // *pos_val = entity_id;
    }
}
