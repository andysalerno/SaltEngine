use super::{
    card_in_hand_entity::CardInHand,
    game_state::{GameState, Position},
};
use entity_arena::{id::EntityId, TypedEntity, Value};
use protocol::entities::PlayerId;
use std::borrow::Borrow;

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
            .filter(|(k, _)| matches!(k, Position::Hand(p) if *p == self.player_id))
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
