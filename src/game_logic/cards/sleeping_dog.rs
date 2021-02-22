use crate::{
    game_logic::{
        buff::{Buff, BuffSourceId},
        BuffInstanceId,
    },
    game_state::{board::RowId, UnitCardInstanceId},
    id::Id,
};
use crate::{
    game_logic::{passive_effect::PassiveEffectInstanceId, PassiveEffectDefinition},
    game_state::GameState,
};

use super::{CardDefinition, Position, UnitCardDefinition};

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

    fn upon_receive_damage(
        &self,
    ) -> Box<dyn FnOnce(UnitCardInstanceId, &mut GameState, &mut crate::game_logic::EventDispatcher)>
    {
        Box::new(|id, game_state, _dispatcher| {
            // can move to front row?
            let slots = game_state.board().slots_with_creature(id);
            let starting_pos = slots[0].pos();

            for slot in slots {
                if slot.pos().row() == RowId::FrontRow {
                    // Can't move to front row if already in front row.
                    return;
                }

                let mut in_front = slot.pos();
                in_front.row_id = RowId::FrontRow;

                if game_state.board().creature_at_pos(in_front).is_some() {
                    // Can't moveto front row if something is already there.
                    return;
                }
            }

            println!("sleeping dog is in slots: {:?}", slots);

            // Take the card off the back row
            let instance = game_state.board_mut().take_creature_by_id(id);

            // Put it on the front row
            let mut front_slot = starting_pos;
            front_slot.row_id = RowId::FrontRow;
            game_state
                .board_mut()
                .set_creature_at_pos(front_slot, instance);

            let buff = Box::new(SleepingDogBuff::new(id));
            let instance_mut = game_state.board_mut().creature_instance_mut(id);
            instance_mut.add_buff(buff);
        })
    }
}

#[derive(Debug)]
struct SleepingDogBuff {
    instance_id: BuffInstanceId,
    source_id: BuffSourceId,
}

impl SleepingDogBuff {
    pub fn new(source_id: UnitCardInstanceId) -> Self {
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
