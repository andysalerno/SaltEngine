use entity_arena::{EntityArena, IsEntity, TypedEntity, Value};
use protocol::entities::PlayerId;

use super::deck::DeckEntity;

enum Position {
    Hand,
    Board,
    Abyss,
}

struct CardEntity<T: IsEntity> {
    obj: T,
    position: Position,
}

/// Engine will internally maintain one GameState, with full visibility
/// of all state (cards in both players hands, deck content, etc)
/// and will create a sanitized copy that is sent to players
#[derive(Debug)]
struct GameState {
    player_a_id: PlayerId,
    player_b_id: PlayerId,
    entity_arena: EntityArena,
}

impl GameState {
    fn new(player_a_id: PlayerId, player_b_id: PlayerId) -> Self {
        let mut state = Self {
            player_a_id,
            player_b_id,
            entity_arena: EntityArena::new(),
        };

        let deck_a = DeckEntity::new(player_a_id);
        let deck_b = DeckEntity::new(player_b_id);

        state.entity_arena.add(deck_a);
        state.entity_arena.add(deck_b);

        state
    }

    pub fn deck(&self, player_id: PlayerId) -> TypedEntity<DeckEntity, &Value> {
        self.entity_arena
            .of_type::<DeckEntity>()
            .find(|d| d.get(|d| d.player_id() == player_id))
            .unwrap()
    }

    pub fn deck_mut(&mut self, player_id: PlayerId) -> TypedEntity<DeckEntity, &mut Value> {
        self.entity_arena
            .of_type_mut::<DeckEntity>()
            .find(|d| d.get(|d| d.player_id() == player_id))
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::game_state::{
        card_in_deck_entity::CardInDeck, creature_definition::CreatureDefinitionId,
        deck::DeckEntity,
    };

    use super::GameState;
    use protocol::entities::PlayerId;

    #[test]
    fn game_state_new_expects_can_get_deck() {
        let player_a = PlayerId::new();
        let player_b = PlayerId::new();

        let game_state = GameState::new(player_a, player_b);

        let deck_a = game_state.deck(player_a);
        let deck_b = game_state.deck(player_b);

        assert_ne!(deck_a.id(), deck_b.id());
    }

    #[test]
    fn game_state_new_expects_can_add_cards_to_deck() {
        let player_a = PlayerId::new();
        let player_b = PlayerId::new();

        let mut game_state = GameState::new(player_a, player_b);

        let player_a_card_1 = CardInDeck::new(CreatureDefinitionId::new());
        let player_a_card_2 = CardInDeck::new(CreatureDefinitionId::new());
        let player_a_card_3 = CardInDeck::new(CreatureDefinitionId::new());

        game_state.deck_mut(player_a).get_mut(|d| {
            d.add_card(player_a_card_1);
            d.add_card(player_a_card_2);
            d.add_card(player_a_card_3);
        });

        let player_b_card_1 = CardInDeck::new(CreatureDefinitionId::new());
        let player_b_card_2 = CardInDeck::new(CreatureDefinitionId::new());

        game_state.deck_mut(player_b).get_mut(|d| {
            d.add_card(player_b_card_1);
            d.add_card(player_b_card_2);
        });

        let player_a_deck_len = game_state.deck(player_a).get(DeckEntity::len);
        let player_b_deck_len = game_state.deck(player_b).get(DeckEntity::len);

        assert_eq!(3, player_a_deck_len);
        assert_eq!(2, player_b_deck_len);
    }
}
