use std::borrow::Borrow;

use crate::{game_logic::cards::UnitCardDefinition, id::HasId};
use crate::{
    game_logic::Buff,
    id::{new_id, Id},
};

#[derive(Debug)]
pub struct UnitCardBoardInstance {
    definition: Box<dyn UnitCardDefinition>,
    buffs: Vec<Box<dyn Buff>>,
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
            buffs: Vec::new(),
            id: new_id(),
        }
    }

    pub fn attack(&self) -> i32 {
        let attack_buf: i32 = self.buffs().iter().map(|b| b.attack_amount()).sum();

        self.attack + attack_buf
    }

    pub fn health(&self) -> i32 {
        let health_buf: i32 = self.buffs().iter().map(|b| b.health_amount()).sum();
        self.health + health_buf
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn buffs(&self) -> &[Box<dyn Buff>] {
        self.buffs.as_slice()
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
