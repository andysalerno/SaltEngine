use super::{
    board::BoardPos,
    buff::BuffPlayerView,
    unit_card_definition::{PassiveEffectInstancePlayerView, UnitCardDefinitionPlayerView},
    UnitCardInstanceId,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum InstanceState {
    Pos(BoardPos),
    CreatureInstanceId(UnitCardInstanceId),
}

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
