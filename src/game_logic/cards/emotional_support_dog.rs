use crate::{game_logic::buff::Buff, id::Id};
use crate::{game_logic::PassiveEffect, id::HasId};

use super::{CardDefinition, UnitCardDefinition};

#[derive(Debug, Clone)]
pub struct EmotionalSupportDog;

impl HasId for EmotionalSupportDog {
    fn id(&self) -> Id {
        // id::parse("...")
        todo!()
    }
}

impl CardDefinition for EmotionalSupportDog {
    fn title(&self) -> &str {
        "Emotional Support Dog"
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
        3
    }

    fn row_width(&self) -> usize {
        1
    }
}

#[derive(Debug)]
struct EmotionalSupportDogBuff {
    instance_id: Id,
}

impl Buff for EmotionalSupportDogBuff {
    fn attack_amount(&self) -> i32 {
        1
    }

    fn health_amount(&self) -> i32 {
        1
    }

    fn instance_id(&self) -> Id {
        self.instance_id
    }

    fn definition_id(&self) -> Id {
        todo!()
    }
}

#[derive(Debug)]
struct EmotionalSupportDogPassive {
    originator: Id,
    definition_id: Id,
    instance_id: Id,
}

impl PassiveEffect for EmotionalSupportDogPassive {
    fn originator(&self) -> Id {
        self.originator
    }

    fn definition_id(&self) -> Id {
        self.definition_id
    }

    fn instance_id(&self) -> Id {
        self.instance_id
    }

    fn update(&self) -> Box<dyn FnOnce(&mut crate::game_state::GameState)> {
        let id = self.instance_id;

        Box::new(move |game_state| {
            //let id = self.instance_id;
            // Find the buff already applied by this instance
            let existing = game_state
                .board_iter()
                .filter(|i| i.buffs().iter().any(|b| b.instance_id() == id))
                .next();
            // add it if none
        })
    }
}
