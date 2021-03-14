use uuid::Uuid;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Id(Uuid);

impl Id {
    pub fn new() -> Self {
        Id(Uuid::new_v4())
    }
}
