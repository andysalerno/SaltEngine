use super::{
    board::{Board, BoardView},
    HandView,
};
use protocol::entities::{
    AsId, BoardPos, CreatureInstance, CreatureInstanceId, Entity, EntityId, EntityPosition, Hand,
    HasId, Id, IsEntity, PlayerHero, PlayerId,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A trait defining a view of a `GameState`.
pub trait GameStateView<'a> {
    type HandView: HandView<'a>;
    type BoardView: BoardView<'a>;

    fn cur_player_turn(&self) -> PlayerId;
    fn opponent_id(&self, player_id: PlayerId) -> PlayerId;

    fn player_mana(&self, player_id: PlayerId) -> u32;
    fn player_mana_limit(&self, player_id: PlayerId) -> u32;

    fn board(&self) -> &Self::BoardView;
    fn hand(&self, player_id: PlayerId) -> &Self::HandView;

    fn iter(&'a self) -> std::slice::Iter<'_, <Self::BoardView as BoardView<'a>>::SlotView> {
        self.board().slots_iter()
    }
}

// state the client gets, which is basically just a display-only version of gstate
// matches LocalState in reality
// GState has Into<DisplayState> or something like make_player_view(PlayerId)
// this is synchronized to the clients
pub struct DisplayState {}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GameState {
    player_a_id: PlayerId,
    player_b_id: PlayerId,

    // All entities, mapped by `Id` of the entity.
    entities: HashMap<CreatureInstanceId, CreatureInstance>,

    /// Positions of all known entities, mapped to the resident entity id.
    positions: HashMap<EntityPosition, CreatureInstanceId>,
}

impl GameState {
    #[must_use]
    pub fn new(player_id: PlayerId, opponent_id: PlayerId) -> Self {
        Self {
            player_a_id: player_id,
            player_b_id: opponent_id,
            entities: HashMap::new(),
            positions: HashMap::new(),
        }
    }

    /// Given an id, return the matching entity.
    pub fn find_entity(&self, id: CreatureInstanceId) -> &CreatureInstance {
        self.entities.get(&id).unwrap()
    }

    /// Adds a new entity at the given position.
    pub fn add_at(&mut self, to_add: CreatureInstance, position: EntityPosition) {
        let id = to_add.id();
        self.entities.insert(id, to_add);
        self.positions.insert(position, id);
    }

    /// Gets an iterator over the card instances in the given player's hand.
    pub fn cards_in_player_hand(
        &self,
        player_id: PlayerId,
    ) -> impl Iterator<Item = &CreatureInstance> + '_ {
        if player_id != self.player_a_id && player_id != self.player_b_id {
            panic!("Unknown player id: {player_id:?}")
        }

        let mut matching_creature_ids: Vec<_> = self
            .positions
            .iter()
            .filter_map(|(position, creature_instance_id)| match position {
                EntityPosition::Hand(pos_player_hand_id, pos_in_hand) => {
                    if *pos_player_hand_id == player_id {
                        Some((creature_instance_id, pos_in_hand))
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect();

        // Make sure the hand is sorted.
        matching_creature_ids.sort_by_key(|(id, &pos_in_hand)| pos_in_hand);

        matching_creature_ids
            .into_iter()
            .map(|(id, _)| self.entities.get(id).unwrap())
    }

    /// Get a reference to the local state's player a id.
    #[must_use]
    pub fn player_a_id(&self) -> PlayerId {
        self.player_a_id
    }

    /// Get a reference to the local state's player b id.
    #[must_use]
    pub fn player_b_id(&self) -> PlayerId {
        self.player_b_id
    }

    /// Set the local state's player a id.
    pub fn set_player_a_id(&mut self, player_a_id: PlayerId) {
        self.player_a_id = player_a_id;
    }

    /// Set the local state's player b id.
    pub fn set_player_b_id(&mut self, player_b_id: PlayerId) {
        self.player_b_id = player_b_id;
    }

    pub fn is_game_over(&self) -> bool {
        todo!()
    }

    pub fn cur_player_id(&self) -> PlayerId {
        todo!()
    }

    pub fn board(&self) -> &Board {
        todo!()
    }

    pub fn board_mut(&mut self) -> &mut Board {
        todo!()
    }

    pub fn draw_card(&mut self, _: PlayerId) {
        todo!()
    }

    pub fn set_next_player_turn(&mut self) -> PlayerId {
        todo!()
    }

    pub fn raise_mana_limit(&mut self, _: PlayerId, _: u32) {
        todo!()
    }

    pub fn reduce_mana(&mut self, _: PlayerId, _: u32) {
        todo!()
    }

    pub fn is_pos_defended(&self, _: BoardPos) -> bool {
        todo!()
    }

    pub fn hand_mut(&mut self, _: PlayerId) -> &mut Hand {
        todo!()
    }

    pub fn hand(&self, _: PlayerId) -> &Hand {
        todo!()
    }
}

impl<'a> GameStateView<'a> for GameState {
    type HandView = super::Hand;

    type BoardView = Board;

    fn cur_player_turn(&self) -> PlayerId {
        todo!()
    }

    fn opponent_id(&self, player_id: PlayerId) -> PlayerId {
        todo!()
    }

    fn player_mana(&self, player_id: PlayerId) -> u32 {
        todo!()
    }

    fn player_mana_limit(&self, player_id: PlayerId) -> u32 {
        todo!()
    }

    fn board(&self) -> &Self::BoardView {
        todo!()
    }

    fn hand(&self, player_id: PlayerId) -> &Self::HandView {
        todo!()
    }

    fn iter(&'a self) -> std::slice::Iter<'_, <Self::BoardView as BoardView<'a>>::SlotView> {
        self.board().slots_iter()
    }
}

#[cfg(test)]
mod test {}
