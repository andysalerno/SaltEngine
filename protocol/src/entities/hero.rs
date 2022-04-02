use super::{AsId, EntityId, EntityTypeId, HasId, Id, IsEntity, PlayerId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerHero {
    id: HeroId,
    player_id: PlayerId,
    mana_limit: u32,
    mana_available: u32,
}

impl PlayerHero {
    pub fn new(player_id: PlayerId) -> Self {
        Self {
            player_id,
            id: HeroId::new(),
            mana_limit: 0,
            mana_available: 0,
        }
    }

    pub fn player_id(&self) -> PlayerId {
        self.player_id
    }

    pub fn mana_limit(&self) -> u32 {
        self.mana_limit
    }

    pub fn mana_available(&self) -> u32 {
        self.mana_available
    }
}

impl HasId for PlayerHero {
    type IdType = HeroId;

    fn id(&self) -> Self::IdType {
        self.id
    }
}

impl IsEntity for PlayerHero {
    type IdType = HeroId;

    fn type_id() -> EntityTypeId {
        EntityTypeId::parse_str("b1038bfa-f055-40d6-a7ef-8c0415d225aa")
    }
}

#[derive(Debug, Eq, Hash, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HeroId(Id);

impl EntityId for HeroId {
    type EntityType = PlayerHero;
}

impl HeroId {
    #[must_use]
    pub fn new() -> Self {
        Self(Id::new())
    }
}

impl Default for HeroId {
    fn default() -> Self {
        Self::new()
    }
}

impl AsId for HeroId {
    fn as_id(&self) -> Id {
        self.0
    }
}
