use std::borrow::Borrow;

use crate::id::{new_id, Id};
use crate::{game_logic::cards::UnitCardDefinition, id::HasId};

#[derive(Debug)]
pub struct UnitCardBoardInstance {
    definition: Box<dyn UnitCardDefinition>,
    id: Id,
    attack: i32,
    health: i32,
    width: usize,
}

impl UnitCardBoardInstance {
    pub fn new(definition: Box<dyn UnitCardDefinition>) -> Self {
        Self {
            attack: definition.attack(),
            health: definition.health(),
            width: definition.row_width(),
            definition,
            id: new_id(),
        }
    }

    pub fn attack(&self) -> i32 {
        self.attack
    }

    pub fn health(&self) -> i32 {
        self.health
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn definition(&self) -> &dyn UnitCardDefinition {
        self.definition.borrow()
    }

    pub fn take_damage(&mut self, damage_amount: usize) {
        self.health -= damage_amount as i32;
    }
}

impl HasId for UnitCardBoardInstance {
    fn id(&self) -> Id {
        self.id
    }
}
