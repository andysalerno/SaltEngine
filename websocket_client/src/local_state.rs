use protocol::entities::{
    AsId, BoardPos, Entity, EntityId, EntityPosition, Hand, Id, IsEntity, PlayerHero, PlayerId,
    UnitCardInstance, UnitCardInstanceId,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A struct that represents the client-side state of the game.
/// This struct keeps track of all entities.
/// Entities are indexed by both id and position within the game world.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LocalState {
    player_a_id: PlayerId,
    player_b_id: PlayerId,

    // All entities, mapped by `Id` of the entity.
    entities: HashMap<Id, Entity>,

    /// The entities in the hand for player_a.
    player_a_hand: Vec<UnitCardInstanceId>,

    /// The entities in the hand for player_b.
    player_b_hand: Vec<UnitCardInstanceId>,

    /// All entities, mapped by their location.
    positions: HashMap<EntityPosition, UnitCardInstanceId>,
}

impl LocalState {
    #[must_use]
    pub fn new(player_id: PlayerId, opponent_id: PlayerId) -> Self {
        Self {
            player_a_id: player_id,
            player_b_id: opponent_id,
            entities: HashMap::new(),
            player_a_hand: Vec::new(),
            player_b_hand: Vec::new(),
            positions: HashMap::new(),
        }
    }

    /// Given an id, return the matching entity.
    pub fn find_entity(&self, id: impl AsId) -> &Entity {
        let id = id.as_id();
        self.entities.get(&id).unwrap()
    }

    /// Given an id, return the matching entity, unpacked as the internal type.
    pub fn find<T: EntityId>(&self, id: T) -> T::EntityType {
        let id = id.as_id();
        let entity = self.entities.get(&id).unwrap();

        let unpacked: T::EntityType = entity.unpack_copy();

        unpacked
    }

    /// Finds all entities that represent the specified type.
    pub fn find_type<T: IsEntity>(&self) -> impl Iterator<Item = T> + '_ {
        let type_id = T::type_id();
        self.entities
            .values()
            .filter(move |e| e.type_id() == type_id)
            .map(protocol::entities::Entity::unpack_copy)
    }

    #[deprecated]
    pub fn add<T: IsEntity>(&mut self, to_add: T) {
        let entity = to_add.as_entity();
        self.entities.insert(entity.id(), entity);
    }

    /// Adds a new entity at the given position.
    pub fn add_at<T: IsEntity>(&mut self, to_add: T, position: EntityPosition) {
        let entity = to_add.as_entity();
        let id = entity.id();
        self.entities.insert(id, entity);

        let card_id = UnitCardInstanceId::from(id);
        self.positions.insert(position, card_id);

        if let EntityPosition::Hand(player_id) = position {
            if player_id == self.player_a_id {
                self.player_a_hand.push(card_id);
            } else if player_id == self.player_b_id {
                self.player_b_hand.push(card_id);
            } else {
                panic!("Unknown player id: {player_id:?}");
            }
        }
    }

    /// Updates the existing entity (found by matching id) by replacing it with `to_update`.
    pub fn update<T: IsEntity>(&mut self, to_update: T) {
        let found = self.entities.get_mut(&to_update.id().as_id()).unwrap();
        *found = to_update.as_entity();
    }

    #[must_use]
    #[deprecated]
    pub fn player_hand(&self, player_id: PlayerId) -> Hand {
        let hand = self
            .find_type::<Hand>()
            .find(|h| h.player_id == player_id)
            .unwrap();

        hand
    }

    /// Gets an iterator over the card instances in the given player's hand.
    pub fn cards_in_player_hand(
        &self,
        player_id: PlayerId,
    ) -> impl Iterator<Item = UnitCardInstance> + '_ {
        let player_hand = if player_id == self.player_a_id {
            &self.player_a_hand
        } else if player_id == self.player_b_id {
            &self.player_b_hand
        } else {
            panic!("Unknown player id: {:?}", player_id)
        };

        player_hand.iter().map(|id| self.find(*id))
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
}

#[cfg(test)]
mod test {
    use super::LocalState;
    use protocol::entities::{
        BuffInstanceId, BuffPlayerView, BuffSourceId, EntityPosition, Hand, HandId, HasId, Id,
        PlayerId, UnitCardDefinition, UnitCardInstance, UnitCardInstanceId,
    };

    #[test]
    fn can_add_entity() {
        let mut state = LocalState::default();

        let buff_view = BuffPlayerView {
            attack_amount: 10,
            health_amount: 11,
            source_id: BuffSourceId::Other(Id::new()),
            instance_id: BuffInstanceId::new(),
            definition_id: Id::new(),
            is_from_passive: false,
        };

        state.add(buff_view);
    }

    #[test]
    fn can_retrieve_entity() {
        let mut state = LocalState::default();

        let buff_view = BuffPlayerView {
            attack_amount: 10,
            health_amount: 11,
            source_id: BuffSourceId::Other(Id::new()),
            instance_id: BuffInstanceId::new(),
            definition_id: Id::new(),
            is_from_passive: false,
        };

        let id = buff_view.instance_id;

        state.add(buff_view);

        let retrieved = state.find(id);

        assert_eq!(10, retrieved.attack_amount);
        assert_eq!(11, retrieved.health_amount);
    }

    #[test]
    fn can_update_entity() {
        let mut state = LocalState::default();

        let buff_view = BuffPlayerView {
            attack_amount: 10,
            health_amount: 11,
            source_id: BuffSourceId::Other(Id::new()),
            instance_id: BuffInstanceId::new(),
            definition_id: Id::new(),
            is_from_passive: false,
        };

        let id = buff_view.instance_id;

        state.add(buff_view);

        let mut retrieved = state.find(id);

        retrieved.attack_amount = 9;

        state.update(retrieved);

        let retrieved = state.find(id);

        assert_eq!(9, retrieved.attack_amount);
    }

    #[test]
    fn can_add_multiple_entity_types() {
        let mut state = LocalState::default();

        let buff_view = BuffPlayerView {
            attack_amount: 10,
            health_amount: 11,
            source_id: BuffSourceId::Other(Id::new()),
            instance_id: BuffInstanceId::new(),
            definition_id: Id::new(),
            is_from_passive: false,
        };

        state.add(buff_view);

        let hand = Hand {
            player_id: PlayerId::new(),
            id: HandId::new(),
            cards: Vec::new(),
        };

        state.add(hand);
    }

    #[test]
    fn can_find_entity_by_type() {
        let mut state = LocalState::default();

        let buff_view = BuffPlayerView {
            attack_amount: 10,
            health_amount: 11,
            source_id: BuffSourceId::Other(Id::new()),
            instance_id: BuffInstanceId::new(),
            definition_id: Id::new(),
            is_from_passive: false,
        };

        state.add(buff_view);

        let hand = Hand {
            player_id: PlayerId::new(),
            id: HandId::new(),
            cards: Vec::new(),
        };

        state.add(hand);

        assert_eq!(1, state.find_type::<Hand>().count());
    }

    #[test]
    fn can_find_entity_by_type_multiple() {
        let mut state = LocalState::default();

        let buff_view = BuffPlayerView {
            attack_amount: 10,
            health_amount: 11,
            source_id: BuffSourceId::Other(Id::new()),
            instance_id: BuffInstanceId::new(),
            definition_id: Id::new(),
            is_from_passive: false,
        };

        state.add(buff_view);

        let hand = Hand {
            player_id: PlayerId::new(),
            id: HandId::new(),
            cards: Vec::new(),
        };

        state.add(hand);

        let hand_2 = Hand {
            player_id: PlayerId::new(),
            id: HandId::new(),
            cards: Vec::new(),
        };

        state.add(hand_2);

        assert_eq!(2, state.find_type::<Hand>().count());
    }

    #[test]
    fn can_find_player_hand() {
        let mut state = LocalState::default();

        // Add some other entity type to the state, first
        {
            let buff_view = BuffPlayerView {
                attack_amount: 10,
                health_amount: 11,
                source_id: BuffSourceId::Other(Id::new()),
                instance_id: BuffInstanceId::new(),
                definition_id: Id::new(),
                is_from_passive: false,
            };

            state.add(buff_view);
        }

        // Then add a hand for player 1
        let player_id_1 = PlayerId::new();
        let hand_id_1 = HandId::new();
        {
            let hand_1 = Hand {
                player_id: player_id_1,
                id: hand_id_1,
                cards: Vec::new(),
            };

            state.add(hand_1);
        }

        // Then add a hand for player 2
        let player_id_2 = PlayerId::new();
        let hand_id_2 = HandId::new();
        {
            let hand_2 = Hand {
                player_id: player_id_2,
                id: hand_id_2,
                cards: Vec::new(),
            };

            state.add(hand_2);
        }

        // Expect the player 1 hand to be found correctly
        {
            let player_1_hand = state.player_hand(player_id_1);
            assert_eq!(hand_id_1, player_1_hand.id());
        }

        // Expect the player 2 hand to be found correctly
        {
            let player_2_hand = state.player_hand(player_id_2);
            assert_eq!(hand_id_2, player_2_hand.id());
        }
    }

    #[test]
    fn can_find_player_hand_2() {
        let player_a = PlayerId::new();
        let player_b = PlayerId::new();
        let mut state = LocalState::new(player_a, player_b);

        // Add a few cards to player 1's hand
        {
            let card_1 = UnitCardInstance::new(
                UnitCardInstanceId::new(),
                UnitCardDefinition::new("hello"),
                Vec::new(),
                None,
            );

            let card_2 = UnitCardInstance::new(
                UnitCardInstanceId::new(),
                UnitCardDefinition::new("hello"),
                Vec::new(),
                None,
            );

            let card_3 = UnitCardInstance::new(
                UnitCardInstanceId::new(),
                UnitCardDefinition::new("hello"),
                Vec::new(),
                None,
            );

            let player_a_hand = EntityPosition::Hand(player_a);

            state.add_at(card_1, player_a_hand);
            state.add_at(card_2, player_a_hand);
            state.add_at(card_3, player_a_hand);
        }

        // Expect the cards to be found in player 1's hand
        let hand = state.cards_in_player_hand(player_a);
        assert!(hand.count() == 3);

        let hand = state.cards_in_player_hand(player_b);
        assert!(hand.count() == 0);
    }
}
