use super::card_in_deck_entity::CardInDeck;
use entity_arena::{
    id::{EntityId, EntityTypeId},
    IsEntity,
};
use isentity_macro_derive::entity;
use protocol::entities::PlayerId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[entity("2819f025-47c8-41fa-aad0-b9eefc515ae1")]
pub struct DeckEntity {
    id: EntityId,
    player_id: PlayerId,
    cards: Vec<CardInDeck>,
}

impl DeckEntity {
    pub fn new(player_id: PlayerId) -> Self {
        Self {
            id: EntityId::new(),
            cards: Vec::new(),
            player_id,
        }
    }

    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }

    pub fn add_card(&mut self, to_add: CardInDeck) {
        self.cards.push(to_add);
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }
}
