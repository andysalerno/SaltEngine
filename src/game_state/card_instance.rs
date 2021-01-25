use crate::id::{new_id, Id};
use crate::{game_logic::cards::UnitCardDefinition, id::HasId};

pub struct UnitCardBoardInstance {
    definition: Box<dyn UnitCardDefinition>,
    id: Id,
    attack: i32,
    health: i32,
}

impl UnitCardBoardInstance {
    pub fn new(definition: Box<dyn UnitCardDefinition>) -> Self {
        Self {
            attack: definition.attack(),
            health: definition.health(),
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
}

impl HasId for UnitCardBoardInstance {
    fn id(&self) -> Id {
        self.id
    }
}
