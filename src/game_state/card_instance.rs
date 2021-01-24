use crate::id::HasId;
use crate::id::{new_id, Id};

pub struct CardInstance<T> {
    definition: T,
    id: Id,
}

impl<T> CardInstance<T> {
    pub fn new(definition: T) -> Self {
        Self {
            definition,
            id: new_id(),
        }
    }
}

impl<T> HasId for CardInstance<T> {
    fn id(&self) -> Id {
        self.id
    }
}
