use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Id(Uuid);
