use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Hideable<T> {
    Hidden,
    Revealed(T),
}
