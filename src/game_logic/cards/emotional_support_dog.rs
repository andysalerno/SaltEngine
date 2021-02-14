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
        "Frontbuff: +1/+1"
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

    fn placeable_at(&self) -> Position {
        Position::Back
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
    ) -> Box<dyn FnOnce(PassiveEffectInstanceId, UnitCardInstanceId, &mut GameState)> {
        Box::new(move |instance_id, originator_id, game_state| {
            let doggy_pos = game_state.get_pos_by_id(originator_id);

            if doggy_pos.row_id != RowId::BackRow {
                return;
            }

            let mut front_pos = doggy_pos.clone();
            front_pos.row_id = RowId::FrontRow;

            let front_card = game_state.get_at(front_pos);

            if let Some(front_card) = front_card {
                let id = front_card.id();
                game_state.update_by_id(id, |c| {
                    c.add_buff(Box::new(EmotionalSupportDogBuff::new(instance_id)));
                });
            }
        })
    }
}
