use salt_engine::id::Id;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NewGameResponse {
    pub game_id: Id,
    pub players_in_queue: usize,
}

impl NewGameResponse {
    pub(crate) fn new(game_id: Id) -> Self {
        Self {
            game_id,
            players_in_queue: 0,
        }
    }
}
