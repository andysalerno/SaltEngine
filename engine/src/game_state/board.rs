use super::game_state::GameState;
use crate::v2::CreatureInstance;
use entity_arena::{id::EntityId, Entity, IsEntity, TypedEntity, Value};
use protocol::entities::{BoardPos, EntityPosition};
use std::borrow::{Borrow, BorrowMut};

/// A view over a `GameState` that provides board-level functionality,
/// such as getting and setting `CreatureInstance`s at positions.
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
        position: impl Borrow<BoardPos>,
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

    pub fn entity_at_pos(&self, position: impl Borrow<BoardPos>) -> Option<&Entity> {
        let game_state: &GameState = self.game_state.borrow();

        let entity_id = self.entity_id_at_pos(position)?;

        let entity = game_state.entity_arena().get(entity_id);

        Some(entity)
    }

    pub fn entity_id_at_pos(&self, position: impl Borrow<BoardPos>) -> Option<EntityId> {
        let game_state: &GameState = self.game_state.borrow();

        let entity_pos = EntityPosition::BoardPos(*position.borrow());

        game_state.positions_map().get(&entity_pos).copied()
    }
}

impl<T> Board<T>
where
    T: BorrowMut<GameState>,
{
    pub fn set_creature_at_pos(
        &mut self,
        creature: CreatureInstance,
        position: impl Borrow<BoardPos>,
    ) {
        let game_state: &mut GameState = self.game_state.borrow_mut();
        let entity_arena = game_state.entity_arena_mut();
        let entity_id = entity_arena.add(creature);

        let entity_pos = EntityPosition::BoardPos(*position.borrow());
        game_state.positions_map_mut().insert(entity_pos, entity_id);
    }

    pub fn remove_entity_at_pos(&mut self, position: impl Borrow<BoardPos>) {
        let game_state: &mut GameState = self.game_state.borrow_mut();
        let position_mapping = game_state.positions_map_mut();

        // What entity is in that position?
        let entity_pos = EntityPosition::BoardPos(*position.borrow());
        let entity_id = position_mapping
            .remove(entity_pos.borrow())
            .expect("Attempted to remove at a position that had no entity.");

        game_state.entity_arena_mut().remove(entity_id);
    }
}

#[cfg(test)]
mod tests {
    use crate::v2::{CreatureDefinitionId, CreatureInstance};

    use super::GameState;
    use protocol::entities::{BoardPos, PlayerId, RowId};

    #[test]
    fn game_state_new_expects_can_get_board() {
        let player_a = PlayerId::new();
        let player_b = PlayerId::new();

        let game_state = GameState::new(player_a, player_b);

        let _board = game_state.board();
    }

    #[test]
    fn board_expects_can_set_creature_at_pos() {
        let player_a = PlayerId::new();
        let player_b = PlayerId::new();

        let mut game_state = GameState::new(player_a, player_b);

        let mut board = game_state.board_mut();

        let creature = CreatureInstance::new_from_definition_id(CreatureDefinitionId::new());

        let position = BoardPos::new(player_a, RowId::FrontRow, 0);

        board.set_creature_at_pos(creature, position);
    }

    #[test]
    fn board_expects_can_get_creature_at_pos() {
        let player_a = PlayerId::new();
        let player_b = PlayerId::new();

        let mut game_state = GameState::new(player_a, player_b);

        let mut board = game_state.board_mut();

        let position = BoardPos::new(player_a, RowId::FrontRow, 0);
        let found = board.creature_at_pos(position);

        assert!(
            found.is_none(),
            "Expected no creature since none was inserted yet."
        );

        let creature = CreatureInstance::new_from_definition_id(CreatureDefinitionId::new());

        board.set_creature_at_pos(creature, position);

        let found = board.creature_at_pos(position);

        assert!(found.is_some());
    }

    #[test]
    fn board_expects_can_remove_creature_at_pos() {
        let player_a = PlayerId::new();
        let player_b = PlayerId::new();

        let mut game_state = GameState::new(player_a, player_b);

        let mut board = game_state.board_mut();

        let position = BoardPos::new(player_a, RowId::FrontRow, 0);

        let creature = CreatureInstance::new_from_definition_id(CreatureDefinitionId::new());

        board.set_creature_at_pos(creature, position);

        let found = board.creature_at_pos(position);

        assert!(
            found.is_some(),
            "The creature was inserted, so expected it to be found."
        );

        board.remove_entity_at_pos(position);

        let found = board.creature_at_pos(position);

        assert!(
            found.is_none(),
            "The creature was removed, so expected it to not be found."
        );
    }
}
