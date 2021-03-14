use serde::{de::Visitor, Deserialize, Serialize, Serializer};
use uuid::Uuid;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Id(Uuid);

impl Id {
    pub fn new() -> Self {
        Id(Uuid::new_v4())
    }
}
