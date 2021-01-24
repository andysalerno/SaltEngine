use uuid::Uuid;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Id(Uuid);

pub fn new_id() -> Id {
    Id(Uuid::new_v4())
}

pub trait HasId {
    fn id(&self) -> Id;
}
