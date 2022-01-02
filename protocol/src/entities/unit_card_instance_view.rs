use super::{
    board::BoardPos, buff::BuffPlayerView, unit_card_definition::UnitCardDefinitionPlayerView,
    PassiveEffectInstancePlayerView, UnitCardInstanceId,
};
use serde::{Deserialize, Serialize};

/// A view of a creature card instance.
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct UnitCardInstancePlayerView {
    definition: UnitCardDefinitionPlayerView,
    buffs: Vec<BuffPlayerView>,
    passive_effect: Option<PassiveEffectInstancePlayerView>,
    id: UnitCardInstanceId,
    attack: i32,
    health: i32,
    width: usize,
    state: Option<InstanceState>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum InstanceState {
    Pos(BoardPos),
    CreatureInstanceId(UnitCardInstanceId),
}
