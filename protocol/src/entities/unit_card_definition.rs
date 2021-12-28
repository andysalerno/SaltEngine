use serde::{Deserialize, Serialize};

use super::{
    board::Position, buff::BuffPlayerView, passive_effect::PassiveEffectInstanceId,
    unit_card_instance_view::InstanceState, Id, UnitCardInstanceId,
};

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

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct UnitCardDefinitionPlayerView {
    title: String,
    cost: i32,
    text: String,
    flavor_text: String,
    attack: i32,
    health: i32,
    row_width: usize,
    placeable_at: Position,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct PassiveEffectInstancePlayerView {
    /// The definition of the passive effect.
    definition: PassiveEffectDefinitionPlayerView,

    /// The unique ID of this instance of the passive effect.
    instance_id: PassiveEffectInstanceId,

    /// The ID of the card instance that originated this passive effect.
    originator_id: UnitCardInstanceId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PassiveEffectDefinitionPlayerView {
    definition_id: Id,
}
