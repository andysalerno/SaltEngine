use crate::{
    game_logic::{
        buff::{Buff, BuffSourceId},
        BuffInstanceId,
    },
    game_state::{board::RowId, AsSelector, UnitCardInstanceId},
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
