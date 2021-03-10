use crate::{
    game_logic::{
        buff::{Buff, BuffSourceId},
        BuffInstanceId,
    },
    game_state::{board::RowId, UnitCardInstanceId},
    id::Id,
};
use crate::{
    game_logic::{CreatureHealedEvent, EventDispatcher},
    game_state::{board::BoardPos, GameState, InstanceState, UnitCardInstance},
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
Back: Give another
friendly creature +2
attack, and heal it
for 3 each time your
turn ends."
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

    fn upon_summon(
        &self,
    ) -> Box<dyn FnOnce(&mut UnitCardInstance, BoardPos, &mut GameState, &mut EventDispatcher)>
    {
        Box::new(|instance, pos, game_state, dispatcher| {
            if pos.row() == RowId::FrontRow {
                // Front: buffs self
                instance.add_buff(Box::new(buff_self::PopcornVendorBuff::new(instance.id())));
            } else {
                // Back: buffs another
                if !game_state.player_has_any_creature(pos.player_id) {
                    return;
                }

                let other_pos = dispatcher
                    .player_prompter()
                    .prompt_player_creature_pos(game_state);
                let slot = game_state.board_mut().slot_at_pos_mut(other_pos);

                let creature = slot.maybe_creature_mut().expect(
                    "Slot must have a creature, since player was prompted for a creature slot.",
                );
                creature.add_buff(Box::new(buff_other::PopcornVendorBuff::new(instance.id())));

                instance.set_state(Some(InstanceState::CreatureInstanceId(creature.id())));
            }
        })
    }

    fn upon_turn_end(
        &self,
    ) -> Box<dyn FnOnce(UnitCardInstanceId, &mut GameState, &mut EventDispatcher)> {
        Box::new(|id, game_state, dispatcher| {
            let instance = game_state.board().creature_instance(id);

            // Front row has no turn-end action.
            if game_state.board().position_with_creature(id).row() == RowId::FrontRow {
                return;
            }

            let state = instance.state();

            if let Some(InstanceState::CreatureInstanceId(target_id)) = state {
                // Heal the target for 3
                let heal_amount = 3;
                let heal_event = CreatureHealedEvent::new(*target_id, heal_amount);

                {
                    let target_creature = game_state.board().creature_instance(*target_id);
                    println!(
                        "Popcorn Vendor heals {} for 3",
                        target_creature.definition().title()
                    );
                }

                dispatcher.dispatch(heal_event, game_state);
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
