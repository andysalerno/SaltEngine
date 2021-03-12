use crate::{
    game_logic::{
        buff::{Buff, BuffSourceId},
        BuffInstanceId,
    },
    game_state::{board::RowId, UnitCardInstanceId},
    id::Id,
};
use crate::{
    game_logic::{EventDispatcher, PassiveEffectDefinition},
    game_state::{board::BoardPos, GameState, UnitCardInstance},
};

use super::{CardDefinition, Position, UnitCardDefinition};

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
        1
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
        Some(Box::new(buff_other::PopcornVendorPassive))
    }

    fn upon_summon(
        &self,
    ) -> Box<dyn FnOnce(&mut UnitCardInstance, BoardPos, &mut GameState, &mut EventDispatcher)>
    {
        Box::new(|instance, pos, _game_state, _dispatcher| {
            if pos.row() == RowId::FrontRow {
                // Front: buffs self
                instance.add_buff(Box::new(buff_self::PopcornVendorBuff::new(instance.id())));
            }
        })
    }
}

mod buff_self {
    use super::*;

    #[derive(Debug)]
    pub struct PopcornVendorBuff {
        instance_id: BuffInstanceId,
        source_id: BuffSourceId,
    }

    impl PopcornVendorBuff {
        pub fn new(source_id: UnitCardInstanceId) -> Self {
            Self {
                instance_id: BuffInstanceId::new(),
                source_id: BuffSourceId::CreatureInstance(source_id),
            }
        }
    }

    impl Buff for PopcornVendorBuff {
        fn attack_amount(&self) -> i32 {
            3
        }

        fn health_amount(&self) -> i32 {
            0
        }

        fn instance_id(&self) -> BuffInstanceId {
            self.instance_id
        }

        fn source_id(&self) -> BuffSourceId {
            self.source_id
        }

        fn definition_id(&self) -> Id {
            todo!()
        }
    }
}

mod buff_other {
    use super::*;
    use crate::game_logic::passive_effect::PassiveEffectInstanceId;

    #[derive(Debug)]
    pub struct PopcornVendorPassive;

    impl PassiveEffectDefinition for PopcornVendorPassive {
        fn definition_id(&self) -> Id {
            todo!()
        }

        fn update(
            &self,
        ) -> Box<dyn FnOnce(PassiveEffectInstanceId, UnitCardInstanceId, &mut GameState)> {
            Box::new(move |instance_id, originator_id, game_state| {
                let instance_pos = game_state.board().position_with_creature(originator_id);

                if let Some(companion) = game_state.board().companion_creature(instance_pos) {
                    game_state.update_by_id(companion.id(), |c| {
                        println!("Applying buff to companion of popcorn vendor.");
                        c.add_buff(Box::new(PopcornVendorBuff::new(instance_id)));
                    });
                }
            })
        }
    }

    #[derive(Debug)]
    pub struct PopcornVendorBuff {
        instance_id: BuffInstanceId,
        source_id: BuffSourceId,
    }

    impl PopcornVendorBuff {
        pub fn new(source_id: PassiveEffectInstanceId) -> Self {
            Self {
                instance_id: BuffInstanceId::new(),
                source_id: BuffSourceId::Passive(source_id),
            }
        }
    }

    impl Buff for PopcornVendorBuff {
        fn attack_amount(&self) -> i32 {
            2
        }

        fn health_amount(&self) -> i32 {
            2
        }

        fn instance_id(&self) -> BuffInstanceId {
            self.instance_id
        }

        fn source_id(&self) -> BuffSourceId {
            self.source_id
        }

        fn definition_id(&self) -> Id {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PopcornVendor;
    use crate::game_agent::game_agent::*;
    use crate::game_logic::cards::Pawn;
    use crate::game_logic::*;
    use crate::{
        game_logic::{cards::UnitCardDefinition, SummonCreatureFromHandEvent},
        game_state::board::{BoardPos, RowId},
        game_state::make_test_state,
    };

    #[test]
    fn when_summoned_back_gives_buff() {
        let mut state = make_test_state();
        let mut dispatcher = make_default_dispatcher();
        let player_id = state.player_a_id();

        // Pawn will be summoned here.
        let pawn_pos = BoardPos::new(player_id, RowId::FrontRow, 0);

        // When prompted, player will pick the pawn
        // to receive the buff
        {
            let mut prompter_a = Box::new(MockPrompter::new());
            prompter_a
                .expect_prompt_player_creature_pos()
                .returning(move |_| pawn_pos);
            dispatcher.set_player_a_prompter(prompter_a);
        }

        // Summon a pawn to receive the buff
        let hand = state.hand_mut(player_id);
        let pawn = Pawn.make_instance();
        let pawn_id = pawn.id();
        {
            hand.add_card(pawn);
            let summon_event = SummonCreatureFromHandEvent::new(player_id, pawn_pos, pawn_id);
            dispatcher.dispatch(summon_event, &mut state);
        }

        // Summon the popcorn vendor and target the Pawn with the buff
        let hand = state.hand_mut(player_id);
        let pop_vend_pos = BoardPos::new(player_id, RowId::BackRow, 0);
        let pop_vend = PopcornVendor.make_instance();
        let pop_vend_id = pop_vend.id();
        {
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
    fn when_summoned_back_empty_board_expects_no_prompt() {
        let mut state = make_test_state();
        let mut dispatcher = make_default_dispatcher();
        let player_id = state.player_a_id();

        {
            let mut prompter_a = Box::new(MockPrompter::new());
            prompter_a.expect_prompt_player_creature_pos().never();
            dispatcher.set_player_a_prompter(prompter_a);
        }

        // Summon the popcorn vendor alone in the back row
        let hand = state.hand_mut(player_id);
        let pop_vend_pos = BoardPos::new(player_id, RowId::BackRow, 0);
        let pop_vend = PopcornVendor.make_instance();
        let pop_vend_id = pop_vend.id();
        {
            hand.add_card(pop_vend);
            let summon_event =
                SummonCreatureFromHandEvent::new(player_id, pop_vend_pos, pop_vend_id);
            dispatcher.dispatch(summon_event, &mut state);
        }

        // assertion: we would have panicked if the prompter was called,
        // since the mock is configured with `never()`
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
