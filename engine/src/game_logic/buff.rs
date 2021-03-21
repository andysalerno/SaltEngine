use super::passive_effect::PassiveEffectInstanceId;
use crate::{game_state::UnitCardInstanceId, id::Id};
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuffInstanceId(Id);

impl BuffInstanceId {
    pub fn new() -> Self {
        Self(Id::new())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum BuffSourceId {
    Passive(PassiveEffectInstanceId),
    CreatureInstance(UnitCardInstanceId),
    Other(Id),
}

impl From<PassiveEffectInstanceId> for BuffSourceId {
    fn from(id: PassiveEffectInstanceId) -> Self {
        BuffSourceId::Passive(id)
    }
}

impl From<UnitCardInstanceId> for BuffSourceId {
    fn from(id: UnitCardInstanceId) -> Self {
        BuffSourceId::CreatureInstance(id)
    }
}

pub trait BuffView {
    fn attack_amount(&self) -> i32;
    fn health_amount(&self) -> i32;
    fn source_id(&self) -> BuffSourceId;
    fn instance_id(&self) -> BuffInstanceId;
    fn definition_id(&self) -> Id;
    fn is_from_passive(&self) -> bool;
}

impl BuffView for dyn Buff {
    fn attack_amount(&self) -> i32 {
        self.attack_amount()
    }

    fn health_amount(&self) -> i32 {
        self.health_amount()
    }

    fn source_id(&self) -> BuffSourceId {
        self.source_id()
    }

    fn instance_id(&self) -> BuffInstanceId {
        self.instance_id()
    }

    fn definition_id(&self) -> Id {
        self.definition_id()
    }

    fn is_from_passive(&self) -> bool {
        self.is_from_passive()
    }
}

pub trait Buff: Sync + Send + std::fmt::Debug {
    fn attack_amount(&self) -> i32;
    fn health_amount(&self) -> i32;
    fn source_id(&self) -> BuffSourceId;
    fn instance_id(&self) -> BuffInstanceId;
    fn definition_id(&self) -> Id;

    fn is_from_passive(&self) -> bool {
        match self.source_id() {
            BuffSourceId::Passive(_) => true,
            _ => false,
        }
    }
}

pub struct BuffBuilder {
    attack_amount: i32,
    health_amount: i32,
    source: BuffSourceId,
    instance_id: BuffInstanceId,
    definition_id: Id,
}

impl BuffBuilder {
    pub fn new(source: impl Into<BuffSourceId>, definition_id: Id) -> Self {
        Self {
            attack_amount: 0,
            health_amount: 0,
            instance_id: BuffInstanceId::new(),
            source: source.into(),
            definition_id,
        }
    }

    pub fn attack(mut self, attack_buff_amount: i32) -> Self {
        self.attack_amount = attack_buff_amount;
        self
    }

    pub fn health(mut self, health_buff_amount: i32) -> Self {
        self.health_amount = health_buff_amount;
        self
    }

    pub fn build(self) -> BuiltBuff {
        BuiltBuff {
            attack_amount: self.attack_amount,
            health_amount: self.health_amount,
            source_id: self.source,
            instance_id: self.instance_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BuiltBuff {
    attack_amount: i32,
    health_amount: i32,
    source_id: BuffSourceId,
    instance_id: BuffInstanceId,
}

impl Buff for BuiltBuff {
    fn attack_amount(&self) -> i32 {
        self.attack_amount
    }

    fn health_amount(&self) -> i32 {
        self.health_amount
    }

    fn source_id(&self) -> BuffSourceId {
        self.source_id
    }

    fn instance_id(&self) -> BuffInstanceId {
        self.instance_id
    }

    fn definition_id(&self) -> Id {
        todo!()
    }
}

pub mod player_view {
    use super::*;
    use crate::game_state::MakePlayerView;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct BuffPlayerView {
        attack_amount: i32,
        health_amount: i32,
        source_id: BuffSourceId,
        instance_id: BuffInstanceId,
        definition_id: Id,
        is_from_passive: bool,
    }

    impl MakePlayerView for Box<dyn Buff> {
        type TOut = BuffPlayerView;

        fn player_view(&self, _player_viewing: crate::game_state::PlayerId) -> BuffPlayerView {
            BuffPlayerView {
                attack_amount: self.attack_amount(),
                health_amount: self.health_amount(),
                source_id: self.source_id(),
                instance_id: self.instance_id(),
                definition_id: self.definition_id(),
                is_from_passive: self.is_from_passive(),
            }
        }
    }
}
