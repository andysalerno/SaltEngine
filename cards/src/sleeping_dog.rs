use async_trait::async_trait;
use protocol::entities::{Id, Position};
use salt_engine::{
    cards::{actions::UponReceiveDamageAction, CardDefinition, UnitCardDefinition},
    game_logic::{Buff, EventDispatcher},
    game_state::{board::BoardView, GameState},
};

use self::actions::ReceiveDamageAction;

#[derive(Debug, Clone)]
pub struct SleepingDog;

impl SleepingDog {
    fn id(&self) -> Id {
        // id::parse("...")
        todo!()
    }
}

impl CardDefinition for SleepingDog {
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
        "Back
When damaged, if
in back row,
move to front row,
and +7 attack."
    }
}

impl UnitCardDefinition for SleepingDog {
    fn attack(&self) -> i32 {
        0
    }

    fn health(&self) -> i32 {
        3
    }

    fn row_width(&self) -> usize {
        2
    }

    fn placeable_at(&self) -> Position {
        Position::Back
    }

    fn upon_receive_damage(&self) -> Box<dyn UponReceiveDamageAction> {
        Box::new(ReceiveDamageAction)
    }
}

mod actions {
    use super::{async_trait, buffs, BoardView, EventDispatcher, GameState};
    use protocol::entities::{CreatureInstanceId, RowId};
    use salt_engine::cards::actions::UponReceiveDamageAction;

    pub(super) struct ReceiveDamageAction;

    #[async_trait]
    impl UponReceiveDamageAction for ReceiveDamageAction {
        async fn action(
            &self,
            instance_id: CreatureInstanceId,
            state: &mut GameState,
            _dispatcher: &mut EventDispatcher,
        ) {
            // can move to front row?
            let slots = state.board().slots_with_creature(instance_id);
            let starting_pos = slots[0].pos();

            for slot in slots {
                if slot.pos().row() == RowId::FrontRow {
                    // Can't move to front row if already in front row.
                    return;
                }

                let mut in_front = slot.pos();
                in_front.row_id = RowId::FrontRow;

                if state.board().creature_at_pos(in_front).is_some() {
                    // Can't moveto front row if something is already there.
                    return;
                }
            }

            // Take the card off the back row
            let instance = state.board_mut().take_creature_by_id(instance_id);

            // Put it on the front row
            let mut front_slot = starting_pos;
            front_slot.row_id = RowId::FrontRow;
            state.board_mut().set_creature_at_pos(front_slot, instance);

            let buff = Box::new(buffs::SleepingDogBuff::new(instance_id));
            let instance_mut = state.board_mut().creature_instance_mut(instance_id);
            instance_mut.add_buff(buff);
        }
    }
}

mod buffs {
    use protocol::entities::{BuffInstanceId, BuffSourceId, CreatureInstanceId};

    use super::{Buff, Id};

    #[derive(Debug)]
    pub(super) struct SleepingDogBuff {
        instance_id: BuffInstanceId,
        source_id: BuffSourceId,
    }

    impl SleepingDogBuff {
        pub fn new(source_id: CreatureInstanceId) -> Self {
            Self {
                instance_id: BuffInstanceId::new(),
                source_id: BuffSourceId::CreatureInstance(source_id),
            }
        }
    }

    impl Buff for SleepingDogBuff {
        fn attack_amount(&self) -> i32 {
            7
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
