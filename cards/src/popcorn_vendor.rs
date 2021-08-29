use async_trait::async_trait;
use salt_engine::{
    cards::{actions::UponSummonAction, CardDefinition, Position, UnitCardDefinition},
    game_logic::{
        BuffBuilder, EventDispatcher, PassiveCompanionBuff, PassiveEffectDefinition,
        PassiveEffectInstanceId,
    },
    game_state::{
        board::{BoardPos, RowId},
        GameState, UnitCardInstance,
    },
    id::Id,
};

#[derive(Debug, Clone)]
pub struct PopcornVendor;

impl PopcornVendor {
    fn id(&self) -> Id {
        // id::parse("...")
        todo!()
    }
}

impl CardDefinition for PopcornVendor {
    fn title(&self) -> &str {
        "Popcorn Vendor"
    }

    fn cost(&self) -> i32 {
        3
    }

    fn flavor_text(&self) -> &str {
        "todo"
    }

    fn text(&self) -> &str {
        "Front or Back
Front: +3 attack
Back: Companion
has +2/+2."
    }
}

impl UnitCardDefinition for PopcornVendor {
    fn attack(&self) -> i32 {
        0
    }

    fn health(&self) -> i32 {
        3
    }

    fn row_width(&self) -> usize {
        1
    }

    fn placeable_at(&self) -> Position {
        Position::Either
    }

    fn passive_effect(&self) -> Option<Box<dyn PassiveEffectDefinition>> {
        let buff = BuffBuilder::new(PassiveEffectInstanceId::new(), Id::new())
            .attack(2)
            .health(2)
            .build();

        let passive = PassiveCompanionBuff::new_for_row(Id::new(), Box::new(buff), RowId::BackRow);
        Some(Box::new(passive))
    }

    fn upon_summon(&self) -> Box<dyn UponSummonAction> {
        Box::new(SummonAction)
    }
}

struct SummonAction;

#[async_trait]
impl UponSummonAction for SummonAction {
    async fn action(
        &self,
        instance: &mut UnitCardInstance,
        pos: BoardPos,
        state: &mut GameState,
        dispatcher: &mut EventDispatcher,
    ) {
        if pos.row() == RowId::FrontRow {
            // Front: buffs self
            let buff = BuffBuilder::new(instance.id(), Id::new()).attack(3).build();

            instance.add_buff(Box::new(buff));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        tests::{make_default_dispatcher, make_test_state},
        Pawn,
    };
    use salt_engine::{game_logic::SummonCreatureFromHandEvent, game_state::board::BoardView};

    #[test]
    fn when_summoned_back_gives_buff() {
        let mut state = make_test_state();
        let mut dispatcher = make_default_dispatcher();
        let player_id = state.player_a_id();

        // Summon a pawn to receive the buff
        let pawn = Pawn.make_instance();
        let pawn_id = pawn.id();
        {
            let hand = state.hand_mut(player_id);
            hand.add_card(pawn);

            let pawn_pos = BoardPos::new(player_id, RowId::FrontRow, 0);
            let summon_event = SummonCreatureFromHandEvent::new(player_id, pawn_pos, pawn_id);
            dispatcher.dispatch(summon_event, &mut state);
        }

        // Summon the popcorn vendor and target the Pawn with the buff
        {
            let pop_vend = PopcornVendor.make_instance();
            let pop_vend_id = pop_vend.id();
            let hand = state.hand_mut(player_id);
            let pop_vend_pos = BoardPos::new(player_id, RowId::BackRow, 0);
            hand.add_card(pop_vend);
            let summon_event =
                SummonCreatureFromHandEvent::new(player_id, pop_vend_pos, pop_vend_id);
            dispatcher.dispatch(summon_event, &mut state);
        }

        let pawn_instance = state.board().creature_instance(pawn_id);

        assert!(
            !pawn_instance.buffs().is_empty(),
            "Expected the pawn to receive the buff."
        );
    }

    #[test]
    fn when_summoned_front_gets_buff() {
        let mut state = make_test_state();
        let mut dispatcher = make_default_dispatcher();

        let player_a = state.player_a_id();
        let hand = state.hand_mut(player_a);

        let popcorn_vendor = PopcornVendor.make_instance();
        let popcorn_vendor_id = popcorn_vendor.id();
        hand.add_card(popcorn_vendor);

        let pos = BoardPos::new(player_a, RowId::FrontRow, 0);
        let summon_event = SummonCreatureFromHandEvent::new(player_a, pos, popcorn_vendor_id);
        dispatcher.dispatch(summon_event, &mut state);

        let summoned_vendor = state.board().creature_instance(popcorn_vendor_id);

        assert!(
            !summoned_vendor.buffs().is_empty(),
            "Expected it to receive the buff."
        );
    }
}
