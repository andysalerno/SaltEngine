use serde::{Deserialize, Serialize};

use crate::entities::{Board, UnitCardInstance};

#[derive(Serialize)]
pub struct FullState {
    entities: Vec<Box<dyn Entity>>, // not really
}

pub trait Entity {}

impl Serialize for dyn Entity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bool(false)
    }
}
