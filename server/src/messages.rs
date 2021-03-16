use salt_engine::id::Id;
use serde::{Deserialize, Serialize};

pub mod from_client {
    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct JoinGame;
}

pub mod from_server {
    use super::*;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct GameId(Id);
}
