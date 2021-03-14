use serde::{Serialize, Serializer};
use uuid::Uuid;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Id(Uuid);

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", self.0))
    }
}

impl Id {
    pub fn new() -> Self {
        Id(Uuid::new_v4())
    }
}
