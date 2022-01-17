use protocol::entities::{AsId, Entity, EntityId, Hand, Id, IsEntity, PlayerId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default)]
pub struct LocalState {
    player_a_id: PlayerId,
    player_b_id: PlayerId,
    entities: HashMap<Id, Entity>,
}

impl LocalState {
    #[must_use]
    pub fn new(player_id: PlayerId, opponent_id: PlayerId) -> Self {
        let mut state = Self {
            player_a_id: player_id,
            player_b_id: opponent_id,
            entities: HashMap::new(),
        };

        // both player's have a Hand
        let player_hand = Hand::new(player_id);
        let opponent_hand = Hand::new(opponent_id);

        state.add(player_hand);
        state.add(opponent_hand);

        state
    }

    pub fn find_entity<T: EntityId>(&self, id: T) -> &Entity {
        let id = id.as_id();
        self.entities.get(&id).unwrap()
    }

    pub fn find<T: EntityId>(&self, id: T) -> T::EntityType {
        let id = id.as_id();
        let entity = self.entities.get(&id).unwrap();

        let unpacked: T::EntityType = Self::unpack(entity);

        unpacked
    }

    pub fn find_type<T: IsEntity>(&self) -> impl Iterator<Item = &Entity> {
        let type_id = T::type_id();
        self.entities.values().filter(move |e| e.type_id == type_id)
    }

    pub fn add<T: IsEntity>(&mut self, to_add: T) {
        let entity = to_add.as_entity();
        self.entities.insert(entity.id, entity);
    }

    pub fn update<T: IsEntity>(&mut self, to_update: T) {
        let found = self.entities.get_mut(&to_update.id().as_id()).unwrap();
        *found = to_update.as_entity();
    }

    #[must_use]
    pub fn player_hand(&self, player_id: PlayerId) -> Hand {
        let hand = self
            .find_type::<Hand>()
            .map(Self::unpack::<Hand>)
            .find(|h| h.player_id == player_id)
            .unwrap();

        hand
    }

    fn unpack<T: IsEntity>(e: &Entity) -> T {
        let unpacked: T = serde_json::from_str(e.data.as_str()).unwrap();

        unpacked
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
        BuffInstanceId, BuffPlayerView, BuffSourceId, Hand, HandId, HasId, Id, PlayerId,
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
}
