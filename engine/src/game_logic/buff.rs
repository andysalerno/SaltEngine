use protocol::entities::{BuffInstanceId, BuffSourceId, Id};
use serde::{Deserialize, Serialize};

pub trait BuffView {
    fn attack_amount(&self) -> i32;
    fn health_amount(&self) -> i32;
    fn source_id(&self) -> BuffSourceId;
    fn instance_id(&self) -> BuffInstanceId;
    fn definition_id(&self) -> Id;
    fn is_from_passive(&self) -> bool;
}

impl<T: AsRef<dyn Buff>> BuffView for T {
    fn attack_amount(&self) -> i32 {
        Buff::attack_amount(self.as_ref())
    }

    fn health_amount(&self) -> i32 {
        Buff::health_amount(self.as_ref())
    }

    fn source_id(&self) -> BuffSourceId {
        Buff::source_id(self.as_ref())
    }

    fn instance_id(&self) -> BuffInstanceId {
        Buff::instance_id(self.as_ref())
    }

    fn definition_id(&self) -> Id {
        Buff::definition_id(self.as_ref())
    }

    fn is_from_passive(&self) -> bool {
        Buff::is_from_passive(self.as_ref())
    }
}

pub trait Buff: Sync + Send + std::fmt::Debug {
    fn attack_amount(&self) -> i32;
    fn health_amount(&self) -> i32;
    fn source_id(&self) -> BuffSourceId;
    fn instance_id(&self) -> BuffInstanceId;
    fn definition_id(&self) -> Id;

    fn is_from_passive(&self) -> bool {
        matches!(self.source_id(), BuffSourceId::Passive(_))
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

    #[must_use]
    pub fn attack(mut self, attack_buff_amount: i32) -> Self {
        self.attack_amount = attack_buff_amount;
        self
    }

    #[must_use]
    pub fn health(mut self, health_buff_amount: i32) -> Self {
        self.health_amount = health_buff_amount;
        self
    }

    #[must_use]
    pub fn build(self) -> BuiltBuff {
        BuiltBuff {
            attack_amount: self.attack_amount,
            health_amount: self.health_amount,
            source_id: self.source,
            instance_id: self.instance_id,
            definition_id: self.definition_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltBuff {
    attack_amount: i32,
    health_amount: i32,
    source_id: BuffSourceId,
    instance_id: BuffInstanceId,
    definition_id: Id,
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
        self.definition_id
    }
}

pub mod player_view {
    use protocol::entities::{BuffInstanceId, BuffPlayerView, BuffSourceId, PlayerId};

    use super::{Buff, BuffView, Deserialize, Id, Serialize};
    use crate::game_state::MakePlayerView;

    impl<'a> MakePlayerView<'a> for dyn Buff {
        type TOut = BuffPlayerView;

        fn player_view(&'a self, _player_viewing: PlayerId) -> BuffPlayerView {
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

    impl BuffView for BuffPlayerView {
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
            self.definition_id
        }

        fn is_from_passive(&self) -> bool {
            self.is_from_passive
        }
    }
}
