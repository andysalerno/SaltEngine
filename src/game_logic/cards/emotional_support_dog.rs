use crate::{
    game_logic::buff::Buff,
    game_state::board::RowId,
    id::{new_id, Id},
};
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

impl EmotionalSupportDogBuff {
    pub fn new() -> Self {
        Self {
            instance_id: new_id(),
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
        let originator_id = self.originator;

        Box::new(move |game_state| {
            // Find the buff already applied by this instance
            let existing = game_state
                .board_iter()
                .filter(|i| i.buffs().iter().any(|b| b.instance_id() == id))
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
                    c.add_buf(Box::new(EmotionalSupportDogBuff::new()));
                });
            }
        })
    }
}
