use super::{card_in_hand_entity::CardInHand, game_state::GameState};
use entity_arena::{id::EntityId, TypedEntity, Value};
use protocol::entities::{EntityPosition, PlayerId};
use std::borrow::{Borrow, BorrowMut};

pub struct Hand<T>
where
    T: Borrow<GameState>,
{
    player_id: PlayerId,
    game_state: T,
}

impl<T> Hand<T>
where
    T: Borrow<GameState>,
{
    pub fn new(game_state: T, player_id: PlayerId) -> Self {
        let g = game_state.borrow();
        assert!(
            !(player_id != g.player_a_id() && player_id != g.player_b_id()),
            "PlayerId {player_id:?} was not part of the GameState."
        );

        Self {
            player_id,
            game_state,
        }
    }

    pub fn entity_ids(&self) -> Vec<EntityId> {
        self.game_state
            .borrow()
            .positions_map()
            .iter()
            .filter(|(k, _)| matches!(k, EntityPosition::Hand(p, _) if *p == self.player_id))
            .map(|(_, v)| v)
            .copied()
            .collect()
    }

    pub fn cards(&self) -> impl Iterator<Item = TypedEntity<CardInHand, &Value>> {
        let ids = self.entity_ids();

        let arena = self.game_state.borrow().entity_arena();

        arena
            .of_type::<CardInHand>()
            .filter(move |c| ids.contains(&c.id()))
    }
}

impl<T> Hand<T>
where
    T: BorrowMut<GameState>,
{
    pub fn add_card(&mut self, card: CardInHand) {
        let game_state = self.game_state.borrow_mut();
        let entity_arena = game_state.entity_arena_mut();
        let entity_id = entity_arena.add(card);

        let entity_pos = EntityPosition::Hand(self.player_id, 9);
        game_state.positions_map_mut().insert(entity_pos, entity_id);
    }
}
