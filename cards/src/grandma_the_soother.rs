use async_trait::async_trait;
use log::info;
use salt_engine::{
    cards::{actions::UponEventAction, CardDefinition, Position, UnitCardDefinition},
    game_logic::{
        events::{DrawCardEvent, GameEvent},
        EventDispatcher,
    },
    game_state::{board::BoardView, GameState, PlayerId, UnitCardInstanceId},
    id::Id,
};

#[derive(Debug, Clone)]
pub struct GrandmaTheSoother;

impl GrandmaTheSoother {
    fn id(&self) -> Id {
        // id::parse("...")
        todo!()
    }
}

impl CardDefinition for GrandmaTheSoother {
    fn title(&self) -> &str {
        "Grandma The Soother"
    }

    fn cost(&self) -> i32 {
        2
    }

    fn flavor_text(&self) -> &str {
        "todo"
    }

    fn text(&self) -> &str {
        "Back.
After her companion
takes damage and survives,
draw a card."
    }
}

impl UnitCardDefinition for GrandmaTheSoother {
    fn attack(&self) -> i32 {
        0
    }

    fn health(&self) -> i32 {
        2
    }

    fn row_width(&self) -> usize {
        1
    }

    fn placeable_at(&self) -> Position {
        Position::Back
    }

    fn post_event_action(
        &self,
        card_instance_id: UnitCardInstanceId,
        event: &GameEvent,
        game_state: &GameState,
        _dispatcher: &mut EventDispatcher,
    ) -> Option<Box<dyn UponEventAction>> {
        let damage_event = if let GameEvent::CreatureTakesDamageEvent(e) = event {
            e
        } else {
            return None;
        };

        let grandma_pos = game_state.board().pos_with_creature(card_instance_id);
        let companion = game_state.board().companion_creature(grandma_pos)?;

        if companion.id() == damage_event.creature_id() {
            info!("Grandma's companion was damaged");
            Some(Box::new(GrandmasKissesAction::new(grandma_pos.player_id))
                as Box<dyn UponEventAction>)
        } else {
            None
        }
    }
}

pub struct GrandmasKissesAction {
    player_id: PlayerId,
}

impl GrandmasKissesAction {
    fn new(player_id: PlayerId) -> Self {
        Self { player_id }
    }
}

#[async_trait]
impl UponEventAction for GrandmasKissesAction {
    async fn action(
        &self,
        _event: &GameEvent,
        state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        info!("Grandma The Soother triggers card draw");
        let draw_card = DrawCardEvent::new(self.player_id);
        dispatcher.dispatch(draw_card, state).await;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        tests::{make_dispatcher, make_test_state},
        IndoorCat, Pawn,
    };
    use salt_engine::{
        game_logic::events::{CreatureTakesDamageEvent, SummonCreatureFromHandEvent},
        game_state::board::{BoardPos, RowId},
    };

    #[test]
    fn when_companion_takes_damage_and_survives_expects_draws_card() {
        let _ = env_logger::builder().is_test(true).try_init();
        let mut state = make_test_state();
        let mut dispatcher = make_dispatcher(state.player_a_id(), state.player_b_id());
        let player_id = state.player_a_id();

        // Summon a pawn to receive the buff
        let cat = IndoorCat.make_instance();
        let cat_id = cat.id();
        {
            let hand = state.hand_mut(player_id);
            hand.add_card(cat);

            let pawn_pos = BoardPos::new(player_id, RowId::FrontRow, 0);
            let summon_event = SummonCreatureFromHandEvent::new(player_id, pawn_pos, cat_id);

            smol::block_on(async {
                dispatcher.dispatch(summon_event, &mut state).await;
            });
        }

        // Summon grandma
        {
            let grandma = GrandmaTheSoother.make_instance();
            let grandma_id = grandma.id();
            let hand = state.hand_mut(player_id);
            let grandma_pos = BoardPos::new(player_id, RowId::BackRow, 0);
            hand.add_card(grandma);
            let summon_event = SummonCreatureFromHandEvent::new(player_id, grandma_pos, grandma_id);

            smol::block_on(async {
                dispatcher.dispatch(summon_event, &mut state).await;
            });
        }

        let original_hand_size = state.hand(player_id).len();

        // Damage the pawn (to trigger Grandma)
        let damage_event = CreatureTakesDamageEvent::new(cat_id, 1);
        smol::block_on(async {
            dispatcher.dispatch(damage_event, &mut state).await;
        });

        let hand_size_after = state.hand(player_id).len();

        assert_eq!(
            hand_size_after,
            original_hand_size + 1,
            "Expected the hand size to be higher since Grandma should have been triggered."
        );
    }

    #[test]
    fn when_companion_takes_damage_and_dies_expects_not_draws_card() {
        let _ = env_logger::builder().is_test(true).try_init();
        let mut state = make_test_state();
        let mut dispatcher = make_dispatcher(state.player_a_id(), state.player_b_id());
        let player_id = state.player_a_id();

        // Summon a pawn to receive the buff
        let pawn = Pawn.make_instance();
        let pawn_id = pawn.id();
        {
            let hand = state.hand_mut(player_id);
            hand.add_card(pawn);

            let pawn_pos = BoardPos::new(player_id, RowId::FrontRow, 0);
            let summon_event = SummonCreatureFromHandEvent::new(player_id, pawn_pos, pawn_id);

            smol::block_on(async {
                dispatcher.dispatch(summon_event, &mut state).await;
            });
        }

        // Summon grandma
        info!("Summoning grandma");
        {
            let grandma = GrandmaTheSoother.make_instance();
            let grandma_id = grandma.id();
            let hand = state.hand_mut(player_id);
            let grandma_pos = BoardPos::new(player_id, RowId::BackRow, 0);
            hand.add_card(grandma);
            let summon_event = SummonCreatureFromHandEvent::new(player_id, grandma_pos, grandma_id);

            smol::block_on(async {
                dispatcher.dispatch(summon_event, &mut state).await;
            });
        }

        let original_hand_size = state.hand(player_id).len();

        // Damage the pawn (to trigger Grandma)
        let damage_event = CreatureTakesDamageEvent::new(pawn_id, 1);
        smol::block_on(async {
            dispatcher.dispatch(damage_event, &mut state).await;
        });

        let hand_size_after = state.hand(player_id).len();

        assert_eq!(
            hand_size_after, original_hand_size,
            "Expected the hand size to be the same since grandma should not have triggered"
        );
    }

    #[test]
    fn when_non_companion_takes_damage_expects_not_draws_card() {
        let _ = env_logger::builder().is_test(true).try_init();
        let mut state = make_test_state();
        let mut dispatcher = make_dispatcher(state.player_a_id(), state.player_b_id());
        let player_id = state.player_a_id();

        // Summon a card to receive the buff
        let cat = IndoorCat.make_instance();
        let cat_id = cat.id();
        {
            let hand = state.hand_mut(player_id);
            hand.add_card(cat);

            let cat_pos = BoardPos::new(player_id, RowId::FrontRow, 0);
            let summon_event = SummonCreatureFromHandEvent::new(player_id, cat_pos, cat_id);

            smol::block_on(async {
                dispatcher.dispatch(summon_event, &mut state).await;
            });
        }

        // Summon grandma
        info!("Summoning grandma");
        {
            let grandma = GrandmaTheSoother.make_instance();
            let grandma_id = grandma.id();
            let hand = state.hand_mut(player_id);
            let grandma_pos = BoardPos::new(player_id, RowId::BackRow, 1);
            hand.add_card(grandma);
            let summon_event = SummonCreatureFromHandEvent::new(player_id, grandma_pos, grandma_id);

            smol::block_on(async {
                dispatcher.dispatch(summon_event, &mut state).await;
            });
        }

        let original_hand_size = state.hand(player_id).len();

        // Damage the pawn (to trigger Grandma)
        let damage_event = CreatureTakesDamageEvent::new(cat_id, 1);
        smol::block_on(async {
            dispatcher.dispatch(damage_event, &mut state).await;
        });

        let hand_size_after = state.hand(player_id).len();

        assert_eq!(
            hand_size_after, original_hand_size,
            "Expected the hand size to be the same since grandma should not have triggered"
        );
    }
}
