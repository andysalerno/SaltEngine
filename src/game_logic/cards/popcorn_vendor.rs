use crate::{
    game_logic::EventDispatcher,
    game_state::{board::BoardPos, GameState, UnitCardInstance},
};
use crate::{
    game_logic::{
        buff::{Buff, BuffSourceId},
        BuffInstanceId,
    },
    game_state::{board::RowId, UnitCardInstanceId},
    id::Id,
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
        "Sleeping Dog"
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
Back: Give a friendly
creature +3 attack
and fully heal it."
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
        Position::Back
    }

    fn upon_summon(
        &self,
    ) -> Box<dyn FnOnce(&mut UnitCardInstance, BoardPos, &mut GameState, &mut EventDispatcher)>
    {
        Box::new(|instance, pos, game_state, dispatcher| {
            if pos.row() == RowId::FrontRow {
                // Front: buffs self
                instance.add_buff(Box::new(buff_self::PopcornVendorBuff::new(instance.id())))
            } else {
                // Back: buffs another
                //let other_instance
                let other_pos = dispatcher.player_prompter().prompt_creature_pos(game_state);
                let slot = game_state.board_mut().slot_at_pos_mut(other_pos);

                let creature = slot.maybe_creature_mut().expect(
                    "Slot must have a creature, since player was prompted for a creature slot.",
                );
                creature.add_buff(Box::new(buff_other::PopcornVendorBuff::new(instance.id())))
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
