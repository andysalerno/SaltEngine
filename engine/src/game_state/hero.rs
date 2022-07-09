use entity_arena::{id::EntityTypeId, IsEntity};
use isentity_macro_derive::entity;
use protocol::entities::PlayerId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[entity("dab552df-8955-4b6a-88fc-a9af53e0ee43")]
pub struct HeroInstance {
    player_id: PlayerId,
    health: i32,
    mana: i32,
}

impl HeroInstance {
    pub fn new(player_id: PlayerId) -> Self {
        Self {
            player_id,
            health: 100,
            mana: 0,
        }
    }

    pub fn health(&self) -> i32 {
        self.health
    }

    pub fn mana(&self) -> i32 {
        self.mana
    }

    pub fn set_mana(&mut self, val: i32) {
        self.mana = val;
    }

    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }
}
