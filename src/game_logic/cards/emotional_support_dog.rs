use crate::{
    game_logic::{
        buff::{Buff, BuffSourceId},
        BuffInstanceId,
    },
    game_state::{board::RowId, UnitCardBoardInstanceId},
    id::Id,
};
use crate::{
    game_logic::{passive_effect::PassiveEffectInstanceId, PassiveEffectDefinition},
    game_state::GameState,
};

use super::{CardDefinition, UnitCardDefinition};

#[derive(Debug, Clone)]
pub struct EmotionalSupportDog;

impl EmotionalSupportDog {
    fn id(&self) -> Id {
        // id::parse("...")
        todo!()
    }
}

impl CardDefinition for EmotionalSupportDog {
    fn title(&self) -> &str {
        "Emo Sup Dog"
        //"Emotional Support Dog"
    }

    fn cost(&self) -> i32 {
        2
    }

    fn flavor_text(&self) -> &str {
        "But really, aren't all dogs Emotional Support Dogs?"
    }

    fn text(&self) -> &str {
        "Passive: the unit in front gets +1/+1."
    }
}

impl UnitCardDefinition for EmotionalSupportDog {
    fn attack(&self) -> i32 {
        1
    }

    fn health(&self) -> i32 {
        1
    }

    fn row_width(&self) -> usize {
        1
    }

    fn passive_effect(&self) -> Option<Box<dyn PassiveEffectDefinition>> {
        Some(Box::new(EmotionalSupportDogPassiveDefinition::new()))
    }
}

#[derive(Debug)]
struct EmotionalSupportDogBuff {
    instance_id: BuffInstanceId,
    source_id: BuffSourceId,
}

impl EmotionalSupportDogBuff {
    pub fn new(source_id: PassiveEffectInstanceId) -> Self {
        Self {
            instance_id: BuffInstanceId::new(),
            source_id: BuffSourceId::Passive(source_id),
        }
    }
}

impl Buff for EmotionalSupportDogBuff {
    fn attack_amount(&self) -> i32 {
        1
    }

    fn health_amount(&self) -> i32 {
        1
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

#[derive(Debug)]
struct EmotionalSupportDogPassiveDefinition {
    definition_id: Id,
}

impl EmotionalSupportDogPassiveDefinition {
    pub fn new() -> Self {
        Self {
            // TODO: replace with constant
            definition_id: Id::new(),
        }
    }
}

impl PassiveEffectDefinition for EmotionalSupportDogPassiveDefinition {
    fn definition_id(&self) -> Id {
        todo!()
    }

    fn update(
        &self,
    ) -> Box<dyn FnOnce(PassiveEffectInstanceId, UnitCardBoardInstanceId, &mut GameState)> {
        // let id = self.instance_id;
        // let originator_id = self.originator;

        Box::new(move |instance_id, originator_id, game_state| {
            // Find the buff already applied by this instance
            let existing = game_state
                .board_iter()
                .filter(|i| {
                    i.buffs().iter().any(|buff| match buff.source_id() {
                        BuffSourceId::Passive(_) => true,
                        _ => false,
                    })
                    //.any(|b| b.instance_id() == instance.instance_id())
                })
                .next();

            if let Some(_) = existing {
                // Nothing to do since it already exists
                return;
            }

            let doggy = game_state
                .board_iter()
                .filter(|i| i.id() == originator_id)
                .next()
                .expect("A passive is active for ESD, but ESD doesn't exist?");

            let doggy_pos = game_state.get_pos_by_id(doggy.id());

            if doggy_pos.row_id != RowId::BackRow {
                return;
            }

            let mut front_pos = doggy_pos.clone();
            front_pos.row_id = RowId::FrontRow;

            let front_card = game_state.get_at(front_pos);

            if let Some(front_card) = front_card {
                game_state.update_by_id(front_card.id(), |c| {
                    c.add_buff(Box::new(EmotionalSupportDogBuff::new(instance_id)));
                });
            }
        })
    }
}
