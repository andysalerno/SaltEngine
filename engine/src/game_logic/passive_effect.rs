use super::Buff;
use crate::game_state::{board::BoardView, GameState};
use protocol::entities::{CreatureInstanceId, Id, PassiveEffectInstanceId, RowId};
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;

/// A definition of a passive effect, including
/// an ID and the update logic that will be re-executed
// whenever the gamestate changes.
pub trait PassiveEffectDefinition: Send + Sync {
    fn definition_id(&self) -> Id;
    fn update(
        &self,
    ) -> Box<dyn FnOnce(PassiveEffectInstanceId, CreatureInstanceId, &mut GameState)>;
}

impl std::fmt::Debug for dyn PassiveEffectDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{PassiveEffectDefinition}}{:?}", self.definition_id())
    }
}

// /// An ID representing a unique instance of a passive effect.
// #[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
// pub struct PassiveEffectInstanceId(Id);

// impl PassiveEffectInstanceId {
//     #[must_use]
//     pub fn new() -> Self {
//         Self(Id::new())
//     }
// }

// impl Default for PassiveEffectInstanceId {
//     fn default() -> Self {
//         Self::new()
//     }
// }

pub trait PassiveEffectView {
    fn definition_id(&self) -> Id;
    fn instance_id(&self) -> PassiveEffectInstanceId;
    fn originator_id(&self) -> CreatureInstanceId;
}

impl PassiveEffectView for PassiveEffectInstance {
    fn definition_id(&self) -> Id {
        self.definition.definition_id()
    }

    fn instance_id(&self) -> PassiveEffectInstanceId {
        self.instance_id
    }

    fn originator_id(&self) -> CreatureInstanceId {
        self.originator_id
    }
}

/// An instance of a passive effect in the game.
/// Passive effects are continuously re-evaluated
/// whenever the game state changes.
#[derive(Debug)]
pub struct PassiveEffectInstance {
    /// The definition of the passive effect.
    definition: Box<dyn PassiveEffectDefinition>,

    /// The unique ID of this instance of the passive effect.
    instance_id: PassiveEffectInstanceId,

    /// The ID of the card instance that originated this passive effect.
    originator_id: CreatureInstanceId,
}

impl PassiveEffectInstance {
    #[must_use]
    pub fn new(
        definition: Box<dyn PassiveEffectDefinition>,
        originator_id: CreatureInstanceId,
    ) -> Self {
        Self {
            definition,
            instance_id: PassiveEffectInstanceId::new(),
            originator_id,
        }
    }

    /// The unique ID of this instance of the passive effect.
    #[must_use]
    pub fn instance_id(&self) -> PassiveEffectInstanceId {
        self.instance_id
    }

    /// The ID of the card instance that originated this passive effect.
    #[must_use]
    pub fn originator_id(&self) -> CreatureInstanceId {
        self.originator_id
    }

    /// The definition of the passive effect.
    #[must_use]
    pub fn definition(&self) -> &dyn PassiveEffectDefinition {
        self.definition.borrow()
    }
}

/// An implementation of `PassiveEffectDefinition`
/// that buffs the companion of the card with the passive effect.
#[derive(Debug)]
pub struct PassiveCompanionBuff<T>
where
    T: Buff,
{
    definition_id: Id,
    buff: Box<T>,
    for_row: Option<RowId>,
}

impl<T> PassiveCompanionBuff<T>
where
    T: Buff + Clone,
{
    #[must_use]
    pub fn new(definition_id: Id, buff: Box<T>) -> Self {
        Self {
            definition_id,
            buff,
            for_row: None,
        }
    }

    #[must_use]
    pub fn new_for_row(definition_id: Id, buff: Box<T>, row: RowId) -> Self {
        Self {
            definition_id,
            buff,
            for_row: Some(row),
        }
    }
}

impl<T> PassiveEffectDefinition for PassiveCompanionBuff<T>
where
    T: Buff + Sync + Clone + Send + 'static,
{
    fn definition_id(&self) -> Id {
        self.definition_id
    }

    fn update(
        &self,
    ) -> Box<dyn FnOnce(PassiveEffectInstanceId, CreatureInstanceId, &mut GameState)> {
        let buff = self.buff.clone();
        let for_row = self.for_row;

        Box::new(move |_, originator_id, game_state| {
            let originator_pos = game_state.board().pos_with_creature(originator_id);

            if let Some(row) = for_row {
                if row != originator_pos.row() {
                    return;
                }
            }

            if let Some(companion) = game_state.board().companion_creature(originator_pos) {
                let id = companion.id();

                game_state.update_by_id(id, |c| {
                    c.add_buff(buff);
                });
            }
        })
    }
}

pub mod player_view {
    use protocol::entities::PlayerId;

    use super::{
        CreatureInstanceId, Deserialize, Id, PassiveEffectDefinition, PassiveEffectInstance,
        PassiveEffectInstanceId, PassiveEffectView, Serialize,
    };
    use crate::game_state::MakePlayerView;

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct PassiveEffectDefinitionPlayerView {
        definition_id: Id,
    }

    impl<'a> MakePlayerView<'a> for dyn PassiveEffectDefinition {
        type TOut = PassiveEffectDefinitionPlayerView;

        fn player_view(&'a self, _player_viewing: PlayerId) -> PassiveEffectDefinitionPlayerView {
            PassiveEffectDefinitionPlayerView {
                definition_id: self.definition_id(),
            }
        }
    }

    #[derive(Debug, Serialize, Clone, Deserialize)]
    pub struct PassiveEffectInstancePlayerView {
        /// The definition of the passive effect.
        definition: PassiveEffectDefinitionPlayerView,

        /// The unique ID of this instance of the passive effect.
        instance_id: PassiveEffectInstanceId,

        /// The ID of the card instance that originated this passive effect.
        originator_id: CreatureInstanceId,
    }

    impl<'a> MakePlayerView<'a> for PassiveEffectInstance {
        type TOut = PassiveEffectInstancePlayerView;

        fn player_view(&'a self, player_viewing: PlayerId) -> PassiveEffectInstancePlayerView {
            let definition = self.definition.player_view(player_viewing);

            PassiveEffectInstancePlayerView {
                definition,
                instance_id: self.instance_id(),
                originator_id: self.originator_id(),
            }
        }
    }

    impl PassiveEffectView for PassiveEffectInstancePlayerView {
        fn definition_id(&self) -> Id {
            self.definition.definition_id
        }

        fn instance_id(&self) -> PassiveEffectInstanceId {
            self.instance_id
        }

        fn originator_id(&self) -> CreatureInstanceId {
            self.originator_id
        }
    }
}
