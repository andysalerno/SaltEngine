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

// #[derive(Debug, Default)]
// pub struct Deck {
//     cards: Vec<CreatureInstance>,
// }

// impl Deck {
//     #[must_use]
//     pub fn len(&self) -> usize {
//         self.cards.len()
//     }

//     #[must_use]
//     pub fn is_empty(&self) -> bool {
//         self.len() == 0
//     }

//     pub fn draw_card(&mut self) -> Option<CreatureInstance> {
//         self.cards.pop()
//     }

//     #[must_use]
//     pub fn new(cards: Vec<CreatureInstance>) -> Self {
//         Self { cards }
//     }

//     pub fn shuffle(&mut self) {
//         self.cards.shuffle(&mut rand::thread_rng());
//     }
// }
